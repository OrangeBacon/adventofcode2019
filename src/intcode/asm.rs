use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use std::process::exit;
use indexmap::map::IndexMap;
use regex::Regex;

#[derive(Debug)]
enum Argument {
    Literal(i32),
    Variable(usize),
}
use Argument::*;

#[derive(Debug)]
struct Opcode {
    number: i32,
    args: Vec<Argument>,
    arg_count: usize,
    arg_requirements: Vec<i32>
}

#[derive(Debug)]
struct Environment {
    variables: IndexMap<String, i32>,
    code: Vec<Opcode>,
}

impl Environment {
    fn new() -> Environment {
        Environment {
            variables: IndexMap::new(),
            code: vec![],
        }
    }

    fn add_var(&mut self, name: &str, val: &str) {
        let num = match val.parse::<i32>() {
            Ok(a) => a,
            Err(err) => {
                println!("Could not parse number: {}", err);
                exit(1);
            }
        };
        self.variables.insert(name.to_string(), num);
    }
}

pub fn asm(in_file: &str, out_file: &str) -> Vec<i32> {
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

    for (line_num, line) in s.lines().enumerate() {
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

        let mut opcode = Opcode {number: 0, args: vec![], arg_count: 0, arg_requirements: vec![]};
        let mut words = chars.split_whitespace();

        match words.next().unwrap() {
            "add" => {
                opcode.number = 1;
                opcode.arg_count = 3;
                opcode.arg_requirements = vec![-1, -1, 0];
            },
            "mul" => {
                opcode.number = 2;
                opcode.arg_count = 3;
                opcode.arg_requirements = vec![-1, -1, 0];
            },
            "inp" => {
                opcode.number = 3;
                opcode.arg_count = 1;
                opcode.arg_requirements = vec![0];
            }
            "out" => {
                opcode.number = 4;
                opcode.arg_count = 1;
                opcode.arg_requirements = vec![-1];
            }
            "nlt" => {
                opcode.number = 7;
                opcode.arg_count = 3;
                opcode.arg_requirements = vec![-1, -1, 0];
            }
            "eql" => {
                opcode.number = 8;
                opcode.arg_count = 3;
                opcode.arg_requirements = vec![-1, -1, 0];
            }
            a => {
                println!("Unrecognised opcode: {} on line {}", a, line_num+1);
                exit(1);
            }
        }

        for word in words {
            match word.chars().nth(0).unwrap() {
                '$' => {
                    match env.variables.get_full(&word[1..].to_string()) {
                        Some(a) => opcode.args.push(Variable(a.0)),
                        None => {
                            println!("Undefined variable: {} on line {}", word[1..].to_string(), line_num+1);
                            exit(1);
                        }
                    }
                    continue;
                },
                ';' => break,
                _ => (),
            }
            match word.parse::<i32>() {
                Ok(a) => opcode.args.push(Literal(a)),
                Err(_) => {
                    println!("Bad numeric argument: {} on line {}", word, line_num + 1);
                    exit(1);
                }
            }
        }

        if opcode.args.len() != opcode.arg_count {
            println!("Wrong argument count on line {}, expecting {}, got {}", 
                line_num+1, opcode.arg_count, opcode.args.len());
            exit(0);
        }

        for (i, req) in opcode.arg_requirements.iter().enumerate() {
            match req {
                0 => {
                    match opcode.args[i] {
                        Literal(_) => {
                            println!("Position {} must be a variable on line {}", i, line_num+1);
                            exit(0);
                        },
                        Variable(_) => (),
                    }
                }
                1 => {
                    match opcode.args[i] {
                        Variable(_) => {
                            println!("Position {} must be a literal on line {}", i, line_num+1);
                            exit(0);
                        },
                        Literal(_) => (),
                    }
                }
                _ => ()
            }
        }

        env.code.push(opcode);
    }

    env.code.push(Opcode {number: 99, args: vec![], arg_count: 0, arg_requirements: vec![]});

    let mut patches: Vec<(usize, usize)> = vec![];
    let mut output_nums: Vec<i32> = vec![];
    for opcode in env.code {
        let mut num = opcode.number;
        for (i, arg) in opcode.args.iter().enumerate() {
            match arg {
                Literal(_) => num += 10i32.pow(i as u32 + 2),
                Variable(_) => (),
            }
        }
        output_nums.push(num);

        for arg in &opcode.args {
            match arg {
                Literal(a) => output_nums.push(*a),
                Variable(a) => {
                    patches.push((output_nums.len(), *a));
                    output_nums.push(-1);
                },
            }
        }
    }

    let data_base = output_nums.len();
    for data in env.variables {
        output_nums.push(data.1);
    }

    for patch in patches {
        output_nums[patch.0] = (data_base + patch.1) as i32;
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