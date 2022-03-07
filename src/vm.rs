use std::borrow::Borrow;
use crate::chunk::Chunk;
use crate::compiler::Compiler;
use crate::opcode::Opcode;
use crate::stack::Stack;
use crate::value::Value;

pub struct VM {
    pub stack: Stack<Value>
}
pub enum InterpretResult {
    Ok(Value),
    CompileError,
    RuntimeError
}


impl VM {

    pub fn new() -> Self {
        VM {
            stack: Stack::with_capacity(256)
        }
    }

    fn print_value(value: &Value) {
        println!("{}", value)
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

    pub fn pop_operand_as_number(&mut self) -> Result<f64, InterpretResult> {
        if !self.stack.peek(0).is_number()  {
            self.runtime_error("Operand must be numbers");
            return Err(InterpretResult::RuntimeError)
        }

        Ok(*self.stack.pop().as_number().unwrap())
    }

    pub fn pop_operand_as_numbers(&mut self) -> Result<(f64, f64), InterpretResult> {
        if !self.stack.peek(0).is_number() || !self.stack.peek(1).is_number() {
            self.runtime_error("Operands must be numbers");
            return Err(InterpretResult::RuntimeError)
        }
        let b = *self.stack.pop().as_number().unwrap();
        let a = *self.stack.pop().as_number().unwrap();
        Ok((a, b))
    }

    pub fn pop_operand_as_bool(&mut self) -> Result<bool, InterpretResult> {
        if !self.stack.peek(0).is_bool() {
            self.runtime_error("Operands must be boolean");
            return Err(InterpretResult::RuntimeError)
        }
        let b = *self.stack.pop().as_bool().unwrap();

        Ok(b)
    }


    ///
    ///
    pub fn run(&mut self, chunk: &Chunk) -> InterpretResult {
        for c in &chunk.op_codes
        {
            match c {
                Opcode::OpConstant(idx) => {
                    let const_val = chunk.constants.get(*idx).unwrap();
                    self.stack.push(const_val.borrow().clone());
                   // println!("const val {}", const_val);
                }
                Opcode::OpReturn => {
                    let ret_val = self.stack.pop();
                    VM::print_value(&ret_val);
                    return InterpretResult::Ok(ret_val.borrow().clone())
                }

                Opcode::OpNegate => {

                    match self.pop_operand_as_number() {
                        Ok(f) => {
                            // VM::replace(&mut self.stack, &mut Value::Number(-f));
                            self.stack.push( Value::Number(-f));

                        }
                        Err(result) => {
                            self.runtime_error("Operand must be a number");
                            return result;
                        }
                    }
                }

                Opcode::OpAdd => {

                    match self.pop_operand_as_numbers() {
                        Ok((a,b)) => {
                             self.stack.push( Value::Number(a + b))
                        }
                        Err(result) => {
                            return result
                        }
                    }
                }




                Opcode::OPSubtract => {
                    match self.pop_operand_as_numbers() {
                        Ok((a,b)) => {
                             self.stack.push( Value::Number(a - b))
                        }
                        Err(result) => {
                            return result
                        }
                    }
                }

                Opcode::OPMultiply => {
                    match self.pop_operand_as_numbers() {
                        Ok((a,b)) => {
                             self.stack.push( Value::Number(a * b))
                        }
                        Err(result) => {
                            return result
                        }
                    }
                }

                Opcode::OpDivide => {
                    match self.pop_operand_as_numbers() {
                        Ok((a,b)) => {
                             self.stack.push( Value::Number(a / b))
                        }
                        Err(result) => {
                            return result
                        }
                    }
                },
                Opcode::OpFalse =>  self.stack.push( Value::Boolean(false)),
                Opcode::OpNil =>  self.stack.push( Value::Nil),
                Opcode::OpTrue =>   self.stack.push( Value::Boolean(true)),

                Opcode::OpNot => {
                    match self.pop_operand_as_bool() {
                        Ok(bool_operand) => {
                             self.stack.push( Value::Boolean(!bool_operand))
                        }
                        Err(result) => {
                            return result
                        }
                    }
                }

                Opcode::OpEqual =>  {
                    let b = self.stack.pop();
                    let a = self.stack.pop();
                     self.stack.push( Value::Boolean(a == b))

                }

                Opcode::OpGreater => {
                    match self.pop_operand_as_numbers() {
                        Ok((a,b)) => {
                             self.stack.push( Value::Boolean(a > b))
                        }
                        Err(result) => {
                            return result
                        }
                    }
                }

                Opcode::OpLess => {
                    match self.pop_operand_as_numbers() {
                        Ok((a,b)) => {
                             self.stack.push( Value::Boolean(a < b))
                        }
                        Err(result) => {
                            return result
                        }
                    }
                }

            }
        }
        InterpretResult::RuntimeError
    }

}


#[cfg(test)]
mod tests {
    use crate::value::Value;
    use crate::vm::{InterpretResult, VM};

