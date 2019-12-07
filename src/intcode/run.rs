use std::io;
use std::io::Write;

fn getnum(code: &mut Vec<i32>, ip: usize, mode: i32) -> i32 {
    match mode {
        0 => code[num2ip(code[ip])],
        1 => code[ip],
        _ => unimplemented!("Bad mode {}", mode),
    }
}

fn num2ip(num: i32) -> usize {
    if num < 0 {
        panic!("Cannot read negative location");
    }
    num as usize
}

pub fn run(code: &mut Vec<i32>, input: &Vec<i32>) -> i32 {
    let mut ip = 0usize;
    let mut input_idx = 0;
    loop {
        let mut opcode = code[ip];
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

        match opcode {
            1 => {
                let loc = code[ip+3];
                let a = getnum(code, ip+1, modes[0]);
                let b = getnum(code, ip+2, modes[1]);
                code[num2ip(loc)] = a + b;
                ip += 4;
            },
            2 => {
                let loc = code[ip+3]; 
                let a = getnum(code, ip+1, modes[0]);
                let b = getnum(code, ip+2, modes[1]);
                code[num2ip(loc)] = a * b;
                ip += 4;
            },
            3 => {
                let loc = code[ip+1];
                let inval;
                if input_idx < input.len() {
                    inval = input[input_idx];
                    input_idx += 1;
                } else {
                    let mut input = String::new();
                    print!("> ");
                    io::stdout().flush().expect("Could not write prompt");
                    match io::stdin().read_line(&mut input) {
                        Ok(_) => inval = match input.trim().parse::<i32>() {
                            Ok(a) => a,
                            Err(_) => panic!("Didn't read number"),
                        },
                        Err(_) => panic!("Unable to read"),
                    }
                }
                code[num2ip(loc)] = inval;
                ip += 2;
            },
            4 => {
                println!("{}", getnum(code, ip+1, modes[0])); 
                ip += 2;
            },
            5 => {
                if getnum(code, ip+1, modes[0]) == 0 {
                    ip += 3;
                } else {
                    ip = num2ip(getnum(code, ip+2, modes[1]));
                }
            },
            6 => {
                if getnum(code, ip+1, modes[0]) == 0 {
                    ip = num2ip(getnum(code, ip+2, modes[1]));
                } else {
                    ip += 3;
                }
            },
            7 => {
                let loc = num2ip(code[ip+3]);
                if getnum(code, ip+1, modes[0]) < getnum(code, ip+2, modes[1]) {
                    code[loc] = 1;
                } else {
                    code[loc] = 0;
                }
                ip += 4;
            },
            8 => {
                let loc = num2ip(code[ip+3]);
                if getnum(code, ip+1, modes[0]) == getnum(code, ip+2, modes[1]) {
                    code[loc] = 1;
                } else {
                    code[loc] = 0;
                }
                ip += 4;
            }
            99 => break,
            _ => unimplemented!("Unimplemented Opcode reached: {}, {:?} at {}", opcode, modes, ip),
        }
    }
    return code[0];
}

pub struct RunData {
    pub code: Vec<i32>,
    pub ip: usize,
}

pub fn run_yield(data: &mut RunData, input: i32) -> Option<i32> {
    let code = &mut data.code;
    let ip = &mut data.ip;
    let mut has_input = false;

    loop {
        let mut opcode = code[*ip];
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

        match opcode {
            1 => {
                let loc = code[*ip+3];
                let a = getnum(code, *ip+1, modes[0]);
                let b = getnum(code, *ip+2, modes[1]);
                code[num2ip(loc)] = a + b;
                *ip += 4;
            },
            2 => {
                let loc = code[*ip+3]; 
                let a = getnum(code, *ip+1, modes[0]);
                let b = getnum(code, *ip+2, modes[1]);
                code[num2ip(loc)] = a * b;
                *ip += 4;
            },
            3 => {
                let loc = code[*ip+1];
                let inval;
                if has_input {
                    return None;
                } else {
                    has_input = true;
                    inval = input;
                }
                code[num2ip(loc)] = inval;
                *ip += 2;
            },
            4 => {
                *ip += 2;
                return Some(getnum(code, *ip-1, modes[0])); 
            },
            5 => {
                if getnum(code, *ip+1, modes[0]) == 0 {
                    *ip += 3;
                } else {
                    *ip = num2ip(getnum(code, *ip+2, modes[1]));
                }
            },
            6 => {
                if getnum(code, *ip+1, modes[0]) == 0 {
                    *ip = num2ip(getnum(code, *ip+2, modes[1]));
                } else {
                    *ip += 3;
                }
            },
            7 => {
                let loc = num2ip(code[*ip+3]);
                if getnum(code, *ip+1, modes[0]) < getnum(code, *ip+2, modes[1]) {
                    code[loc] = 1;
                } else {
                    code[loc] = 0;
                }
                *ip += 4;
            },
            8 => {
                let loc = num2ip(code[*ip+3]);
                if getnum(code, *ip+1, modes[0]) == getnum(code, *ip+2, modes[1]) {
                    code[loc] = 1;
                } else {
                    code[loc] = 0;
                }
                *ip += 4;
            }
            99 => break,
            _ => unimplemented!("Unimplemented Opcode reached: {}, {:?} at {}", opcode, modes, ip),
        }
    }

    return None;
}