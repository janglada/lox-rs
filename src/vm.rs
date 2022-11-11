use crate::chunk::ChunkOpCodeReader;
use std::backtrace::Backtrace;

use crate::error::{LoxCompileError, LoxRuntimeError};
use crate::function::ObjectFunction;
use crate::native::{NativeFn, ObjectNative};
use crate::opcode::Opcode;
use crate::parser::Parser;
use crate::stack::Stack;
use crate::value::Value;
use crate::value::Value::Number;
use crate::vm::CallResponse::{Native, Standard};
use arrayvec::ArrayVec;
use miette::{miette, Error, IntoDiagnostic, NamedSource, Result};
use std::borrow::Borrow;
use std::collections::HashMap;
use std::time::{SystemTime, UNIX_EPOCH};

pub struct CallFrame {
    function: ObjectFunction,
    //  The slots field points into the VM’s value stack at the first slot that this function can use
    value_stack_pos: usize,
    return_address_pos: usize,
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

pub enum CallResponse {
    Standard(bool),
    Native,
}

impl VM {
    pub fn new() -> Self {
        let mut vm = VM {
            frames: ArrayVec::<CallFrame, 64>::new(),
            frame_count: 0,
            stack: Stack::with_capacity(256),
            globals: HashMap::new(),
        };
        vm.define_native("clock".to_string(), |_a, _b| {
            let now = SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_millis();
            return Number(now as f64);
        });

        vm.define_native("sin".to_string(), |_a, b| unsafe {
            let arg = *b.as_ref().unwrap().as_number().unwrap();
            return Number(arg.sin());
        });

        vm
    }

    fn print_value(value: &Value) {
        println!("{}", value)
    }

    fn define_native(&mut self, name: String, function: NativeFn) {
        // self.stack.push(Value::String(name.clone()));
        // self.stack.push(Value::NativeFunction(ObjectNative::new(
        //     name.clone(),
        //     function,
        // )));
        self.globals.insert(
            name.clone(),
            Value::NativeFunction(ObjectNative::new(name.clone(), function)),
        );
    }

    pub fn interpret(&mut self, source: &str) -> Result<Option<Value>> {
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
                //    println!("{:?}", err);
                Err(LoxCompileError {
                    src: NamedSource::new("bad_file.rs", parser.scanner.get_input()),
                    bad_bit: (err.line as usize, err.start as usize).into(),
                    label: err.msg,
                })
                .into_diagnostic()
            }
            Ok(function) => {
                // println!("The origin is: {function:?}");
                //write!(stdout(), s.to_string());
                // std::io::stdout().flush();

                // write!(stdout(), "CALLING FUNCTION {}\n", function.name);
                // function.disassemble_chunk(&mut (Box::new(io::stdout()) as Box<dyn Write>));

                self.stack.push(Value::Function(function.clone()));
                let _ = self.call(function, &0, 0);
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

        // unsafe {
        Err(Error::msg(
            format!("{}\n\n{}", msg.to_string(), Backtrace::force_capture()).to_string(),
        ))?
        // }
    }

    fn wrong_type_error<T>(&mut self, msg: &str) -> Result<T> {
        // eprintln!("Runtime error:{}\n", msg);

        // Err(Report::new(msg.to_string()))
        // unsafe {
        //     println!("Custom backtrace: {}", Backtrace::force_capture());
        // }

        Err(Error::msg(
            format!("{}\n\n{}", msg.to_string(), Backtrace::force_capture()).to_string(),
        ))?
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
            return self.wrong_type_error(
                format!(
                    "Operand must be numbers, found operand #1 = {:?}, #2 = {:?}",
                    op1.to_string(),
                    op2.to_string()
                )
                .as_str(),
            );
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

    pub fn unchecked_pop_operand_as_string_and_number(&mut self) -> Result<(f64, String)> {
        let b = self.stack.pop().as_string().unwrap().clone();
        let a = self.stack.pop().as_number().unwrap().clone();
        Ok((a, b))
    }

    pub fn unchecked_pop_operand_as_number_and_string(&mut self) -> Result<(String, f64)> {
        let b = self.stack.pop().as_number().unwrap().clone();
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

    fn call_value(&mut self, arity: &u8, opcode_pos: usize) -> Result<CallResponse> {
        // let callee1 = self.stack.peek_mut(peek_pos - 1);
        let callee = self.stack.peek_mut(*arity as usize);

        // println!(
        //     "peek_pos {}, arg_count {}, calle {}",
        //     peek_pos,
        //     arg_count,
        //     callee.to_string()
        // );
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
                return Ok(CallResponse::Standard(
                    self.call(&mut func, arity, opcode_pos).unwrap(),
                ));
            }
            if let Ok(native) = callee.as_native() {
                unsafe {
                    let fn_native = native.function;
                    let _ptr0 = self.stack.as_ptr();
                    let ptr = self.stack.as_ptr().offset(-((*arity) as isize));
                    let result = fn_native(*arity, ptr);
                    self.stack.pop_n(arity + 1);

                    self.stack.push(result);
                    return Ok(CallResponse::Native);
                }
            }
        }

        self.runtime_error("Can only call functions and classes")
    }

