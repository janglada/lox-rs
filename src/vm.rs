use std::borrow::Borrow;
use std::collections::HashMap;
use std::iter::Map;
use crate::chunk::{Chunk};
use crate::compiler::Compiler;
use crate::opcode::Opcode;
use crate::stack::Stack;
use crate::value::{ObjectValue, Value};

pub struct VM {
    pub stack: Stack<Value>,
    pub globals: HashMap<String, Value>
}
pub enum InterpretResult {
    Ok(Option<Value>),
    CompileError,
    RuntimeError
}


impl VM {

    pub fn new() -> Self {
        VM {
            stack: Stack::with_capacity(256),
            globals: HashMap::new()
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
            return Err(InterpretResult::RuntimeError)
        }
        let b = *self.stack.pop().as_number().unwrap();
        let a = *self.stack.pop().as_number().unwrap();
        Ok((a, b))
    }
    pub fn pop_operand_as_strings(&mut self) -> Result<(String, String), InterpretResult> {
        if !self.stack.peek(0).is_string() || !self.stack.peek(1).is_string() {
            return Err(InterpretResult::RuntimeError)
        }
        let b = self.stack.pop().as_string().unwrap().clone();
        let a = self.stack.pop().as_string().unwrap().clone();
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

    fn is_falsey(v: &Value) -> bool{
        match v {
            Value::Nil => true,
            Value::Boolean(b) => !b,
            _ => false
        }
    }


    ///
    ///
    pub fn run(&mut self, chunk: &Chunk) -> InterpretResult {
        // for c in &chunk.op_codes
        let mut op_code_iter = chunk.op_codes.iter();
        while let Some(c) = op_code_iter.next()
        {
            match c {
                Opcode::OpConstant(idx) => {
                    let const_val = chunk.constants.get(*idx).unwrap();
                    self.stack.push(const_val.borrow().clone());
                   // println!("const val {}", const_val);
                }
                Opcode::OpReturn => {
                    return if let Some(ret_val) = self.stack.safe_pop() {
                        VM::print_value(&ret_val);
                        InterpretResult::Ok(Some(ret_val.borrow().clone()))
                    } else {
                        InterpretResult::Ok(None)
                    }
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
                        _ => {
                            match self.pop_operand_as_strings() {
                                Ok((a,b)) => {
                                    self.stack.push( Value::Object(ObjectValue::String(format!("{}{}", a, b))))
                                }
                                _ => {
                                    self.runtime_error("Operands must be of same type");
                                    return InterpretResult::RuntimeError
                                }
                            }
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

                Opcode::OpPrint => {
                    println!("{}", self.stack.pop());
                }

                Opcode::OpDefineGlobal(index) => {
                    let name = chunk.read_constant(*index).unwrap().as_string().ok().unwrap();
                    self.globals.insert(name.to_string(), self.stack.peek(0).borrow().clone());
                }



                Opcode::OpGetGlobal(index) => {
                    let name = chunk.read_constant(*index).unwrap().as_string().ok().unwrap();
                    match self.globals.get(name){
                        Some(value) => {
                            self.stack.push(value.borrow().clone());
                        },
                        None => {
                            self.runtime_error(format!("Undefined variable {}", name).as_str());
                            return InterpretResult::RuntimeError;
                        }
                    }
                }
                Opcode::OpSetGlobal(index) => {
                    let name = chunk.read_constant(*index).unwrap().as_string().ok().unwrap();

                    if !self.globals.contains_key(name) {
                        self.runtime_error(format!("Undefined variable {}", name).as_str());
                        return InterpretResult::RuntimeError;
                    } else {
                        let v = self.stack.peek(0).borrow().clone();
                        self.globals.insert(name.to_string(), v);
                    }
                }


                Opcode::OpGetLocal(index) => {
                    let v = self.stack.get(*index).borrow().clone();
                    self.stack.push(v)
                }
                Opcode::OpSetLocal(index) => {
                    self.stack.replace(*index, self.stack.peek(0).borrow().clone());
                }
                Opcode::OpPop => {
                    self.stack.pop();
                }
                Opcode::OpJumpIfFalse(jump) => {

                    if VM::is_falsey(self.stack.peek(0)) {
                        for i in 0..*jump {
                            op_code_iter.next();
                        }
                    }
                }
                Opcode::OpJump(jump) => {
                    for i in 0..*jump {
                        op_code_iter.next();
                    }
                }

                Opcode::OpLoop(offset) => {
                    for i in 0..*jump {
                        op_code_iter.next();
                    }
                }

            }
        }
        InterpretResult::RuntimeError
    }

}


#[cfg(test)]
mod tests {
    use crate::value::{ObjectValue, Value};
    use crate::value::Value::Object;
    use crate::vm::{InterpretResult, VM};


    fn assert_ok(vm: &mut VM, s:&str) {
        match vm.interpret(s) {
            InterpretResult::Ok(val) => {
                println!("Ok {:?}", val);
            }
            InterpretResult::CompileError => {
                panic!("CompileError")
            }
            InterpretResult::RuntimeError => {
                panic!("RuntimeError")
            }
        }
    }

    fn assert_ok_value(vm: &mut VM, s:&str, expected_value: Value) {
        match vm.interpret(s) {
            InterpretResult::Ok(val) => {
                if let Some(r) = val {
                    println!("Ok {}", r);
                    assert_eq!(expected_value, r)
                } else {
                    println!("Ok(empty)");
                }


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

                panic!("Expected RuntimeError, found OK({})", val.unwrap_or(Value::Object(ObjectValue::String("empty".to_string()))))
            }
            InterpretResult::CompileError => {
                panic!("Expected RuntimeError, found CompileError")
            }
            InterpretResult::RuntimeError => {
                println!("RuntimeError")
            }
        }
    }


    fn assert_compile_error(vm: &mut VM, s:&str) {
        match vm.interpret(s) {
            InterpretResult::Ok(val) => {

                panic!("Expected RuntimeError, found OK({})", val.unwrap_or(Value::Object(ObjectValue::String("empty".to_string()))))
            }
            InterpretResult::CompileError => {
                println!("CompileError")

            }
            InterpretResult::RuntimeError => {
                panic!("Expected CompileError found RuntimeError")
            }
        }
    }



    #[test]
    fn vm_multiply() {
        assert_ok_value(&mut VM::new(), "1*2;", Value::Number(2f64))
    }
    #[test]
    fn vm_add() {
        assert_ok_value(&mut VM::new(), "1 + 2;", Value::Number(3f64))
    }
    #[test]
    fn vm_unary() {
        assert_ok_value(&mut VM::new(), "-1;", Value::Number(-1f64));
    }
    #[test]
    fn vm_number() {
        assert_ok_value(&mut VM::new(), "1;", Value::Number(1f64));
    }
    #[test]
    fn vm_grouping() {
        assert_ok_value(&mut VM::new(), "-(1);", Value::Number(-1f64));
    }
    #[test]
    fn vm_minus() {
        assert_ok_value(&mut VM::new(), " 2+5*10;", Value::Number(52f64));
    }

    #[test]
    fn vm_bool_t() {
        assert_ok_value(&mut VM::new(), "true;", Value::Boolean(true));
    }

    #[test]
    fn vm_bool_f() {
        assert_ok_value(&mut VM::new(), "false;", Value::Boolean(false));
    }
    #[test]
    fn vm_bool_not() {
        assert_ok_value(&mut VM::new(), "!false;", Value::Boolean(true));
    }
    #[test]
    fn vm_nil() {
        assert_ok_value(&mut VM::new(), "nil;", Value::Nil);
    }

    #[test]
    fn vm_not_nil() {
        assert_runtime_error(&mut VM::new(),"!nil;");
    }

    #[test]
    fn vm_not_number() {
        assert_runtime_error(&mut VM::new(),"!3.14;");
    }

    #[test]
    fn vm_negate_bool() {
        assert_runtime_error(&mut VM::new(),"-false;");
    }
    #[test]
    fn vm_negate_nil() {
        assert_runtime_error(&mut VM::new(),"-nil;");
    }

    #[test]
    fn vm_greater() {
        assert_ok_value(&mut VM::new(), "2 > 1;", Value::Boolean(true));
        assert_ok_value(&mut VM::new(), "2 >= 1;", Value::Boolean(true));
    }

    #[test]
    fn vm_less() {
        assert_ok_value(&mut VM::new(), "2 < 1;", Value::Boolean(false));
        assert_ok_value(&mut VM::new(), "2 <= 1;", Value::Boolean(false));
    }
    #[test]
    fn vm_equal() {
        assert_ok_value(&mut VM::new(), "2 == 2;", Value::Boolean(true));
    }

    #[test]
    fn vm_equal_fail() {
        assert_ok_value(&mut VM::new(), "2 == 2;", Value::Boolean(true));
    }

    #[test]
    fn vm_str_eval() {
        assert_ok_value(&mut VM::new(), r#""A";"#, Value::Object(ObjectValue::String("A".to_string())));
    }

    #[test]
    fn vm_str_compare() {
        assert_ok_value(&mut VM::new(), r#""A" == "A";"#, Value::Boolean(true));
        assert_ok_value(&mut VM::new(), r#""A" == "B";"#, Value::Boolean(false));
    }


    #[test]
    fn vm_add_str() {
        assert_ok_value(&mut VM::new(), r#""A" + "b";"#, Value::Object(ObjectValue::String("Ab".to_string())));
    }

    #[test]
    fn vm_add_distinct_types() {
        assert_runtime_error(&mut VM::new(),r#""A" + 3.1;"#);
    }

    #[test]
    fn vm_add_distinct_types_2() {
        assert_runtime_error(&mut VM::new(),r#"3.1 +  "A" ;"#);
    }


    #[test]
    fn vm_print_expr() {
        assert_ok_value(&mut VM::new(), "print 1 + 2;", Value::Object(ObjectValue::String("Ab".to_string())));
    }
    #[test]
    fn vm_global_get() {
        assert_ok_value(&mut VM::new(), r#"
        var beverage = "cafe au lait";
        var breakfast = "beignets with " + beverage ;
        print breakfast;
        "#, Value::Object(ObjectValue::String("beignets with cafe au lait".to_string())));
    }

    #[test]
    fn vm_global_set() {
        assert_ok_value(&mut VM::new(), r#"
var beverage  = "cafe au lait";
var breakfast = "beignets";
breakfast = breakfast + " with " +   beverage ;
print breakfast;
breakfast;
        "#, Value::Object(ObjectValue::String("beignets with cafe au lait".to_string())));
    }

    #[test]
    fn vm_local_set_duplicate() {
        assert_compile_error(&mut VM::new(),r#"
{
    var a ="first";
    var a = "second"
}
        "#);
    }


    #[test]
    fn vm_local_set1() {
        assert_ok_value(&mut VM::new(), r#"
{
    var a = "outer";
    {
        var a =  "inner";
    }
}
        "#, Value::Nil);
    }
    #[test]
    fn vm_local_set_2() {
        assert_ok_value(&mut VM::new(), r#"
{
    var a = "outer";
    {
        var b =  "inner";
        var c =  "hi " + b;
        print c;
    }
}
        "#, Value::Nil);
    }

    #[test]
    fn vm_if_stmt() {
        assert_ok(&mut VM::new(), r#"
print "1";
if (false) {
    print "2";
}
if (true) {
    print "3";
}
print "4";
        "#);
    }

    #[test]
    fn vm_if_else_stmt() {
        assert_ok(&mut VM::new(), r#"
print "1";
if (false) {
    print "2";
} else {
    print "3";
}
print "4";
        "#);
    }

    #[test]
    fn vm_logical_and() {
        assert_ok(&mut VM::new(), r#"

var a =  true;
var b =  false;
a and b;


        "#);
    }
    #[test]
    fn vm_logical_or() {
        assert_ok(&mut VM::new(), r#"

var a =  true;
var b =  false;
print a or b;


        "#);
    }

}