use std::iter::FromIterator;
use std::collections::VecDeque;
use std::mem;
use super::instruction::*;

#[derive(Debug)]
pub enum Interrupt {
    Input,
    Output(i32),
    Halt,
}

impl Interrupt {
    pub fn as_output(self) -> i32 {
        match self {
            Interrupt::Output(a) => a,
            a => {
                panic!("Expected output interrupt, got {:?}", a);
            }
        }
    }
}

pub struct RunData {
    code: Vec<i32>,
    ip: usize,
    inputs: VecDeque<i32>,
}

impl RunData {
    pub fn new(code: &Vec<i32>) -> RunData {
        return RunData {
            code: code.clone(),
            ip: 0,
            inputs: VecDeque::new(),
        }
    }

    pub fn input(&mut self, num: i32) {
        self.inputs.push_back(num);
    }

    pub fn input_vec(&mut self, nums: &[i32]) {
        for num in nums {
            self.inputs.push_back(*num);
        }
    }
}

fn getnum(code: &mut Vec<i32>, ip: usize, mode: ParameterMode) -> i32 {
    match mode {
        ParameterMode::Reference => code[num2ip(code[ip])],
        ParameterMode::Literal => code[ip],
        _ => unimplemented!("Cannot run virtual mode: {:?}", mode),
    }
}

fn num2ip(num: i32) -> usize {
    if num < 0 {
        panic!("Cannot read negative location");
    }
    num as usize
}

pub fn run(code: &mut Vec<i32>, input_data: &Vec<i32>) -> i32 {
    let mut data = RunData::new(code);
    let mut inputs = VecDeque::from_iter(input_data);
    loop {
        match run_yield(&mut data) {
            Interrupt::Halt => break,
            Interrupt::Input => {
                match inputs.pop_front() {
                    Some(a) => data.input(*a),
                    None => panic!("Ran out of inputs"),
                }
            },
            Interrupt::Output(a) => println!("{}", a),
        }
    }

    mem::replace(code, data.code);
    return code[0];
}

pub fn run_yield(data: &mut RunData) -> Interrupt {
    let code = &mut data.code;
    let ip = &mut data.ip;

    loop {
        let mut opcode = code[*ip];
        let mut modes = vec![ParameterMode::Reference; 3];

        if opcode >= 100 {
            let mut mode_num = (opcode - opcode % 100) / 100;
            let mut i = 0;
            while mode_num > 0 {
                modes[i] = ParameterMode::from_i32(mode_num % 10);
                mode_num -= mode_num % 10;
                mode_num /= 10;
                i += 1;
            }
            opcode %= 100;
        }

        match OpCode::from_i32(opcode) {
            OpCode::Add => {
                let loc = code[*ip+3];
                let a = getnum(code, *ip+1, modes[0]);
                let b = getnum(code, *ip+2, modes[1]);
                code[num2ip(loc)] = a + b;
                *ip += 4;
            },
            OpCode::Multiply => {
                let loc = code[*ip+3]; 
                let a = getnum(code, *ip+1, modes[0]);
                let b = getnum(code, *ip+2, modes[1]);
                code[num2ip(loc)] = a * b;
                *ip += 4;
            },
            OpCode::Input => {
                let loc = code[*ip+1];
                let val = match data.inputs.pop_front() {
                    Some(a) => a,
                    None => return Interrupt::Input,
                };
                code[num2ip(loc)] = val;
                *ip += 2;
            },
            OpCode::Output => {
                *ip += 2;
                return Interrupt::Output(getnum(code, *ip-1, modes[0])); 
            },
            OpCode::JumpNotZero => {
                if getnum(code, *ip+1, modes[0]) == 0 {
                    *ip += 3;
                } else {
                    *ip = num2ip(getnum(code, *ip+2, modes[1]));
                }
            },
            OpCode::JumpZero => {
                if getnum(code, *ip+1, modes[0]) == 0 {
                    *ip = num2ip(getnum(code, *ip+2, modes[1]));
                } else {
                    *ip += 3;
                }
            },
            OpCode::LessThan => {
                let loc = num2ip(code[*ip+3]);
                if getnum(code, *ip+1, modes[0]) < getnum(code, *ip+2, modes[1]) {
                    code[loc] = 1;
                } else {
                    code[loc] = 0;
                }
                *ip += 4;
            },
            OpCode::EqualTo => {
                let loc = num2ip(code[*ip+3]);
                if getnum(code, *ip+1, modes[0]) == getnum(code, *ip+2, modes[1]) {
                    code[loc] = 1;
                } else {
                    code[loc] = 0;
                }
                *ip += 4;
            }
            OpCode::Halt => return Interrupt::Halt,
            _ => unimplemented!("Unimplemented Opcode reached: {}, {:?} at {}", opcode, modes, ip),
        }
    }
}
