mod chunk;

use chunk::*;

fn main() {
    let mut chunk = Chunk::new();
    
    chunk.write_opcode(OpCode::OP_RETURN);
    chunk.disassamble("test chunk");
}
