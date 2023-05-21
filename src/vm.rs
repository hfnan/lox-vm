
use crate::{chunk::*, value::Value};

macro_rules! binary_op {
    ($self: expr, $op: tt) => {{
        let b = $self.stack.pop().unwrap();
        let a = $self.stack.pop().unwrap();
        $self.stack.push(a $op b);
    }};
}
pub enum InterpretError {
    CompilerError,
    RuntimeError,
}

pub type InterpretResult<T> = Result<T, InterpretError>;


pub struct VM {
    ip: usize,
    stack: Vec<Value>,
}

impl VM {
    pub fn new() -> Self {   
        Self { ip: 0, stack: Vec::new() }
    }

    pub fn interpret(&mut self, chunk: &Chunk) -> InterpretResult<()> {
        self.ip = 0;
        self.run(chunk)
    }

    fn run(&mut self, chunk: &Chunk) -> InterpretResult<()> {
        loop {
            #[cfg(feature = "debug_trace_execution")] {
                print!("          ");
                for slot in &self.stack {
                    print!("[ {slot} ]");
                }
                println!();
                chunk.disassamble_instruction(self.ip);
            }

            let instruction = self.read_byte(chunk);
            match instruction {
                x if opcode!(OpCode::OpReturn, x) => {
                    println!("{}", self.stack.pop().unwrap());
                    return Ok(());
                },
                x if opcode!(OpCode::OpConstant, x) => {
                    let constant = self.read_constant(chunk);
                    self.stack.push(constant);
                },
                x if opcode!(OpCode::OpNegate, x) => {
                    let value = - self.stack.pop().unwrap();
                    self.stack.push(value);
                },
                x if opcode!(OpCode::OpAdd, x) => binary_op!(self, +),
                x if opcode!(OpCode::OpSubtract, x) => binary_op!(self, -),
                x if opcode!(OpCode::OpMultiply, x) => binary_op!(self, *),
                x if opcode!(OpCode::OpDivide, x) => binary_op!(self, /),
                _ => {}
            }
        }
        Ok(())
    }

    fn read_byte(&mut self, chunk: &Chunk) -> u8 {
        let val = chunk.get(self.ip);
        self.ip += 1;
        val
    }

    fn read_constant(&mut self, chunk: &Chunk) -> Value {
        let seq = self.read_byte(chunk);
        chunk.get_constant(seq as usize)
    }

    /* 
    fn reset_stack(&mut self) {
        self.stack = Vec::new();
    }
     */
}