    ///
    ///
    ///
    fn call(
        &mut self,
        function: &mut ObjectFunction,
        arg_count: &u8,
        opcode_pos: usize,
    ) -> Result<bool> {
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
        //   println!("value_stack_pos {}", p);
        self.frames.push(CallFrame {
            function: function.clone(),
            value_stack_pos: p,
            return_address_pos: opcode_pos,
        });

        Ok(true)
    }
    ///
    ///
    pub fn run(&mut self) -> Result<Option<Value>> {
        // let mut frame = &mut self.frames[self.frame_count - 1];
        let mut frame = self.frames.last_mut().unwrap();
        let mut frame_slot = frame.value_stack_pos;
        // let frame = frames_opt.last().unwrap();
        let mut chunk = frame.function.chunk.clone(); //unsafe { (*frame.function).chunk.clone() }; // unsafe { &(*frame.function).chunk };
                                                      // for c in &chunk.op_codes
        let mut op_code_iter = ChunkOpCodeReader::new(chunk.op_codes.as_slice(), 0);

        let _counter = 0;
        // let mut op_code_iter = chunk.op_codes.iter();
        while let Some((_ip, c)) = op_code_iter.next() {
            //  let _a = c.clone();

            //write!(stdout(), "OP CODE {:?}\n", a);

            // if counter > 50 {
            //     panic!();
            // }
            // counter = counter + 1;

            // println!("OP CODE {:?}", a);
            //  io::stdout().flush().unwrap();
            match c {
                Opcode::OpConstant(idx) => {
                    let const_val = chunk.read_constant(*idx).unwrap();
                    self.stack.push(const_val.borrow().clone());
                    // println!("const val {}", const_val);
                }

                Opcode::OpNegate => {
                    let r = self.pop_operand_as_number().map(|f| {
                        self.stack.push(Value::Number(-f));
                        Some(Value::Number(-f))
                    });
                    if r.is_err() {
                        return r;
                    }
                }

                Opcode::OpAdd => {
                    // println!("ADD ---------------------------------------");
                    // println!("{:?}", self.stack);

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
                    } else if op1.is_string() && op2.is_number() {
                        match self.unchecked_pop_operand_as_string_and_number() {
                            Ok((a, b)) => self.stack.push(Value::String(format!("{}{}", a, b))),
                            Err(err) => {
                                return Err(err);
                            }
                        }
                    } else if op1.is_number() && op2.is_string() {
                        match self.unchecked_pop_operand_as_number_and_string() {
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
                    // println!(
                    //     "STACK GET INDEX {}, STACK LEN {}",
                    //     *index + frame_slot,
                    //     self.stack.len()
                    // );

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
                        op_code_iter.jump(*jump);
                    }
                }
                Opcode::OpJump(jump) => {
                    op_code_iter.jump(*jump);
                }

                Opcode::OpLoop(offset) => {
                    op_code_iter.prev(*offset);
                }

                Opcode::OpCall(arity) => {
                    // println!("OPCALL {}", _ip);
                    // let mut v = self.stack.peek_mut((*num_args) as usize);

                    match self.call_value(arity, _ip).unwrap() {
                        Standard(success) => {
                            if success {
                                frame = self.frames.last_mut().unwrap();
                                chunk = frame.function.chunk.clone(); //unsafe { (*frame.function).chunk.clone() }; //unsafe { &(*frame.function).chunk };
                                frame_slot = frame.value_stack_pos; // for c in &chunk.op_codes
                                op_code_iter = ChunkOpCodeReader::new(chunk.op_codes.as_slice(), 0);
                            } else {
                                return self.runtime_error("Could not call function");
                            }
                        }
                        Native => {
                            // return Err(err);
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
                    // println!("RETURN ---------------------------------------");
                    // println!("{:?}", self.stack);

                    let _result: Value = self.stack.pop();
                    let last_frame = self.frames.pop().expect("no frame");

                    self.stack.truncate(last_frame.value_stack_pos);

                    if self.frames.is_empty() {
                        //let _ = self.stack.pop();
                        return Ok(Some(_result));
                    }
                    self.stack.push(_result);
                    frame = self.frames.last_mut().unwrap();
                    chunk = frame.function.chunk.clone();

                    op_code_iter = ChunkOpCodeReader::new(
                        chunk.op_codes.as_slice(),
                        last_frame.return_address_pos,
                    );
                }
            }
        }
        let _a = self.stack.peek(0);
        return Err(LoxRuntimeError::new("end program"))?;
    }
}
