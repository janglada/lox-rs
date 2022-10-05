#![feature(backtrace)]
use crate::chunk::ChunkOpCodeReader;
use std::backtrace::Backtrace;

use crate::error::LoxRuntimeError;
use crate::function::ObjectFunction;
use crate::opcode::Opcode;
use crate::parser::Parser;
use crate::stack::Stack;
use crate::value::Value;
use arrayvec::ArrayVec;
use miette::{Error, IntoDiagnostic, Report, Result};
use std::borrow::Borrow;
use std::collections::HashMap;
use std::io;
use std::io::Write;

pub struct CallFrame {
    function: ObjectFunction,
    //  The slots field points into the VM’s value stack at the first slot that this function can use
    value_stack_pos: usize,
}

pub struct VM {
    pub frames: ArrayVec<CallFrame, 64>,
    pub frame_count: usize,
    pub stack: Stack<Value>,
    pub globals: HashMap<String, Value>,
}
pub enum InterpretResult {
    Ok(Option<Value>),
    CompileError,
    RuntimeError,
}

impl VM {
    pub fn new() -> Self {
        VM {
            frames: ArrayVec::<CallFrame, 64>::new(),
            frame_count: 0,
            stack: Stack::with_capacity(256),
            globals: HashMap::new(),
        }
    }

    fn print_value(value: &Value) {
        println!("{}", value)
    }

    pub fn interpret(&mut self, source: &str) -> Result<()> {
        let mut parser = Parser::new(source);

        // let mut function = parser.compile()?;
        // self.frames.push(CallFrame {
        //     function,
        //     ip: 0,
        //     value_stack_pos: 0,
        // });
        // self.frame_count += 1;
        // self.run()
        let compile_result = parser.compile();
        return match compile_result {
            Err(err) => {
                println!("{:?}", err);
                Err(err).into_diagnostic()
            }
            Ok(function) => {
                self.stack.push(Value::Function(function.clone()));
                self.call(function, &0);
                // self.frames.push(CallFrame {
                //     function,
                //     ip: 0,
                //     value_stack_pos: 0,
                // });
                // self.frame_count += 1;
                self.run()
            }
        };

        // if let Err(err) = compile_result {
        //     return Err(err).into_diagnostic();
        // } else if let Ok(function) = compile_result {
        //     // let handle_ptr: *mut ObjectFunction = ;
        //     self.frames.push(CallFrame {
        //         function,
        //         ip: 0,
        //         value_stack_pos: 0,
        //     });
        //     self.frame_count += 1;
        //     return self.run();
        // }

        // let result = match parser.compile() {
        //     Ok(function) => {
        //         // let handle_ptr: *mut ObjectFunction = ;
        //         self.frames.push(CallFrame {
        //             function,
        //             ip: 0,
        //             value_stack_pos: 0,
        //         });
        //         self.frame_count += 1;
        //         self.run()
        //     }
        //     Err(_err) => Err(_err).into_diagnostic(),
        // };
        // result
    }

    fn runtime_error<T>(&mut self, msg: &str) -> Result<T> {
        //  panic!("{}", msg);

        // if let Some(_frame) = self.frames.last() {
        //     // unsafe { (*frame.function).chunk }
        // }
        // eprintln!("Runtime error:{}\n", msg);
        unsafe {
            println!("Custom backtrace: {}", Backtrace::force_capture());
        }
        // Err(Report::new(msg.to_string()))
        Err(LoxRuntimeError::new(msg).into())
    }

    fn wrong_type_error<T>(&mut self, msg: &str) -> Result<T> {
        // eprintln!("Runtime error:{}\n", msg);

        // Err(Report::new(msg.to_string()))
        unsafe {
            println!("Custom backtrace: {}", Backtrace::force_capture());
        }
        Err(Error::msg(msg.to_string()))?
    }

    pub fn pop_operand_as_number(&mut self) -> Result<f64> {
        if !self.stack.peek(0).is_number() {
            return self.wrong_type_error("Operand must be numbers");
            //  Err(LoxRuntimeError::new("Operand must be numbers"))?;
        }
        //let p = self.stack.peek(0).is_number();
        Ok(*self.stack.pop().as_number().unwrap())
    }

