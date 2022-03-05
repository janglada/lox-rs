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

    fn replace(stack: &mut Vec<Value>, value: &mut Value) {
        stack.pop().unwrap();
        stack.push(*value);
    }


    fn print_value(value: Value) {
        print!("VALUE = {:?} ", value)
    }

    ///
    ///
    fn peek(stack: &Vec<Value>, distance: usize) -> &Value{
       stack.get(stack.len() - (distance +1)).unwrap()
    }


    pub fn interpret(&mut self, source: &str) -> InterpretResult {
        let mut chunk = Chunk::new();
        let mut compiler = Compiler::new(source, &mut chunk);
        if !compiler.compile() {
            return InterpretResult::CompileError;
        }

        self.run(&chunk)

    }

    fn runtime_error(&mut self, msg: &str,) {
        eprintln!("Runtime error {}", msg);
    }

    pub fn ensure_number_binary_operands(&mut self) -> Result<(f64, f64), InterpretResult> {
        if !VM::peek(&self.stack,0).is_number() || !VM::peek(&self.stack,1).is_number() {
            self.runtime_error("Operands must be numbers");
            return Err(InterpretResult::RuntimeError)
        }
        let b = *VM::pop(&mut self.stack).as_number().unwrap();
        let a = *VM::pop(&mut self.stack).as_number().unwrap();
        Ok((a, b))
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


                    let mut_stack =  &mut self.stack;

                    let value = VM::peek(mut_stack, 0 );
                    // let value:Value = self.stack.as_mut().get_mut(len - (0 +1)).unwrap();
                   // let value2 = VM::peek2(ptr, 0);
                    match value.as_number() {
                        Ok(f) => {
                            VM::replace(mut_stack, &mut Value::Number(-*f));
                            // VM::pop(mut_stack);
                            // VM::push(mut_stack, Value::Number(-*f));
                        }
                        Err(msg) => {
                            self.runtime_error("Operand must be a number")
                        }
                    }
                }

                Opcode::OpAdd => {

                    match self.ensure_number_binary_operands() {
                        Ok((a,b)) => {
                            VM::push(&mut self.stack, Value::Number(a + b))
                        }
                        Err(result) => {
                            return result
                        }
                    }

                }

                Opcode::OPSubtract => {
                    match self.ensure_number_binary_operands() {
                        Ok((a,b)) => {
                            VM::push(&mut self.stack, Value::Number(a - b))
                        }
                        Err(result) => {
                            return result
                        }
                    }
                }

                Opcode::OPMultiply => {
                    match self.ensure_number_binary_operands() {
                        Ok((a,b)) => {
                            VM::push(&mut self.stack, Value::Number(a * b))
                        }
                        Err(result) => {
                            return result
                        }
                    }
                }

                Opcode::OpDivide => {
                    match self.ensure_number_binary_operands() {
                        Ok((a,b)) => {
                            VM::push(&mut self.stack, Value::Number(a / b))
                        }
                        Err(result) => {
                            return result
                        }
                    }
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