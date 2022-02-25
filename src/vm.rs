use crate::chunk::Chunk;
use crate::opcode::Opcode;

pub struct VM {

}
pub enum InterpretResult {
    Ok,
    CompileError,
    RuntimeError
}

impl VM {
    fn interpret(&self, chunk: &Chunk) -> InterpretResult {
        for c in &chunk.op_codes {
            match c {
                Opcode::OpConstant(idx) => {
                    let const_val = chunk.constants.get(*idx).unwrap();
                    println!("const val {}", const_val);
                }
                Opcode::OpReturn => {
                    return InterpretResult::Ok
                }
            }
        }
        return InterpretResult::RuntimeError
    }
}