use crate::value::*;

#[repr(u8)]
pub enum OpCode {
    Constant,
    Add, 
    Subtract,
    Multiply,
    Divide,
    Negate,
    Return,
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

    pub fn get(&self, ip: usize) -> u8 {
        self.code[ip]
    }

    pub fn add_constant(&mut self, value: Value) -> usize {
        self.constants.write(value);
        self.constants.len() - 1
    }

    pub fn get_constant(&self, seq: usize) -> Value {
        self.constants.get(seq)
    }

    pub fn print_value(&self, constant: Value) {
        print!("{constant}")
    }

    pub fn disassamble(&self, name: &str) {
        println!("== {name} ==");

        let mut offset = 0;
        while offset < self.code.len() {
            offset = self.disassamble_instruction(offset);
        }
    }

    pub fn disassamble_instruction(&self, offset: usize) -> usize {
        print!("{offset:04} ");

        if offset > 0 && self.lines[offset] == self.lines[offset - 1] {
            print!("   | ");
        } else {
            print!("{:4} ", self.lines[offset]);
        }
        
        let instruction = self.code[offset];
        match instruction {
            x if opcode!(OpCode::Constant, x) => self.constant_instruction("OP_CONSTANT", offset),
            x if opcode!(OpCode::Return, x) => self.simple_instruction("OP_RETURN", offset),
            x if opcode!(OpCode::Negate, x) => self.simple_instruction("OP_NEGATE", offset),
            x if opcode!(OpCode::Add, x) => self.simple_instruction("OP_ADD", offset),
            x if opcode!(OpCode::Subtract, x) => self.simple_instruction("OP_SUBTRACT", offset),
            x if opcode!(OpCode::Multiply, x) => self.simple_instruction("OP_MULTIPLY", offset),
            x if opcode!(OpCode::Divide, x) => self.simple_instruction("OP_DIVIDE", offset),
            _ => {
                println!("Unknown opcode {instruction}");
                offset + 1
            },
        }
    }

    fn constant_instruction(&self, name: &str, offset: usize) -> usize {
        let seq = self.code[offset + 1];
        let constant = self.constants.get(seq as usize);
        print!("{name:-16} {seq:4} '");
        self.print_value(constant);
        println!("'");
        offset + 2
    }

    fn simple_instruction(&self, name: &str, offset: usize) -> usize {
        println!("{name}");
        offset + 1
    }
}