    fn assert_ok(vm: &mut VM, s:&str, expected_value: Value) {
        match vm.interpret(s) {
            InterpretResult::Ok(val) => {
                println!("Ok {}", val);
                assert_eq!(expected_value, val)
            }
            InterpretResult::CompileError => {
                panic!("CompileError")
            }
            InterpretResult::RuntimeError => {
                panic!("RuntimeError")
            }
        }
    }

    fn assert_runtime_error(vm: &mut VM, s:&str) {
        match vm.interpret(s) {
            InterpretResult::Ok(val) => {
                panic!("Expected RuntimeError, found OK({})", val)
            }
            InterpretResult::CompileError => {
                panic!("Expected RuntimeError, found CompileError")
            }
            InterpretResult::RuntimeError => {
                println!("RuntimeError")
            }
        }
    }


    #[test]
    fn vm_multiply() {
        assert_ok(&mut VM::new(), "1*2", Value::Number(2f64))
    }
    #[test]
    fn vm_add() {
        assert_ok(&mut VM::new(), "1 + 2", Value::Number(3f64))
    }
    #[test]
    fn vm_unary() {
        assert_ok(&mut VM::new(), "-1", Value::Number(-1f64));
    }
    #[test]
    fn vm_number() {
        assert_ok(&mut VM::new(), "1", Value::Number(1f64));
    }
    #[test]
    fn vm_grouping() {
        assert_ok(&mut VM::new(), "-(1)", Value::Number(-1f64));
    }
    #[test]
    fn vm_minus() {
        assert_ok(&mut VM::new(), " 2+5*10", Value::Number(52f64));
    }

    #[test]
    fn vm_bool_t() {
        assert_ok(&mut VM::new(),"true", Value::Boolean(true));
    }

    #[test]
    fn vm_bool_f() {
        assert_ok(&mut VM::new(),"false", Value::Boolean(false));
    }
    #[test]
    fn vm_bool_not() {
        assert_ok(&mut VM::new(),"!false", Value::Boolean(true));
    }
    #[test]
    fn vm_nil() {
        assert_ok(&mut VM::new(),"nil", Value::Nil);
    }

    #[test]
    fn vm_not_nil() {
        assert_runtime_error(&mut VM::new(),"!nil");
    }

    #[test]
    fn vm_not_number() {
        assert_runtime_error(&mut VM::new(),"!3.14");
    }

    #[test]
    fn vm_negate_bool() {
        assert_runtime_error(&mut VM::new(),"-false");
    }
    #[test]
    fn vm_negate_nil() {
        assert_runtime_error(&mut VM::new(),"-nil");
    }

    #[test]
    fn vm_greater() {
        assert_ok(&mut VM::new(),"2 > 1", Value::Boolean(true));
        assert_ok(&mut VM::new(),"2 >= 1", Value::Boolean(true));
    }

    #[test]
    fn vm_less() {
        assert_ok(&mut VM::new(),"2 < 1", Value::Boolean(false));
        assert_ok(&mut VM::new(),"2 <= 1", Value::Boolean(false));
    }
    #[test]
    fn vm_equal() {
        assert_ok(&mut VM::new(),"2 == 2", Value::Boolean(true));
    }

    #[test]
    fn vm_equal_fail() {
        assert_ok(&mut VM::new(),"2 == 2", Value::Boolean(true));
    }
}