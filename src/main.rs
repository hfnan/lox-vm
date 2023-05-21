#[macro_use]
mod chunk;
mod value;
mod vm;

use chunk::*;
use vm::VM;

fn main() {
    let mut vm = VM::new();

    let mut chunk = Chunk::new();
    
    let mut constant = chunk.add_constant(1.2);
    chunk.write(OpCode::OpConstant, 123);
    chunk.write(constant as u8, 123);
    constant = chunk.add_constant(3.4);
    chunk.write(OpCode::OpConstant, 123);
    chunk.write(constant as u8, 123);

    chunk.write(OpCode::OpAdd, 123);

    constant = chunk.add_constant(5.6);
    chunk.write(OpCode::OpConstant, 123);
    chunk.write(constant as u8, 123);
    
    chunk.write(OpCode::OpDivide, 123);
    chunk.write(OpCode::OpNegate, 123);
    chunk.write(OpCode::OpReturn, 123);
    chunk.disassamble("test chunk");

    match vm.interpret(&chunk) { _ => {}};
}
