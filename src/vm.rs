use crate::chunk::{Chunk, Value};
use crate::compiler::Compiler;

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

    pub fn interpret(&mut self, source: &str) -> InterpretResult {
        let mut chunk = Chunk::new();
        let mut compiler = Compiler::new(source, &mut chunk);
        if !compiler.compile() {
            return InterpretResult::CompileError;
        }

        self.run(&chunk)

    }

    ///
    ///
    pub fn run(&mut self, chunk: &Chunk) -> InterpretResult {
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

                Opcode::OpNegate => {
                    let value = -VM::pop(&mut self.stack);
                    {
                        VM::push(&mut self.stack, value);
                    }

                }

                Opcode::OpAdd => {
                    let b = VM::pop(&mut self.stack);
                    let a = VM::pop(&mut self.stack);
                    VM::push(&mut self.stack, a + b);
                }

                Opcode::OPSubtract => {
                    let b = VM::pop(&mut self.stack);
                    let a = VM::pop(&mut self.stack);
                    VM::push(&mut self.stack, a - b);
                }

                Opcode::OPMultiply => {
                    let b = VM::pop(&mut self.stack);
                    let a = VM::pop(&mut self.stack);
                    VM::push(&mut self.stack, a * b);
                }

                Opcode::OpDivide => {
                    let b = VM::pop(&mut self.stack);
                    let a = VM::pop(&mut self.stack);
                    VM::push(&mut self.stack, a / b);
                }

            }
        }
        return InterpretResult::RuntimeError
    }

}


#[cfg(test)]
mod tests {
    use crate::vm::{InterpretResult, VM};

    fn assert_ok(s:&str) {
        let mut vm = VM::new();
        match vm.interpret(s) {
            InterpretResult::Ok => {
                println!("Ok")
            }
            InterpretResult::CompileError => {
                panic!("CompileError")
            }
            InterpretResult::RuntimeError => {
                panic!("RuntimeError")
            }
        }
    }

    #[test]
    fn vm_basic() {
        assert_ok("1*2")
    }

    #[test]
    fn vm_unary() {
        assert_ok("-1");
    }
    #[test]
    fn vm_number() {
        assert_ok("1");
    }
    #[test]
    fn vm_grouping() {
        assert_ok("-(1)");
    }
    #[test]
    fn vm_minus() {
        assert_ok(" 2+5*10");
    }
}