#[repr(u8)]
pub enum OpCode {
    OP_RETURN,
}

macro_rules! opcode {
    ($code: expr, $x: expr) => {
         $code as u8 == $x
    };
}

pub struct Chunk {
    code: Vec<u8>,
}

impl Chunk {
    pub fn new() -> Self {
        Self { code: Vec::new() }
    }

    pub fn write(&mut self, byte: u8) {
        self.code.push(byte)
    }

    pub fn write_opcode(&mut self, opcode: OpCode) {
        self.code.push(opcode as u8);
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
        
        let instruction = self.code[offset];
        match instruction {
            x if opcode!(OpCode::OP_RETURN, x) => self.simple_instruction("OP_RETURN", offset),
            _ => {
                println!("Unknown opcode {instruction}");
                offset + 1
            },
        }
    }

    fn simple_instruction(&self, name: &str, offset: usize) -> usize {
        println!("{name}");
        offset + 1
    }
}
