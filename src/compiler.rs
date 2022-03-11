use std::borrow::BorrowMut;
use std::fs::File;
use std::io;
use std::io::Write;
use std::ops::{Add, AddAssign, SubAssign};
use std::path::Path;
use num_traits::FromPrimitive;
use crate::chunk::{Chunk};
use crate::opcode::Opcode;
use crate::opcode::Opcode::OpJumpIfFalse;
use crate::parser::Parser;
use crate::precedence::{ParserRule, Precedence};
use crate::scanner::Scanner;
use crate::token::{Token, TokenType};
use crate::value::{ObjectValue, Value};



#[derive(Debug)]
pub struct Compiler<'a> {
    scanner: Scanner<'a>,
    parser: Parser<'a>,
    writer: ChunkWriter<'a>,
    // scope
    locals: Vec<Local>,
    local_count: usize,
    scope_depth:isize
}
#[derive(Debug,  Clone)]
pub struct Local{
    token: Token, // clone!!! noooo, just a ref..
    depth: isize
}



///
///
#[derive(Debug)]
pub struct ChunkWriter<'a> {
    chunk: &'a mut Chunk
}



impl<'a>  ChunkWriter<'a> {

    fn new(chunk : &'a mut Chunk) ->  Self {
        ChunkWriter {
            chunk
        }
    }

    ///
    ///
    fn emit_byte(&mut self, byte: Opcode, line: isize) {
        self.write_chunk( byte, line);
    }


    ///
    ///
    fn emit_bytes(&mut self, byte1: Opcode, byte2: Opcode, line: isize) {
        self.emit_byte(byte1, line);
        self.emit_byte(byte2, line);
    }

    ///
    ///
    fn emit_return(&mut self, line: isize) {
        self.emit_byte(Opcode::OpReturn, line)
    }
    ///
    ///
    fn emit_constant(&mut self, value: Value, line: isize) {
        let idx = self.make_constant(value);
        self.emit_byte(Opcode::OpConstant(idx), line)
    }


    fn write_chunk(&mut self, byte: Opcode, _line: isize) {
        self.chunk.write_chunk(byte);
    }

    fn make_constant(&mut self, value: Value) -> usize {
        self.chunk.add_constant(value)
    }

    pub fn disassemble_chunk(&mut self, writer: &mut Box<dyn Write>) {
        self.chunk.disassemble_chunk(writer)
    }
}



impl<'a> Compiler<'a> {

    pub fn new(source:  &'a str, chunk: &'a mut Chunk) -> Self {

        const INIT: Option<Local> = None;
        Compiler {
            scanner: Scanner::new(source),
            parser:Parser::new(),
            writer: ChunkWriter::new(chunk),
            locals:Vec::with_capacity(256),
            local_count : 0 ,
            scope_depth: 0
        }
    }

    fn advance(&mut self) {
        self.parser.previous = self.parser.current.clone();
        loop {
            self.parser.current = self.scanner.scan_token();
            match self.parser.current.token_type  {
                TokenType::Error => {
                    println!("ERROR")
                },
                _ => {

                    break;
                }
            }
        }
    }

    ///
    ///
    pub fn compile(&mut self) -> bool {
        self.parser.result =Ok(());
        self.parser.panic_mode = false;
        self.advance();
        // self.expression();
        // self.consume(TokenType::EOF, "Expect end of expression");

        while !self.match_token(TokenType::EOF){
            self.declaration();
        }

        self.end_compiler();
        self.parser.result.is_ok()
    }

    ///
    ///
    ///
    fn declaration(&mut self) {

        if self.match_token(TokenType::Var) {
            self.var_declaration()
        } else {
            self.statement();
        }
        if self.parser.panic_mode {
            self.synchronize();
        }
    }

    fn var_declaration(&mut self) {
        let index = self.parse_variable("Expect variable name");
        if self.match_token(TokenType::Equal) {
            self.expression()
        } else {
            self.writer.emit_byte(Opcode::OpNil,   self.parser.previous.line);
        }

        self.consume(TokenType::SemiColon, "Expect ';' after value");

        self.define_variable(index);

    }