    pub fn unchecked_pop_operand_as_numbers(&mut self) -> Result<(f64, f64)> {
        let b = *self.stack.pop().as_number().unwrap();
        let a = *self.stack.pop().as_number().unwrap();
        Ok((a, b))
    }

    pub fn pop_operand_as_numbers(&mut self) -> Result<(f64, f64)> {
        let op1 = self.stack.peek(0);
        let op2 = self.stack.peek(1);
        if !op1.is_number() || !op2.is_number() {
            return self.wrong_type_error("Operand must be numbers");
        }
        let b = *self.stack.pop().as_number().unwrap();
        let a = *self.stack.pop().as_number().unwrap();
        Ok((a, b))
    }

    pub fn unchecked_pop_operand_as_strings(&mut self) -> Result<(String, String)> {
        let b = self.stack.pop().as_string().unwrap().clone();
        let a = self.stack.pop().as_string().unwrap().clone();
        Ok((a, b))
    }

    pub fn pop_operand_as_strings(&mut self) -> Result<(String, String)> {
        if !self.stack.peek(0).is_string() || !self.stack.peek(1).is_string() {
            return self.wrong_type_error("Operand must be string");
        }
        let b = self.stack.pop().as_string().unwrap().clone();
        let a = self.stack.pop().as_string().unwrap().clone();
        Ok((a, b))
    }
    pub fn pop_operand_as_bool(&mut self) -> Result<bool> {
        if !self.stack.peek(0).is_bool() {
            return self.wrong_type_error("Operands must be boolean");

            // return Err(LoxRuntimeError::new())?;
        }
        let b = *self.stack.pop().as_bool().unwrap();

        Ok(b)
    }

    fn is_falsey(v: &Value) -> bool {
        match v {
            Value::Nil => true,
            Value::Boolean(b) => !b,
            _ => false,
        }
    }

    // fn peek_and_call_value(&mut self, peek_pos: usize, arg_count: &u8) -> bool {
    //     let v = self.stack.peek_mut(peek_pos);
    //     self.call_value(v, arg_count)
    // }

    fn call_value(&mut self, peek_pos: usize, arg_count: &u8) -> Result<bool> {
        // let callee1 = self.stack.peek_mut(peek_pos - 1);
        let callee = self.stack.peek_mut(peek_pos);
        if callee.is_object() {
            // match callee {
            //     Value::Function(func) => {
            //         //  let f = unsafe { &mut (*(*func)) };
            //
            //         return self.call(func, arg_count);
            //     }
            //     _ => {
            //         todo!();
            //     }
            // }
            if let Ok(mut func) = callee.as_function() {
                return self.call(&mut func, arg_count);
            } else {
                return self.runtime_error("Non callable object");
            };
        }

        self.runtime_error("Can only call functions and classes")
    }

