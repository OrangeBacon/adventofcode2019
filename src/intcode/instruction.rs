use std::fmt;

#[derive(Copy, Clone, Debug)]
pub enum ParameterMode {
    Position,
    Literal,
    Relative,
    Any,
    Address,
}

impl ParameterMode {
    pub fn from_i64(val: i64) -> ParameterMode {
        match val {
            0 => ParameterMode::Position,
            1 => ParameterMode::Literal,
            2 => ParameterMode::Relative,
            -1 => ParameterMode::Any,
            -2 => ParameterMode::Address,
            a => {
                panic!("Unknown parameter mode: {}", a);
            }
        }
    }
}

#[derive(Debug)]
pub enum Argument {
    Literal(i64),
    Variable(usize),
    Address(String),
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum OpCode {
    Add,
    Multiply,
    Input,
    Output,
    JumpNotZero,
    JumpZero,
    LessThan,
    EqualTo,
    Halt,
    RelativeAdjust,
    Unknown,
}

impl OpCode {
    pub fn from_i64(val: i64) -> OpCode {
        match val {
             1 => OpCode::Add,
             2 => OpCode::Multiply,
             3 => OpCode::Input,
             4 => OpCode::Output,
             5 => OpCode::JumpNotZero,
             6 => OpCode::JumpZero,
             7 => OpCode::LessThan,
             8 => OpCode::EqualTo,
             9 => OpCode::RelativeAdjust,
             99 => OpCode::Halt,
             _ => OpCode::Unknown,
        }
    }

    pub fn to_i64(&self) -> i64 {
        match self {
            OpCode::Add => 1,
            OpCode::Multiply => 2,
            OpCode::Input => 3,
            OpCode::Output => 4,
            OpCode::JumpNotZero => 5,
            OpCode::JumpZero => 6,
            OpCode::LessThan => 7,
            OpCode::EqualTo => 8,
            OpCode::RelativeAdjust => 9,
            OpCode::Halt => 99,
            OpCode::Unknown => {
                panic!("Cannot write unknown OpCode");
            }
       }
    }

    pub fn from_asm_name(name: &str) -> OpCode {
        match name {
            "add" => OpCode::Add,
            "mul" => OpCode::Multiply,
            "inp" => OpCode::Input,
            "out" => OpCode::Output,
            "jnz" => OpCode::JumpNotZero,
            "jez" => OpCode::JumpZero,
            "clt" => OpCode::LessThan,
            "eql" => OpCode::EqualTo,
            "rba" => OpCode::RelativeAdjust,
            "hlt" => OpCode::Halt,
            _ => OpCode::Unknown,
        }
    }

    pub fn to_asm_name(&self) -> &str {
        match self {
            OpCode::Add => "add",
            OpCode::Multiply => "mul",
            OpCode::Input => "inp",
            OpCode::Output => "out",
            OpCode::JumpNotZero => "jnz",
            OpCode::JumpZero => "jez",
            OpCode::LessThan => "clt",
            OpCode::EqualTo => "eql",
            OpCode::RelativeAdjust => "rba",
            OpCode::Halt => "hlt",
            OpCode::Unknown => {
                unreachable!("Cannot output unknown opcode");
            }
        }
    }

    pub fn to_params(&self) -> Vec<ParameterMode> {
        use ParameterMode::*;
        match self {
            OpCode::Add => vec![Any, Any, Position],
            OpCode::Multiply => vec![Any, Any, Position],
            OpCode::Input => vec![Position],
            OpCode::Output => vec![Any],
            OpCode::JumpNotZero => vec![Any, Address],
            OpCode::JumpZero => vec![Any, Address],
            OpCode::LessThan => vec![Any, Any, Position],
            OpCode::EqualTo => vec![Any, Any, Position],
            OpCode::RelativeAdjust => vec![Any],
            OpCode::Halt => vec![],
            OpCode::Unknown => vec![],
        }
    }
}

impl fmt::Display for OpCode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.to_asm_name())
    }
}

#[derive(Debug)]
pub struct Instruction {
    pub opcode: OpCode,
    pub args: Vec<Argument>,
}

impl Instruction {
    pub fn new(opcode: OpCode, args: Vec<Argument>) -> Instruction {
        Instruction {
            opcode: opcode,
            args: args,
        }
    }
}