use std::fmt;

#[derive(Copy, Clone, Debug)]
pub enum ParameterMode {
    Reference,
    Literal,
    Any,
    Address,
}

impl ParameterMode {
    pub fn from_i32(val: i32) -> ParameterMode {
        match val {
            0 => ParameterMode::Reference,
            1 => ParameterMode::Literal,
            -1 => ParameterMode::Any,
            -2 => ParameterMode::Address,
            a => {
                panic!("Unknown parameter mode: {}", a);
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Instruction {
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

impl Instruction {
    pub fn from_i32(val: i32) -> Instruction {
        match val {
             1 => Instruction::Add,
             2 => Instruction::Multiply,
             3 => Instruction::Input,
             4 => Instruction::Output,
             5 => Instruction::JumpNotZero,
             6 => Instruction::JumpZero,
             7 => Instruction::LessThan,
             8 => Instruction::EqualTo,
             99 => Instruction::Halt,
             _ => Instruction::Unknown,
        }
    }

    pub fn to_i32(&self) -> i32 {
        match self {
            Instruction::Add => 1,
            Instruction::Multiply => 2,
            Instruction::Input => 3,
            Instruction::Output => 4,
            Instruction::JumpNotZero => 5,
            Instruction::JumpZero => 6,
            Instruction::LessThan => 7,
            Instruction::EqualTo => 8,
            Instruction::Halt => 99,
            Instruction::Unknown => {
                panic!("Cannot write unknown instruction");
            },
       }
    }

    pub fn from_asm_name(name: &str) -> Instruction {
        match name {
            "add" => Instruction::Add,
            "mul" => Instruction::Multiply,
            "inp" => Instruction::Input,
            "out" => Instruction::Output,
            "jnz" => Instruction::JumpNotZero,
            "jez" => Instruction::JumpZero,
            "clt" => Instruction::LessThan,
            "eql" => Instruction::EqualTo,
            "hlt" => Instruction::Halt,
            _ => Instruction::Unknown,
        }
    }

    pub fn to_asm_name(&self) -> &str {
        match self {
            Instruction::Add => "add",
            Instruction::Multiply => "mul",
            Instruction::Input => "inp",
            Instruction::Output => "out",
            Instruction::JumpNotZero => "jnz",
            Instruction::JumpZero => "jez",
            Instruction::LessThan => "clt",
            Instruction::EqualTo => "eql",
            Instruction::Halt => "hlt",
            Instruction::Unknown => {
                unreachable!("Cannot output unknown opcode");
            }
        }
    }

    pub fn to_params(&self) -> Vec<ParameterMode> {
        use ParameterMode::*;
        match self {
            Instruction::Add => vec![Any, Any, Reference],
            Instruction::Multiply => vec![Any, Any, Reference],
            Instruction::Input => vec![Reference],
            Instruction::Output => vec![Any],
            Instruction::JumpNotZero => vec![Any, Address],
            Instruction::JumpZero => vec![Any, Address],
            Instruction::LessThan => vec![Any, Any, Reference],
            Instruction::EqualTo => vec![Any, Any, Reference],
            Instruction::Halt => vec![],
            Instruction::Unknown => vec![],
        }
    }
}

impl fmt::Display for Instruction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.to_asm_name())
    }
}