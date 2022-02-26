use crate::chunk::{Chunk, Value};
use crate::opcode::Opcode;

pub struct VM {
    pub stack: Vec<Value>
}
pub enum InterpretResult {
    Ok,
    CompileError,
    RuntimeError
}

impl VM {

    pub fn new() -> Self {
        VM {
            stack: Vec::with_capacity(256)
        }
    }

    fn reset_stack(&mut self) {
        self.stack.clear();
    }

    fn push(stack: &mut Vec<Value>, value: Value) {
        stack.push(value);
    }

    fn pop(stack: &mut Vec<Value>) -> Value {
        stack.pop().unwrap()
    }

    fn print_value(value: Value) {
        print!("VALUE = {} ", value)
    }

    pub fn interpret(&mut self, chunk: &Chunk) -> InterpretResult {
        for c in &chunk.op_codes
        {
            match c {
                Opcode::OpConstant(idx) => {
                    let const_val = chunk.constants.get(*idx).unwrap();
                    VM::push(&mut self.stack,*const_val);
                    println!("const val {}", const_val);
                }
                Opcode::OpReturn => {

                    VM::print_value(VM::pop(&mut self.stack));

                    return InterpretResult::Ok
                }

                Opcode::OPNegate => {
                    let value = -VM::pop(&mut self.stack);
                    {
                        VM::push(&mut self.stack, value);
                    }

                }

            }
        }
        return InterpretResult::RuntimeError
    }
}