    fn parse_variable(&mut self, msg: &'a str) -> usize{
        self.consume(TokenType::Identifier("".to_string()), msg);

        self.declare_variable();
        if self.scope_depth > 0 {
            return 0;
        }

        self.identifier_constant()

    }
    fn identifier_constant(&mut self) -> usize{
        match &self.parser.previous.token_type {
            TokenType::Identifier(name) => {
                self.writer.make_constant(Value::Object(ObjectValue::String(name.to_string())))
            }
            _ => panic!("should not happen")
        }

    }
    fn resolve_local(&mut self) -> Option<usize> {
        let token = &self.parser.previous;
        let local = self.locals.iter().enumerate().rev()
            .find(|(i, l)| Compiler::identifiers_equal(&token, &l.token));

        // match local {
        //     None => {
        //         None
        //     }
        //     Some((idx, l)) => {
        //         if l.depth == -1 {
        //             self.error("Can't read local variable in its own initializer")
        //         }
        //         Some(idx)
        //     }
        // }
        if let Some((i,l)) = local {
            if l.depth == -1 {
                self.error("Can't read local variable in its own initializer")
            }
            Some(i)
        } else {
            None
        }

       // local.map(|a| a.0)
    }

    fn define_variable(&mut self, index: usize) {
        if self.scope_depth > 0 {
            self.mark_initialized();
            return;
        }
        self.writer.emit_byte(Opcode::OpDefineGlobal(index),   self.parser.previous.line)

    }
    fn declare_variable(&mut self) {
        if self.scope_depth == 0 {
            return;
        }
        let token = self.parser.previous.clone();
        // it's an error to have two variables with the same name in the same local scope

        if let Some(_) =self.locals.iter_mut().rev().take_while(|l| !(l.depth != -1 && l.depth < self.scope_depth))
            .find(|l| Compiler::identifiers_equal(&token, &l.token)) {
            self.error("Already a variable with this name in this scope")
        }
        /*
        for l in self.locals.iter_mut().rev() {
            if l.depth != -1 && l.depth < self.scope_depth {
                break;
            }

            if Compiler::identifiers_equal(&token, &l.token) {
                self.error("Already a variable with this name in this scope")
            }
        }

         */


        
        self.add_local(Local {
            token: token,
            depth: -1,//self.scope_depth
        });
      //  self.parser.previous.
      //  self.writer.emit_byte(Opcode::OpDefineGlobal(index),   self.parser.previous.line)

    }

    fn mark_initialized(&mut self) {
        self.locals.get_mut(self.local_count-1).unwrap().depth = self.scope_depth;
    }

    fn identifiers_equal(token1: &Token, token2: &Token) -> bool {
        if token1.len != token2.len {
            false
        } else {
            match &token1.token_type {
                TokenType::Identifier(name1) =>{
                    match &token2.token_type {
                        TokenType::Identifier(name2) =>{
                            return name1 == name2
                        }
                        _ => return false
                    }
                }
                _ => return false
            }

            false
        }

    }

    fn add_local(&mut self, local: Local) {
        if self.local_count == 256 {
            self.error("Too many local variables in function");
            return;
        }
        self.local_count.add_assign(1);


        self.locals.push(local);

        // self.locals[self.local_count] = local;

        // std::mem::replace(&mut self.locals[self.local_count], local);
    }



    ///
    ///
    ///
    fn statement(&mut self) {
        if self.match_token(TokenType::Print) {
            self.print_statement()
        } else if self.match_token(TokenType::If) {
            self.if_statement()
        } else if self.match_token(TokenType::While) {
            self.while_statement()
        } else if self.match_token(TokenType::LeftBrace) {
           self.begin_scope();
           self.block();
           self.end_scope();
        } else {
           self.expression_statement()
        }
    }

    fn begin_scope(&mut self) {
        self.scope_depth.add_assign(1)
    }

    fn end_scope(&mut self) {
        self.scope_depth.sub_assign(1);

        while self.local_count > 0 && self.locals.get(self.local_count-1).unwrap().depth > self.scope_depth {
            self.writer.emit_byte(Opcode::OpPop,   self.parser.previous.line);
            self.local_count.sub_assign(1);
        }


    }