    ///
    ///
    ///
    fn call(&mut self, function: &mut ObjectFunction, arg_count: &u8) -> Result<bool> {
        if *arg_count != function.arity {
            return self.runtime_error(
                format!(
                    "Expected {} arguments, but got {}",
                    function.arity, arg_count
                )
                .as_str(),
            );
        }
        let p = self.stack.len() - *arg_count as usize - 1;
        self.frames.push(CallFrame {
            function: function.clone(),
            value_stack_pos: p,
        });

        Ok(true)
    }
    ///
    ///
    pub fn run(&mut self) -> Result<()> {
        // let mut frame = &mut self.frames[self.frame_count - 1];
        let mut frame = self.frames.last_mut().unwrap();
        let frame_slot = frame.value_stack_pos;
        // let frame = frames_opt.last().unwrap();
        let mut chunk = frame.function.chunk.clone(); //unsafe { (*frame.function).chunk.clone() }; // unsafe { &(*frame.function).chunk };
                                                      // for c in &chunk.op_codes
        let mut op_code_iter = ChunkOpCodeReader::new(chunk.op_codes.as_slice());
        // let mut op_code_iter = chunk.op_codes.iter();
        while let Some((_ip, c)) = op_code_iter.next() {
            let a = c.clone();
            println!("OP CODE {:?}", a);
            io::stdout().flush().unwrap();
            match c {
                Opcode::OpConstant(idx) => {
                    let const_val = chunk.read_constant(*idx).unwrap();
                    self.stack.push(const_val.borrow().clone());
                    // println!("const val {}", const_val);
                }

                Opcode::OpNegate => {
                    let r = self.pop_operand_as_number().map(|f| {
                        self.stack.push(Value::Number(-f));
                    });
                    if r.is_err() {
                        return r;
                    }

                    // match self.pop_operand_as_number() {
                    //     Ok(f) => {
                    //         // VM::replace(&mut self.stack, &mut Value::Number(-f));
                    //         self.stack.push(Value::Number(-f));
                    //     }
                    //     Err(result) => {
                    //         self.runtime_error("Operand must be a number");
                    //         return result;
                    //     }
                    // }
                }

                // Opcode::OpAdd => match self.pop_operand_as_numbers() {
                //     Ok((a, b)) => self.stack.push(Value::Number(a + b)),
                //     _ => match self.pop_operand_as_strings() {
                //         Ok((a, b)) => self.stack.push(Value::String(format!("{}{}", a, b))),
                //         _ => {
                //             return self.wrong_type_error("Operands must be of same type");
                //             // return Err(LoxRuntimeError::new().into());
                //         }
                //     },
                // }, //        let op1 = self.stack.peek(0);
                // let op2 = self.stack.peek(1);
                Opcode::OpAdd => {
                    let op1 = self.stack.peek(0);
                    let op2 = self.stack.peek(1);
                    if op1.is_number() && op2.is_number() {
                        match self.unchecked_pop_operand_as_numbers() {
                            Ok((a, b)) => self.stack.push(Value::Number(a + b)),
                            Err(err) => {
                                return Err(err);
                            }
                        }
                    } else if op1.is_string() && op2.is_string() {
                        match self.unchecked_pop_operand_as_strings() {
                            Ok((a, b)) => self.stack.push(Value::String(format!("{}{}", a, b))),
                            Err(err) => {
                                return Err(err);
                            }
                        }
                    } else {
                        return self.wrong_type_error(
                            format!("Addition operation requires operands must be of same type, either number or string. Found operand #1 = {}, operand #2 = {}", op1, op2).as_str());
                    }
                }

                Opcode::OPSubtract => match self.pop_operand_as_numbers() {
                    Ok((a, b)) => self.stack.push(Value::Number(a - b)),
                    Err(result) => return Err(result),
                },

                Opcode::OPMultiply => match self.pop_operand_as_numbers() {
                    Ok((a, b)) => self.stack.push(Value::Number(a * b)),
                    Err(result) => return Err(result),
                },

                Opcode::OpDivide => match self.pop_operand_as_numbers() {
                    Ok((a, b)) => self.stack.push(Value::Number(a / b)),
                    Err(result) => return Err(result),
                },
                Opcode::OpFalse => self.stack.push(Value::Boolean(false)),
                Opcode::OpNil => self.stack.push(Value::Nil),
                Opcode::OpTrue => self.stack.push(Value::Boolean(true)),

                Opcode::OpNot => match self.pop_operand_as_bool() {
                    Ok(bool_operand) => self.stack.push(Value::Boolean(!bool_operand)),
                    Err(result) => return Err(result),
                },

                Opcode::OpEqual => {
                    let b = self.stack.pop();
                    let a = self.stack.pop();
                    self.stack.push(Value::Boolean(a == b))
                }

                Opcode::OpGreater => match self.pop_operand_as_numbers() {
                    Ok((a, b)) => self.stack.push(Value::Boolean(a > b)),
                    Err(result) => return Err(result),
                },

                Opcode::OpLess => match self.pop_operand_as_numbers() {
                    Ok((a, b)) => self.stack.push(Value::Boolean(a < b)),
                    Err(result) => return Err(result),
                },

                Opcode::OpPrint => {
                    println!("{}", self.stack.pop());
                }

                Opcode::OpDefineGlobal(index) => {
                    let name = chunk
                        .read_constant(*index)
                        .unwrap()
                        .as_string()
                        .ok()
                        .unwrap();

                    self.globals
                        .insert(name.to_string(), self.stack.peek(0).borrow().clone());
                    self.stack.pop();
                }

                Opcode::OpGetGlobal(index) => {
                    let name = chunk
                        .read_constant(*index)
                        .unwrap()
                        .as_string()
                        .ok()
                        .unwrap();

                    match self.globals.get(name) {
                        Some(value) => {
                            self.stack.push(value.borrow().clone());
                        }
                        None => {
                            return self
                                .runtime_error(format!("Undefined variable {}", name).as_str());
                            // return Err(LoxRuntimeError::new().into());
                        }
                    }
                }
                Opcode::OpSetGlobal(index) => {
                    let name = chunk
                        .read_constant(*index)
                        .unwrap()
                        .as_string()
                        .ok()
                        .unwrap();

                    if !self.globals.contains_key(name) {
                        return self.runtime_error(format!("Undefined variable {}", name).as_str());
                        // return Err(LoxRuntimeError::new().into());
                    } else {
                        let v = self.stack.peek(0).clone();
                        self.globals.insert(name.to_string(), v);
                    }
                }

                Opcode::OpGetLocal(index) => {
                    println!(
                        "STACK GET INDEX {}, STACK LEN {}",
                        *index + frame_slot,
                        self.stack.len()
                    );

                    let v = self.stack.get(*index + frame_slot).clone();
                    // self.stack.push(v);
                    self.stack.push(v);
                }
                Opcode::OpSetLocal(index) => {
                    self.stack
                        .replace(*index + frame_slot, self.stack.peek(0).clone());
                }
                Opcode::OpPop => {
                    self.stack.pop();
                }
                Opcode::OpJumpIfFalse(jump) => {
                    if VM::is_falsey(self.stack.peek(0)) {
                        for _i in 0..*jump {
                            op_code_iter.next();
                        }
                    }
                }
                Opcode::OpJump(jump) => {
                    for _i in 0..*jump {
                        op_code_iter.next();
                    }
                }

                Opcode::OpLoop(offset) => {
                    for _i in 0..*offset {
                        op_code_iter.prev();
                    }
                }

                Opcode::OpCall(num_args) => {
                    // let mut v = self.stack.peek_mut((*num_args) as usize);
                    match self.call_value((*num_args) as usize, num_args) {
                        Ok(success) => {
                            if success {
                                frame = self.frames.last_mut().unwrap();
                                chunk = frame.function.chunk.clone(); //unsafe { (*frame.function).chunk.clone() }; //unsafe { &(*frame.function).chunk };
                                                                      // for c in &chunk.op_codes
                                op_code_iter = ChunkOpCodeReader::new(chunk.op_codes.as_slice());
                            } else {
                                return self.runtime_error("Could not call function");
                            }
                        }
                        Err(err) => {
                            return Err(err);
                        }
                    }
                }

                // Opcode::OpReturn => {
                //     return if let Some(ret_val) = self.stack.safe_pop() {
                //         VM::print_value(&ret_val);
                //         InterpretResult::Ok(Some(ret_val.borrow().clone()))
                //     } else {
                //         InterpretResult::Ok(None)
                //     }
                // }
                Opcode::OpReturn => {
                    let _result: Value = self.stack.pop();
                    self.frames.pop();
                    if self.frames.is_empty() {
                        self.stack.pop();
                        return Ok(());
                    }

                    frame = self.frames.last_mut().unwrap();
                    chunk = frame.function.chunk.clone(); // unsafe { &(*frame.function).chunk };
                                                          // for c in &chunk.op_codes
                    op_code_iter = ChunkOpCodeReader::new(chunk.op_codes.as_slice());
                }
            }
        }
        return Err(LoxRuntimeError::new("end program"))?;
    }
}
