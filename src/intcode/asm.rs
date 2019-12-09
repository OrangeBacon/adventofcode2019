use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use std::process::exit;
use std::collections::HashMap;
use indexmap::map::IndexMap;
use regex::Regex;
use super::instruction::{*, Argument::*};

impl Environment {
    fn new() -> Environment {
        Environment {
            variables: IndexMap::new(),
            code: vec![],
            labels: HashMap::new(),
            line_num: 0,
            label_counter: 0,
        }
    }

    fn add_var(&mut self, name: &str, val: &str) {
        let num = match val.parse::<i64>() {
            Ok(a) => a,
            Err(err) => {
                println!("Could not parse number: {}", err);
                exit(1);
            }
        };
        if self.variables.contains_key(name) {
            println!("Duplicate variable: {}", name);
            exit(1);
        }
        self.variables.insert(name.to_string(), num);
    }
}

fn parse_arg(env: &mut Environment, word: &str) -> Option<Argument> {
    env.label_counter += 1;
    match word.chars().nth(0).unwrap() {
        '$' => {
            match env.variables.get_full(&word[1..].to_string()) {
                Some(a) => return Some(Variable(a.0)),
                None => {
                    println!("Undefined variable: {} on line {}", word[1..].to_string(), env.line_num);
                    exit(1);
                }
            }
        },
        ':' => return Some(Address(word[1..].to_string())),
        '@' => {
            let num = match word[1..].parse::<i64>() {
                Ok(a) => a,
                Err(_) => {
                    println!("Bad number in relative argument on line {}", env.line_num);
                    exit(1);
                }
            };
            return Some(Relative(num));
        }
        ';' => return None,
        _ => (),
    }
    match word.parse::<i64>() {
        Ok(a) => return Some(Literal(a)),
        Err(_) => {
            println!("Bad numeric argument: {} on line {}", word, env.line_num + 1);
            exit(1);
        }
    }
}

fn param_verify(opcode: OpCode, args: &Vec<Argument>, env: &Environment) {
    if args.len() != opcode.to_params().len() {
        println!("Wrong argument count on line {}, expecting {}, got {}", 
            env.line_num+1, opcode.to_params().len(), args.len());
        exit(0);
    }
    for (i, req) in opcode.to_params().iter().enumerate() {
        match *req {
            ParameterMode::Position => {
                match args[i] {
                    Variable(_) => (),
                    _ => {
                        println!("Position {} must be a variable on line {}", i, env.line_num+1);
                        exit(0);
                    },
                }
            }
            ParameterMode::Literal => {
                match args[i] {
                    Literal(_) => (),
                    _ => {
                        println!("Position {} must be a literal on line {}", i, env.line_num+1);
                        exit(0);
                    },
                }
            }
            ParameterMode::Address => {
                match args[i] {
                    Address(_) => (),
                    _ => {
                        println!("Position {} must be an address on line {}", i, env.line_num+1);
                        exit(0);
                    },
                }
            }
            _ => ()
        }
    }
}

pub fn asm(in_file: &str, out_file: &str) -> Vec<i64> {
    let path = Path::new(in_file);
    let display = path.display();

    let mut file = match File::open(&path) {
        Err(why) => panic!("Couldn't open {}: {}", display, why.description()),
        Ok(file) => file,
    };

    let mut s = String::new();
    match file.read_to_string(&mut s) {
        Err(why) => panic!("Couldn't read {}: {}", display, why.description()),
        Ok(_) => {}
    }

    let mut output = None;
    if out_file != "" {
        output = match File::create(out_file) {
            Err(why) => panic!("Couldn't create output file: {}", why.description()),
            Ok(file) => Some(file),
        };
    }

    let mut env = Environment::new();

    let var = Regex::new(
        r"\$([[:word:]]+)[[:space:]]*=[[:space:]](-?[[:digit:]]+)*"
    ).unwrap();
    let label = Regex::new(
        r"([[:word:]]+):"
    ).unwrap();

    for (line_num, line) in s.lines().enumerate() {
        env.line_num = line_num;
        let chars = line.trim();

        if chars.len() == 0 {
            continue;
        }

        match chars.chars().nth(0).unwrap() {
            ';' => continue,
            '$' => {
                match var.captures(chars) {
                    Some(a) => env.add_var(&a[1], &a[2]),
                    None => {
                        println!("Invalid variable assignment at line {}", line_num+1);
                        exit(1);
                    }
                }
                continue;
            }
            _ => ()
        }

        match label.captures(chars) {
            Some(a) => {
                if env.labels.contains_key(&a[1].to_string()) {
                    println!("Duplicate label: {} at line {}", chars, line_num+1);
                }
                env.labels.insert(a[1].to_string(), env.label_counter);
                continue;
            },
            None => (),
        }

        let mut words = chars.split_whitespace();
        let name = words.next().unwrap();
        let opcode = OpCode::from_asm_name(name);

        if opcode == OpCode::Unknown {
            println!("Unknown opcode: {}", name);
            exit(0);
        }

        env.label_counter += 1;

        let mut args = vec![];
        for _ in opcode.to_params() {
            let word = match words.next() {
                Some(a) => a,
                None => {
                    println!("Expecting another argument on line {}", line_num+1);
                    exit(0);
                }
            };
            match parse_arg(&mut env, word) {
                Some(a) => args.push(a),
                None => {
                    println!("Expecting another argument on line {}", line_num)
                }
            }
        }

        param_verify(opcode, &args, &mut env);
        env.code.push(Instruction::new(opcode, args));
    }

    if env.code.last().unwrap().opcode.to_i64() != 99 {
        env.code.push(Instruction::new(OpCode::Halt, vec![]));
    }

    let mut patches: Vec<(usize, usize)> = vec![];
    let mut output_nums: Vec<i64> = vec![];
    for instruction in &env.code {
        instruction.emit(&mut output_nums, &mut patches, &env);
    }

    let data_base = output_nums.len();
    for data in env.variables {
        output_nums.push(data.1);
    }

    for patch in patches {
        output_nums[patch.0] = (data_base + patch.1) as i64;
    }

    match output {
        Some(mut file) => {
            for (i, num) in output_nums.iter().enumerate() {
                if i != 0 {
                    file.write_all(b",").expect("Unable to write");
                }
                file.write_all(num.to_string().as_bytes()).expect("Unable to write");
            }
        },
        None => (),
    }

    return output_nums;
}