    fn block(&mut self) {
        while !self.check(TokenType::RightBrace) && !self.check(TokenType::EOF) {
            self.declaration();
        }
        self.consume(TokenType::RightBrace, "Expect ')' after block");
    }


    ///
    ///
    ///
    fn match_token(&mut self, token_type: TokenType) -> bool {
        if !self.check(token_type) {
            return false
        }
        self.advance();
        true

    }


    ///
    ///
    ///
    fn check(&mut self, token_type: TokenType) -> bool {
        self.parser.current.token_type == token_type
    }

    ///
    ///
    ///
    fn print_statement(&mut self)  {
        self.expression();
        self.consume(TokenType::SemiColon, "Expect ';' after value");
        self.writer.emit_byte(Opcode::OpPrint,   self.parser.previous.line);
    }

    ///
    ///
    ///
    fn if_statement(&mut self)  {
        self.consume(TokenType::LeftParen, "Expect '(' after if");
        self.expression();
        self.consume(TokenType::RightParen, "Expect ')' after if condition");


        let then_jump =  self.emit_jump(Opcode::OpJumpIfFalse(0));
        self.writer.emit_byte(Opcode::OpPop, self.parser.previous.line);
        self.statement();
        let else_jump =  self.emit_jump(Opcode::OpJump(0));


        self.patch_jump(then_jump, &Opcode::OpJumpIfFalse(0));
        self.writer.emit_byte(Opcode::OpPop, self.parser.previous.line);

        if self.match_token(TokenType::Else) {
            self.statement();
        }
        self.patch_jump(else_jump, &Opcode::OpJump(0));

    }

    fn while_statement(&mut self) {
        let loop_start = self.writer.chunk.op_codes.len();
        self.consume(TokenType::LeftParen, "Expect '(' after while");
        self.expression();
        self.consume(TokenType::RightParen, "Expect ')' after while condition");


        let exit_jump =  self.emit_jump(Opcode::OpJumpIfFalse(0));
        self.writer.emit_byte(Opcode::OpPop, self.parser.previous.line);
        self.statement();
        self.emit_loop(loop_start);

        self.patch_jump(exit_jump, &Opcode::OpJumpIfFalse(0));
        self.writer.emit_byte(Opcode::OpPop, self.parser.previous.line);


    }

    ///
    ///
    ///
    fn emit_jump(&mut self, opcode: Opcode) -> usize {
        self.writer.emit_byte(opcode,    self.parser.previous.line);
        self.writer.chunk.op_codes.len()
    }
    fn emit_loop(&mut self, loop_start: usize)  {
        self.writer.emit_byte(Opcode::OpLoop(0), self.parser.previous.line);
        let len = self.writer.chunk.op_codes.len();
        let offset = len - loop_start;
        if offset > u16::MAX as usize{
            self.error("Loop body too large");
        }
        self.writer.chunk.replace_opcode(len-1, Opcode::OpLoop(offset as u16));

    }
    ///
    ///
    ///
    fn patch_jump(&mut self, offset: usize, opcode: &Opcode)  {
        let jump = self.writer.chunk.op_codes.len() - offset;
        if jump > u16::MAX as usize{
            self.error("Too much code to jump over");
        }

        let patched_opcode =  match opcode {
            Opcode::OpJumpIfFalse(_) => {
                Opcode::OpJumpIfFalse(jump as u16)
            }
            Opcode::OpJump(_) => {
                Opcode::OpJump(jump as u16)
            }
            _ => {
                panic!("Not a jumpable opcode")
            }
        };

        self.writer.chunk.replace_opcode(offset-1, patched_opcode);

    }
    ///
    ///
    ///
    fn expression_statement(&mut self)  {
        self.expression();
        self.consume(TokenType::SemiColon, "Expect ';' after value");
        self.writer.emit_byte(Opcode::OpPop,   self.parser.previous.line);
    }


