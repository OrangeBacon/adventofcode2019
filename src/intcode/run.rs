use std::iter::FromIterator;
use std::collections::VecDeque;
use std::mem;
use super::instruction::*;

#[derive(Debug)]
pub enum Interrupt {
    Input,
    Output(i64),
    Halt,
}

impl Interrupt {
    pub fn as_output(self) -> i64 {
        match self {
            Interrupt::Output(a) => a,
            a => {
                panic!("Expected output interrupt, got {:?}", a);
            }
        }
    }
}

pub struct RunData {
    code: Vec<i64>,
    ip: usize,
    rb: usize,
    inputs: VecDeque<i64>,
}

impl RunData {
    pub fn new(code: &Vec<i64>) -> RunData {
        return RunData {
            code: code.clone(),
            ip: 0,
            rb: 0,
            inputs: VecDeque::new(),
        }
    }

    pub fn input(&mut self, num: i64) {
        self.inputs.push_back(num);
    }

    pub fn input_vec(&mut self, nums: &[i64]) {
        for num in nums {
            self.inputs.push_back(*num);
        }
    }
}

fn extend(code: &mut Vec<i64>, ip: usize, rb: usize) {
    if ip >= code.len() || rb + ip >= code.len() {
        code.extend(vec![0; 2*ip]);
    }
}

fn getnum(code: &mut Vec<i64>, ip: usize, rb: usize, mode: ParameterMode) -> &mut i64 {
    extend(code, ip, rb);
    match mode {
        ParameterMode::Position => {
            let loc = num2ip(code[ip]);
            extend(code, loc, rb);
            &mut code[loc]
        },
        ParameterMode::Literal => &mut code[ip],
        ParameterMode::Relative => {
            let loc = num2ip(rb as i64 + code[ip]);
            extend(code, loc, rb);
            &mut code[loc]
        },
        _ => unimplemented!("Cannot run virtual mode: {:?}", mode),
    }
}

fn num2ip(num: i64) -> usize {
    if num < 0 {
        panic!("Cannot read negative location");
    }
    num as usize
}

pub fn run(code: &mut Vec<i64>, input_data: &Vec<i64>) -> i64 {
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
    let rb = &mut data.rb;

    loop {
        let mut opcode = code[*ip];
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

        match OpCode::from_i64(opcode) {
            OpCode::Add => {
                let a = *getnum(code, *ip+1, *rb, modes[0]);
                let b = *getnum(code, *ip+2,*rb,  modes[1]);
                *getnum(code, *ip+3, *rb, modes[2]) = a + b;
                *ip += 4;
            },
            OpCode::Multiply => {
                let a = *getnum(code, *ip+1, *rb, modes[0]);
                let b = *getnum(code, *ip+2, *rb, modes[1]);
                *getnum(code, *ip+3, *rb, modes[2]) = a * b;
                *ip += 4;
            },
            OpCode::Input => {
                let val = match data.inputs.pop_front() {
                    Some(a) => a,
                    None => return Interrupt::Input,
                };
                *getnum(code, *ip+1, *rb, modes[0]) = val;
                *ip += 2;
            },
            OpCode::Output => {
                *ip += 2;
                return Interrupt::Output(*getnum(code, *ip-1, *rb, modes[0])); 
            },
            OpCode::JumpNotZero => {
                if *getnum(code, *ip+1, *rb, modes[0]) == 0 {
                    *ip += 3;
                } else {
                    *ip = num2ip(*getnum(code, *ip+2, *rb, modes[1]));
                }
            },
            OpCode::JumpZero => {
                if *getnum(code, *ip+1, *rb, modes[0]) == 0 {
                    *ip = num2ip(*getnum(code, *ip+2, *rb, modes[1]));
                } else {
                    *ip += 3;
                }
            },
            OpCode::LessThan => {
                if *getnum(code, *ip+1, *rb, modes[0]) < *getnum(code, *ip+2, *rb, modes[1]) {
                    *getnum(code, *ip+3, *rb, modes[2]) = 1;
                } else {
                    *getnum(code, *ip+3, *rb, modes[2]) = 0;
                }
                *ip += 4;
            },
            OpCode::EqualTo => {
                if *getnum(code, *ip+1, *rb, modes[0]) == *getnum(code, *ip+2, *rb, modes[1]) {
                    *getnum(code, *ip+3, *rb, modes[2]) = 1;
                } else {
                    *getnum(code, *ip+3, *rb, modes[2]) = 0;
                }
                *ip += 4;
            },
            OpCode::RelativeAdjust => {
                let loc = *getnum(code, *ip+1, *rb, modes[0]);
                if loc > 0 {
                    *rb += num2ip(loc);
                } else {
                    *rb -= num2ip(-loc);
                }
                *ip += 2;
            },
            OpCode::Halt => return Interrupt::Halt,
            _ => unimplemented!("Unimplemented Opcode reached: {}, {:?} at {}", opcode, modes, ip),
        }
    }
}
