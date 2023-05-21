mod chunk;
mod value;

use chunk::*;

fn main() {
    let mut chunk = Chunk::new();
    
    let constant = chunk.add_constant(1.2);
    chunk.write(OpCode::OpConstant, 123);
    chunk.write(constant as u8, 123);

    chunk.write(OpCode::OpReturn, 123);
    chunk.disassamble("test chunk");
}