    ///
    ///
    ///
    fn synchronize(&mut self) {
        self.parser.panic_mode = false;
        if self.parser.previous.token_type == TokenType::SemiColon {
            return;
        }
        match  self.parser.current.token_type {
            TokenType::Class | TokenType::Fun | TokenType::Var |
            TokenType::For | TokenType::If | TokenType::While |
            TokenType::Print | TokenType::Return  => {
                    return
            }
            _ => self.advance()
        }
    }
    ///
    ///
    ///
    pub fn consume(&mut self, token_type: TokenType, message: &'a str) {
        if self.parser.current.token_type == token_type {
            self.advance();
            return;
        }
        self.error_at_current(message);
    }


    ///
    ///
    fn end_compiler(&mut self, ) {
        self.writer.emit_return(self.parser.previous.line);
        if let Ok(_res) =  self.parser.result {
            self.writer.disassemble_chunk(&mut (Box::new(io::stdout()) as Box<dyn Write>));
        }
    }


    ///
    ///
    fn expression(&mut self) {
        self.parse_precedence(&Precedence::Assigment)
    }


    fn parse_precedence(&mut self, precedence: &Precedence) {
        self.advance();
        let prefix_rule = ParserRule::get_rule(&self.parser.previous.token_type).prefix;

        let can_assign = *precedence <= Precedence::Assigment;
        if prefix_rule.is_none() {
            self.error("Expect expression")
        } else {
            prefix_rule.unwrap()(self, can_assign);
        }

        while precedence <= ParserRule::get_rule(&self.parser.current.token_type).precedence
        {
            self.advance();
            let infix_rule = ParserRule::get_rule(&self.parser.previous.token_type).infix;
            if infix_rule.is_some() {
                infix_rule.unwrap()(self, can_assign);
            }
        }

        if can_assign && self.match_token(TokenType::Equal) {
            self.error("Invalid assignment target")
        }
    }


    ///
    ///
    fn error_at_current(&mut self, msg: &'a str) {
        self.error_at(&self.parser.current.clone(), msg);
    }

    ///
    ///
    fn error(&mut self,  msg: &'a str) {
        self.error_at(&mut self.parser.previous.clone(), msg);
    }

    ///
    ///
    fn error_at(&mut self, token: &Token, msg: &'a str) {
        if self.parser.panic_mode {
            return;
        }
        self.parser.panic_mode = true;
        eprint!(" [line {}] Error", token.line);
        match token.token_type {
            TokenType::EOF =>  eprint!(" at end"),
            TokenType::Error =>  {

            },
            _ => {
                eprint!(" at {} {}", token.len, token.start);
            }
        }
        eprintln!(": {}", msg);
        self.parser.result = Err(msg);
    }
}


pub fn number(compiler: &mut Compiler, can_assign: bool) {
    match &compiler.parser.previous.token_type {
        TokenType::Number(num) => {
            compiler.writer.emit_constant(Value::Number(*num), compiler.parser.previous.line)
        }
        _ => panic!("unexpected token type")
    }
}


pub fn string(compiler: &mut Compiler, can_assign: bool) {
    match &compiler.parser.previous.token_type {
        TokenType::String(str) => {
            dbg!(str);
            compiler.writer.emit_constant(Value::new_string(str), compiler.parser.previous.line)
        }
        _ => panic!("unexpected token type")
    }
}

///
///
pub fn variable(compiler: &mut Compiler, can_assign: bool) {
    named_variable(compiler, can_assign);
}

///
///
pub fn named_variable(compiler: &mut Compiler, can_assign: bool) {
    let index = compiler.identifier_constant();

    let (get_op, set_op) = if let Some(index) = compiler.resolve_local()  {
        (Opcode::OpGetLocal(index), Opcode::OpSetLocal(index))
    } else {
        let index = compiler.identifier_constant();
        (Opcode::OpGetGlobal(index), Opcode::OpSetGlobal(index))
    };

    if can_assign && compiler.match_token(TokenType::Equal) {
        compiler.expression();
        compiler.writer.emit_byte(set_op, compiler.parser.previous.line)
    } else {
        compiler.writer.emit_byte(get_op, compiler.parser.previous.line)
    }
}



