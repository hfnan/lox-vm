use crate::value::*;

#[repr(u8)]
pub enum OpCode {
    OpConstant,
    OpReturn,
}

impl Into<u8> for OpCode {
    fn into(self) -> u8 {
        self as u8
    }
}

macro_rules! opcode {
    ($code: expr, $x: expr) => {
         $code as u8 == $x
    };
}   

pub struct Chunk {
    code: Vec<u8>,
    lines: Vec<usize>,
    constants: ValueArray,
}

impl Chunk {
    pub fn new() -> Self {
        Self { 
            code: Vec::new(),
            constants: ValueArray::new(),
            lines: Vec::new(),
        }
    }

    pub fn write<T: Into<u8>>(&mut self, byte: T, line: usize) {
        self.code.push(byte.into());
        self.lines.push(line)
    }

    pub fn add_constant(&mut self, value: Value) -> usize {
        self.constants.write(value);
        self.constants.len() - 1
    }

    pub fn disassamble(&self, name: &str) {
        println!("== {name} ==");

        let mut offset = 0;
        while offset < self.code.len() {
            offset = self.disassamble_instruction(offset);
        }
    }

    fn disassamble_instruction(&self, offset: usize) -> usize {
        print!("{offset:04} ");

        if offset > 0 && self.lines[offset] == self.lines[offset - 1] {
            print!("   | ");
        } else {
            print!("{:4} ", self.lines[offset]);
        }
        
        let instruction = self.code[offset];
        match instruction {
            x if opcode!(OpCode::OpConstant, x) => self.constant_instruction("OP_CONSTANT", offset),
            x if opcode!(OpCode::OpReturn, x) => self.simple_instruction("OP_RETURN", offset),
            _ => {
                println!("Unknown opcode {instruction}");
                offset + 1
            },
        }
    }

    fn constant_instruction(&self, name: &str, offset: usize) -> usize {
        let constant = self.code[offset + 1];
        print!("{name:-16} {constant:4} '");
        self.constants.print_value(constant as usize);
        println!("'");
        offset + 2
    }

    fn simple_instruction(&self, name: &str, offset: usize) -> usize {
        println!("{name}");
        offset + 1
    }
}
