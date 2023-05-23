

use std::{rc::Rc};

use crate::{chunk::*, value::Value, compiler::*};

macro_rules! binary_op {
    ($self: expr, $value_type: ident, $op: tt) => {{
        if let (Value::Number(b), Value::Number(a)) = ($self.peek(0), $self.peek(1)) {
            $self.stack.pop().unwrap();
            $self.stack.pop().unwrap();
            $self.stack.push($value_type!(a $op b));
            Ok(())
        } else {
            $self.runtime_error("Operands must be Numbers.")
        }
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
    chunk: Option<Rc<Chunk>>,
}

impl VM {
    pub fn new() -> Self {   
        Self { ip: 0, stack: Vec::new(), chunk: None }
    }

    pub fn interpret(&mut self, source: &str) -> InterpretResult<()> {
        let mut chunk = Chunk::new();
        let mut compiler = Compiler::new(source, &mut chunk);
        compiler.compile()?;
        self.ip = 0;
        self.run(chunk)
    }

    fn run(&mut self, chunk: Chunk) -> InterpretResult<()> {
        self.load_chunk(chunk);
        let chunk = self.get_chunk();
        loop {
            #[cfg(feature = "debug_trace_execution")] {
                print!("          ");
                for slot in &self.stack {
                    print!("[ {slot} ]");
                }
                println!();
                chunk.disassamble_instruction(self.ip);
            }

            let instruction = self.read_byte(&chunk);
            match instruction.into() {
                OpCode::Return => {
                    println!("{}", self.stack.pop().unwrap());
                    return Ok(());
                },
                OpCode::Constant => {
                    let constant = self.read_constant(&chunk);
                    self.stack.push(constant);
                },
                OpCode::Nil => self.stack.push(nil_val!()),
                OpCode::False => self.stack.push(bool_val!(false)),
                OpCode::True => self.stack.push(bool_val!(true)),
                OpCode::Equal => {
                    let b = self.stack.pop().unwrap();
                    let a = self.stack.pop().unwrap();
                    self.stack.push(bool_val!(a == b));
                }
                OpCode::Not => {
                    let value = self.stack.pop().unwrap();
                    self.stack.push(bool_val!(is_falsey!(value)))
                }
                OpCode::Negate => {
                    let last = self.stack.len() - 1;
                    if let Value::Number(value) = self.stack[last] {
                        self.stack[last] = number_val!(- value);
                    } else {
                        self.runtime_error("Operand must be a number.")?;
                    }
                },
                OpCode::Greater => binary_op!(self, bool_val, >)?,
                OpCode::Less => binary_op!(self, bool_val, <)?,
                OpCode::Add => binary_op!(self, number_val, +)?,
                OpCode::Subtract => binary_op!(self, number_val, -)?,
                OpCode::Multiply => binary_op!(self, number_val, *)?,
                OpCode::Divide => binary_op!(self, number_val, /)?,
                _ => {}
            }
        }
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

    fn peek(&self, distance: usize) -> Value {
        self.stack[self.stack.len() - 1 - distance]
    } 

    fn runtime_error(&mut self, format: &str) -> InterpretResult<()> {
        eprintln!("{format}");
        let line = self.get_chunk().lines[self.ip - 1];
        eprintln!("[line {line}] in script");
        self.reset_stack();

        Err(InterpretError::RuntimeError)
    }

    fn reset_stack(&mut self) {
        self.stack.clear();
    }

    fn load_chunk(&mut self, chunk: Chunk) {
        self.chunk = Some(Rc::new(chunk))
    }

    fn get_chunk(&self) -> Rc<Chunk> {
        if let Some(chunk) = &self.chunk {
            chunk.clone()
        } else {
            panic!("No chunk.")
        }
    }
}