///
///
pub fn grouping(compiler: &mut Compiler, can_assign: bool) {
    compiler.expression();
    compiler.consume(TokenType::RightParen, "Expected ')' after expression")
}


///
///
pub fn literal(compiler: &mut Compiler, can_assign: bool) {
    let token_type = &compiler.parser.previous.token_type.clone();
    match token_type {
       TokenType::False => compiler.writer.emit_byte(Opcode::OpFalse,  compiler.parser.previous.line),
       TokenType::Nil => compiler.writer.emit_byte(Opcode::OpNil,  compiler.parser.previous.line),
       TokenType::True => compiler.writer.emit_byte(Opcode::OpTrue,  compiler.parser.previous.line),
        _ => {}
    }
}

///
///
pub fn unary(compiler: &mut Compiler, can_assign: bool) {

    let token_type = &compiler.parser.previous.token_type.clone();
    // compile the operand
    compiler.parse_precedence(&Precedence::Unary);

    // Emit the operator instruction
    match token_type {
        TokenType::Bang => compiler.writer.emit_byte(Opcode::OpNot, compiler.parser.previous.line),
        TokenType::Minus => compiler.writer.emit_byte(Opcode::OpNegate, compiler.parser.previous.line),
        _ => {}
    }
}
///
///
pub fn binary(compiler: &mut Compiler, can_assign: bool) {
    let token_type = &compiler.parser.previous.token_type.clone();
    // compile the operand

    let rule = ParserRule::get_rule(token_type);
    let prec_u8 = (*rule.precedence )as u8;
    let precedence:Precedence = FromPrimitive::from_u8(prec_u8 + 1).unwrap();
    compiler.parse_precedence( &precedence);
    //  let rule = self.getRule(token_type);
    // self.parse_precedence()

    // Emit the operator instruction
    match token_type {
        TokenType::Plus => compiler.writer.emit_byte(Opcode::OpAdd, compiler.parser.previous.line),
        TokenType::Minus => compiler.writer.emit_byte(Opcode::OPSubtract, compiler.parser.previous.line),
        TokenType::Star => compiler.writer.emit_byte(Opcode::OPMultiply, compiler.parser.previous.line),
        TokenType::Slash => compiler.writer.emit_byte(Opcode::OpDivide, compiler.parser.previous.line),

        TokenType::BangEqual => compiler.writer.emit_byte(Opcode::OpEqual, compiler.parser.previous.line),
        TokenType::EqualEqual => compiler.writer.emit_byte(Opcode::OpEqual, compiler.parser.previous.line),
        TokenType::Greater => compiler.writer.emit_byte(Opcode::OpGreater, compiler.parser.previous.line),
        TokenType::GreaterEqual => compiler.writer.emit_bytes(Opcode::OpLess, Opcode::OpNot, compiler.parser.previous.line),
        TokenType::Less => compiler.writer.emit_byte(Opcode::OpLess, compiler.parser.previous.line),
        TokenType::LessEqual => compiler.writer.emit_bytes(Opcode::OpGreater, Opcode::OpNot, compiler.parser.previous.line),
        _ => {}
    }
}

///
///
pub fn and(compiler: &mut Compiler, can_assign: bool) {
    let end_jump = compiler.emit_jump(OpJumpIfFalse(0));
    compiler.writer.emit_byte(Opcode::OpPop, compiler.parser.previous.line);
    compiler.parse_precedence(&Precedence::And);
    compiler.patch_jump(end_jump, &Opcode::OpJumpIfFalse(0))
}

///
///
pub fn or(compiler: &mut Compiler, can_assign: bool) {
    let else_jump = compiler.emit_jump(OpJumpIfFalse(0));
    let end_jump = compiler.emit_jump(Opcode::OpJump(0));

    compiler.patch_jump(else_jump, &Opcode::OpJumpIfFalse(0));

    compiler.writer.emit_byte(Opcode::OpPop, compiler.parser.previous.line);
    compiler.parse_precedence(&Precedence::Or);
    compiler.patch_jump(end_jump, &Opcode::OpJump(0));

}
