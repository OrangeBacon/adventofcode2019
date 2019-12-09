use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::collections::VecDeque;
use std::collections::BTreeSet;
use indexmap::IndexMap;
use indexmap::IndexSet;
use super::input;
use super::instruction::*;

#[derive(Copy, Clone, Debug, PartialEq)]
enum ItemType {
    OpCode(OpCode),
    Literal(i64),
    Reference(i64),
    Relative(i64),
    Address(i64),
    Label(usize),
    Variable(usize, usize),
    LabelAddress(usize),
    Data(i64),
    Assign(usize, i64),
    None,
}

pub fn dasm(in_path: &str, out_path: &str) {
    let code = input(in_path);

    let mut output: Vec<ItemType> = vec![ItemType::None; code.len()];

    let mut locations: VecDeque<usize> = VecDeque::new();
    let mut i = 0;
    loop {
        let mut opcode = match code.get(i) {
            Some(a) => *a,
            None => {
                let loc = match locations.pop_front() {
                    Some(a) => a,
                    None => break,
                };
                i = loc;
                match code.get(i) {
                    Some(a) => *a,
                    None => break,
                }
            }
        };

        if output[i] != ItemType::None {
            i = code.len();
            continue;
        }

        let mut modes = vec![ParameterMode::Position; 3];
        if opcode >= 100 {
            let mut mode_num = (opcode - opcode % 100) / 100;
            let mut i = 0;
            while mode_num > 0 {
                modes[i] = ParameterMode::from_i64(mode_num % 10);
                mode_num -= mode_num % 10;
                mode_num /= 10;
                i += 1;
            }
            opcode %= 100;
        }

        let inst = OpCode::from_i64(opcode);
        output[i] = ItemType::OpCode(inst);
        
        for (j, expected_mode) in inst.to_params().iter().enumerate() {
            i += 1;
            let param = match code.get(i) {
                Some(a) => *a,
                None => break,
            };
            let param_mode = match *expected_mode {
                ParameterMode::Any => modes[j],
                a => a,
            };

            output[i] = match param_mode {
                ParameterMode::Position => ItemType::Reference(param),
                ParameterMode::Literal => ItemType::Literal(param),
                ParameterMode::Address => ItemType::Address(param),
                ParameterMode::Relative => ItemType::Relative(param),
                 _ => unreachable!(),
            };

            match output[i] {
                ItemType::Reference(a) => output[a as usize] = ItemType::Data(code[a as usize]),
                ItemType::Address(a) => locations.push_back(a as usize),
                _ => (),
            }
        }

        if opcode == 99 {
            i = code.len();
        } else {
            i += 1;
        }
    }

    let mut addresses = BTreeSet::new();
    addresses.insert(0);
    let mut variables = IndexSet::new();
    for item in &output {
        match item {
            ItemType::Address(a) => addresses.insert(*a),
            ItemType::Reference(a) => variables.insert(*a),
            _ => true,
        };
    }

    for item in output.iter_mut() {
        let new_item = match &item {
            ItemType::Reference(a) => ItemType::Variable(variables.get_full(a).unwrap().0, *a as usize),
            a => **a,
        };
        *item = new_item;
    }

    let mut variable_first_loc = IndexMap::new();
    for (i, value) in output.iter().enumerate() {
        match value {
            ItemType::Variable(a, _) => {
                if !variable_first_loc.contains_key(a) {
                    variable_first_loc.insert(*a, i);
                }
            },
            _ => (),
        }
    }

    for (i, address) in addresses.iter().enumerate().rev() {
        output.insert(*address as usize, ItemType::Label(i));

        let addr = *address;
        for item in output.iter_mut() {
            let new_item = match &item {
                ItemType::Address(a) if *a == addr => ItemType::LabelAddress(i),
                a => **a,
            };
            *item = new_item;
        }
    }

    let code_get_ignore = |list: &Vec<ItemType>, loc: usize| {
        let mut current = loc;
        for (i, value) in list.iter().enumerate() {
            if current == 0 {
                return i;
            }
            match value {
                ItemType::Label(_) => (),
                ItemType::Assign(_, _) => (),
                _ => {current -= 1;}
            }
        }
        panic!("Could not find data value");
    };

    for (_, location) in &variable_first_loc {
        let mut i = code_get_ignore(&output, *location);
        loop {
            match output[i] {
                ItemType::Label(_) => break,
                ItemType::Assign(_, _) => break,
                _ => (),
            }
            i -= 1;
        }
        match output[code_get_ignore(&output, *location)] {
            ItemType::Variable(name, data_loc) => {
                let new_data_loc = code_get_ignore(&output, data_loc);
                let data = match output[new_data_loc] {
                    ItemType::Data(a) => a,
                    _ => unreachable!(),
                };
                output.insert(i+1, ItemType::Assign(name, data));
            },
            _ => unreachable!(),
        }
    }

    let mut output_file = match File::create(out_path) {
        Err(why) => panic!("Couldn't create output file: {}", why.description()),
        Ok(file) => file,
    };

    let mut first = true;
    for item in &output {
        match item {
            ItemType::OpCode(a) => {
                write!(output_file, "\n    {}", a).expect("Unable to write");
            }
            ItemType::Literal(a) => {
                write!(output_file, " {}", *a).expect("Unable to write");
            }
            ItemType::Relative(a) => {
                write!(output_file, " @{}", *a).expect("Unable to write");
            }
            ItemType::Reference(_) => {
                unreachable!();
            }
            ItemType::Address(_) => {
                unreachable!();
            }
            ItemType::Label(a) => {
                if !first {
                    write!(output_file, "\n\n").expect("Unable to write");
                }
                first = false;
                write!(output_file, "label_{}:", *a).expect("Unable to write");
            }
            ItemType::Variable(name, _) => {
                write!(output_file, " ${}", *name).expect("Unable to write");
            }
            ItemType::LabelAddress(name) => {
                write!(output_file, " :label_{}", *name).expect("Unable to write");
            }
            ItemType::Assign(name, value) => {
                write!(output_file, "\n    ${} = {}", *name, *value).expect("Unable to write");
            }
            ItemType::Data(_) => (),
            ItemType::None => (),
        }
    }
}
