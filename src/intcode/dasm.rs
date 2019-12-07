use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::collections::VecDeque;
use std::collections::BTreeSet;
use std::fmt;
use indexmap::IndexMap;
use indexmap::IndexSet;
use super::input;

#[derive(Copy, Clone, Debug, PartialEq)]
enum Instructions {
    Add,
    Multiply,
    Input,
    Output,
    JumpNotZero,
    JumpZero,
    LessThan,
    EqualTo,
    Halt,
    Unknown,
}

impl Instructions {
    fn from_i32(val: i32) -> Instructions {
        match val {
             1 => Instructions::Add,
             2 => Instructions::Multiply,
             3 => Instructions::Input,
             4 => Instructions::Output,
             5 => Instructions::JumpNotZero,
             6 => Instructions::JumpZero,
             7 => Instructions::LessThan,
             8 => Instructions::EqualTo,
             99 => Instructions::Halt,
             _ => Instructions::Unknown,
        }
    }

    fn to_params(&self) -> Vec<i32> {
        match self {
            Instructions::Add => vec![-1, -1, 0],
            Instructions::Multiply => vec![-1, -1, 0],
            Instructions::Input => vec![0],
            Instructions::Output => vec![-1],
            Instructions::JumpNotZero => vec![-1, -2],
            Instructions::JumpZero => vec![-1, -2],
            Instructions::LessThan => vec![-1, -1, 0],
            Instructions::EqualTo => vec![-1, -1, 0],
            Instructions::Halt => vec![],
            Instructions::Unknown => vec![],
        }
    }

    fn to_asm_name(&self) -> &str {
        match self {
            Instructions::Add => "add",
            Instructions::Multiply => "mul",
            Instructions::Input => "inp",
            Instructions::Output => "out",
            Instructions::JumpNotZero => "jnz",
            Instructions::JumpZero => "jez",
            Instructions::LessThan => "clt",
            Instructions::EqualTo => "eql",
            Instructions::Halt => "hlt",
            Instructions::Unknown => {
                unreachable!("Cannot output unknown opcode");
            }
        }
    }
}

impl fmt::Display for Instructions {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.to_asm_name())
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
enum ItemType {
    Instruction(Instructions),
    Literal(i32),
    Reference(i32),
    Address(i32),
    Label(usize),
    Variable(usize, usize),
    LabelAddress(usize),
    Data(i32),
    Assign(usize, i32),
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

        let mut modes = vec![0,0,0];
        if opcode >= 100 {
            let mut mode_num = (opcode - opcode % 100) / 100;
            let mut i = 0;
            while mode_num > 0 {
                modes[i] = mode_num % 10;
                mode_num -= mode_num % 10;
                mode_num /= 10;
                i += 1;
            }
            opcode %= 100;
        }

        let inst = Instructions::from_i32(opcode);
        output[i] = ItemType::Instruction(inst);
        
        for (j, expected_mode) in inst.to_params().iter().enumerate() {
            i += 1;
            let param = match code.get(i) {
                Some(a) => *a,
                None => break,
            };
            let param_mode = match *expected_mode {
                -1 => modes[j],
                a => a,
            };

            output[i] = match param_mode {
                 0 => ItemType::Reference(param),
                 1 => ItemType::Literal(param),
                -2 => ItemType::Address(param),
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
            ItemType::Instruction(a) => {
                write!(output_file, "\n    {}", a).expect("Unable to write");
            }
            ItemType::Literal(a) => {
                write!(output_file, " {}", *a).expect("Unable to write");
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