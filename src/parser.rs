use crate::chunk::{Chunk, ChunkIndex, ChunkWriterTrait};
use crate::compiler::{Compiler, Local};
use std::io::Write;

use crate::function::{FunctionType, ObjectFunction};
use crate::opcode::Opcode;
use crate::precedence::{ParserRule, Precedence};
use crate::scanner::Scanner;
use crate::token::TokenType::Comma;
use crate::token::{Token, TokenType};
use crate::value::Value;

use crate::chunk::ChunkArena;
use std::mem;

#[derive(Debug, Clone)]
pub struct ParserError {
    pub line: isize,
    pub start: usize,
    len: usize,
    pub msg: String,
}

#[derive(Debug)]
pub struct Parser<'a> {
    pub scanner: Scanner<'a>,
    pub compiler: Box<Compiler>,
    pub current: Token,
    pub previous: Token,
    pub result: Option<ParserError>,
    pub panic_mode: bool,
    resolver_errors: Vec<&'static str>,
    chunks: &'a mut ChunkArena,
}

impl<'a> Parser<'a> {
    ///
    ///
    ///
    pub fn new(source: &'a str, chunks_array: &'a mut ChunkArena) -> Self {
        let initial_chunk = chunks_array.allocate_chunk();

        Parser {
            scanner: Scanner::new(source),
            resolver_errors: Vec::new(),
            compiler: Compiler::new2(ObjectFunction::new(
                FunctionType::Script,
                "script".to_string(),
                initial_chunk,
            )),
            chunks: chunks_array,
            current: Token::dummy(),
            previous: Token::dummy(),
            result: None,
            panic_mode: false,
        }
    }

    ///
    ///
    ///
    fn advance(&mut self) {
        self.previous = self.current.clone();
        loop {
            self.current = self.scanner.scan_token();
            // dbg!(format!("CURRENT {:?} ", self.current.token_type));
            match self.current.token_type {
                TokenType::Error => {
                    println!("ERROR")
                }
                _ => {
                    break;
                }
            }
        }
    }

    ///
    ///
    pub fn compile(&mut self) -> Result<&mut ObjectFunction, ParserError> {
        self.result = None;
        self.panic_mode = false;
        self.advance();
        // self.expression();
        // self.consume(TokenType::EOF, "Expect end of expression");

        while !self.match_token(TokenType::EOF) {
            self.declaration();
        }

        self.end_compiler()
    }

    fn push_compiler(&mut self, kind: FunctionType) {
        // let function_name = self.gc.intern(self.previous..to_owned());
        match &self.previous.token_type {
            TokenType::Identifier(function_name) => {
                let index = self.chunks.allocate_chunk();
                let new_compiler =
                    Compiler::new2(ObjectFunction::new(kind, function_name.to_string(), index));

                // let new_compiler = Compiler::new(function_name, kind);
                let old_compiler = mem::replace(&mut self.compiler, new_compiler);
                self.compiler.enclosing = Some(old_compiler);
            }
            _ => {
                panic!("push_compiler ????")
            }
        }
    }

    fn pop_compiler(&mut self) -> Box<Compiler> {
        self.emit_return(self.previous.line);
        match self.compiler.enclosing.take() {
            Some(enclosing) => mem::replace(&mut self.compiler, enclosing),
            None => panic!("Didn't find an enclosing compiler"),
        }
    }

    ///
    ///
    ///
    fn declaration(&mut self) {
        if self.match_token(TokenType::Fun) {
            self.fun_declaration()
        } else if self.match_token(TokenType::Var) {
            self.var_declaration()
        } else {
            self.statement();
        }
        if self.panic_mode {
            self.synchronize();
        }
    }

    ///
    ///
    ///
    fn fun_declaration(&mut self) {
        let global = self.parse_variable("Expect function name");
        self.mark_initialized();
        self.function(FunctionType::Function);
        self.define_variable(global, self.previous.line);
    }

    ///
    ///
    fn var_declaration(&mut self) {
        let index = self.parse_variable("Expect variable name");
        if self.match_token(TokenType::Equal) {
            self.expression()
        } else {
            self.emit_byte(Opcode::OpNil, self.previous.line);
        }

        self.consume(TokenType::SemiColon, "Expect ';' after value");

        self.define_variable(index, self.previous.line);
    }

    ///
    ///
    fn parse_variable(&mut self, msg: &'a str) -> usize {
        self.consume(TokenType::Identifier("".to_string()), msg);

        self.declare_variable();
        if self.compiler.scope_depth > 0 {
            return 0;
        }

        self.identifier_constant()
    }

    ///
    ///
    ///
    pub(crate) fn identifier_constant(&mut self) -> usize {
        match &self.previous.token_type {
            TokenType::Identifier(name) => self.make_constant(Value::String(name.to_string())),
            _ => panic!("should not happen"),
        }
    }

    ///
    ///
    fn define_variable(&mut self, index: usize, line: isize) {
        // self.previous.line
        if self.compiler.scope_depth > 0 {
            self.mark_initialized();
            return;
        }
        self.emit_byte(Opcode::OpDefineGlobal(index), line)
    }

    ///
    ///
    fn declare_variable(&mut self) {
        if self.compiler.scope_depth == 0 {
            return;
        }
        let token = self.previous.clone();
        // it's an error to have two variables with the same name in the same local scope

        if self
            .compiler
            .locals
            .iter_mut()
            .rev()
            .take_while(|l| !(l.depth != -1 && l.depth < self.compiler.scope_depth))
            .find(|l| Parser::identifiers_equal(&token, &l.token))
            .is_some()
        {
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

        self.add_local(token);
        //  self.previous.
        //  self.writer.emit_byte(Opcode::OpDefineGlobal(index),   self.previous.line)
    }

    ///
    ///
    pub(crate) fn identifiers_equal(token1: &Token, token_opt2: &Option<Token>) -> bool {
        if let Some(token2) = token_opt2 {
            if token1.len != token2.len {
                false
            } else {
                match &token1.token_type {
                    TokenType::Identifier(name1) => match &token2.token_type {
                        TokenType::Identifier(name2) => name1 == name2,
                        _ => false,
                    },
                    _ => false,
                }
            }
        } else {
            false
        }
    }

    ///
    ///
    ///
    fn statement(&mut self) {
        if self.match_token(TokenType::Print) {
            self.print_statement()
        } else if self.match_token(TokenType::For) {
            self.for_statement()
        } else if self.match_token(TokenType::If) {
            self.if_statement()
        } else if self.match_token(TokenType::While) {
            self.while_statement()
        } else if self.match_token(TokenType::Return) {
            self.return_statement()
        } else if self.match_token(TokenType::LeftBrace) {
            self.begin_scope();
            self.block();
            self.end_scope();
        } else {
            self.expression_statement()
        }
    }

    fn block(&mut self) {
        while !self.check(TokenType::RightBrace) && !self.check(TokenType::EOF) {
            self.declaration();
        }
        self.consume(
            TokenType::RightBrace,
            "Expect '}' after block. Maybe a ';' missing?",
        );
    }

    ///
    ///
    ///
    pub(crate) fn match_token(&mut self, token_type: TokenType) -> bool {
        if !self.check(token_type) {
            return false;
        }
        self.advance();
        true
    }

    ///
    ///
    ///
    fn check(&mut self, token_type: TokenType) -> bool {
        self.current.token_type == token_type
    }

    ///
    ///
    ///
    fn print_statement(&mut self) {
        self.expression();
        self.consume(TokenType::SemiColon, "Expect ';' after value");
        self.emit_byte(Opcode::OpPrint, self.previous.line);
    }

    ///
    ///
    ///
    fn if_statement(&mut self) {
        self.consume(TokenType::LeftParen, "Expect '(' after if");
        self.expression();
        self.consume(TokenType::RightParen, "Expect ')' after if condition");

        let then_jump = self.emit_jump(Opcode::OpJumpIfFalse(0));
        self.emit_byte(Opcode::OpPop, self.previous.line);
        self.statement();
        let else_jump = self.emit_jump(Opcode::OpJump(0));

        self.patch_jump(then_jump, &Opcode::OpJumpIfFalse(0));
        self.emit_byte(Opcode::OpPop, self.previous.line);

        if self.match_token(TokenType::Else) {
            self.statement();
        }
        self.patch_jump(else_jump, &Opcode::OpJump(0));
    }

    ///
    ///
    fn return_statement(&mut self) {
        if self.match_token(TokenType::SemiColon) {
            self.emit_return(self.previous.line);
        } else {
            self.expression();
            self.consume(TokenType::SemiColon, "Expect ';' after return value");
            self.emit_byte(Opcode::OpReturn, self.previous.line);
        }
    }
    fn while_statement(&mut self) {
        let loop_start = self.length();
        self.consume(TokenType::LeftParen, "Expect '(' after while");
        self.expression();
        self.consume(TokenType::RightParen, "Expect ')' after while condition");

        let exit_jump = self.emit_jump(Opcode::OpJumpIfFalse(0));
        self.emit_byte(Opcode::OpPop, self.previous.line);
        self.statement();
        self.emit_loop(loop_start);

        self.patch_jump(exit_jump, &Opcode::OpJumpIfFalse(0));
        self.emit_byte(Opcode::OpPop, self.previous.line);
    }

    ///
    ///
    ///
    fn for_statement(&mut self) {
        self.begin_scope();
        self.consume(TokenType::LeftParen, "Expect '(' after 'for'");
        if self.match_token(TokenType::SemiColon) {
            // no initializer
        } else if self.match_token(TokenType::Var) {
            self.var_declaration();
        } else {
            self.expression_statement();
        }
        let mut loop_start = self.length();
        //condition clause
        let mut exit_jump: Option<usize> = None;
        if !self.match_token(TokenType::SemiColon) {
            self.expression();
            self.consume(TokenType::SemiColon, "Expect ';' after loop condition");
            exit_jump = Some(self.emit_jump(Opcode::OpJumpIfFalse(0)));
            self.emit_byte(Opcode::OpPop, self.previous.line);
        }

        // self.consume(TokenType::RightParen, "Expect ')' after 'for' clauses");

        // increment clause
        if !self.match_token(TokenType::RightParen) {
            let body_jump = self.emit_jump(Opcode::OpJump(0));
            let incr_start = self.length();
            self.expression();
            self.emit_byte(Opcode::OpPop, self.previous.line);
            self.consume(TokenType::RightParen, "Expect ')' after 'for' clauses");

            self.emit_loop(loop_start);
            loop_start = incr_start;
            self.patch_jump(body_jump, &Opcode::OpJump(0))
        }
        self.statement();
        self.emit_loop(loop_start);
        if let Some(jump) = exit_jump {
            self.patch_jump(jump, &Opcode::OpJumpIfFalse(0));
            self.emit_byte(Opcode::OpPop, self.previous.line);
        }
        self.end_scope();
    }
    ///
    ///
    ///
    pub(crate) fn emit_jump(&mut self, opcode: Opcode) -> usize {
        self.emit_byte(opcode, self.previous.line);
        self.length()
    }

    ///
    ///
    fn emit_loop(&mut self, loop_start: usize) {
        self.emit_byte(Opcode::OpLoop(0), self.previous.line);
        let len = self.length();
        let offset = len - loop_start;
        if offset > u16::MAX as usize {
            self.error("Loop body too large");
        }
        self.replace_opcode(len - 1, Opcode::OpLoop(offset as u16));
    }
    ///
    ///
    ///
    pub(crate) fn patch_jump(&mut self, offset: usize, opcode: &Opcode) {
        let jump = self.length() - offset;
        if jump > u16::MAX as usize {
            self.error("Too much code to jump over");
        }

        let patched_opcode = match opcode {
            Opcode::OpJumpIfFalse(_) => Opcode::OpJumpIfFalse(jump as u16),
            Opcode::OpJump(_) => Opcode::OpJump(jump as u16),
            _ => {
                panic!("Not a jumpable opcode")
            }
        };

        self.replace_opcode(offset - 1, patched_opcode);
    }
    ///
    ///
    ///
    fn expression_statement(&mut self) {
        self.expression();
        self.consume(TokenType::SemiColon, "Expect ';' after value");
        self.emit_byte(Opcode::OpPop, self.previous.line);
    }

    ///
    ///
    ///
    fn synchronize(&mut self) {
        self.panic_mode = false;
        if self.previous.token_type == TokenType::SemiColon {
            return;
        }
        match self.current.token_type {
            TokenType::Class
            | TokenType::Fun
            | TokenType::Var
            | TokenType::For
            | TokenType::If
            | TokenType::While
            | TokenType::Print
            | TokenType::Return => (),
            _ => self.advance(),
        }
    }
    ///
    ///
    ///
    pub fn consume(&mut self, token_type: TokenType, message: &'a str) {
        if self.current.token_type == token_type {
            self.advance();
            return;
        }
        // io::stdout().flush().ok().unwrap();
        self.error_at_current(message);
    }

    ///
    ///
    fn end_compiler(&mut self) -> Result<&mut ObjectFunction, ParserError> {
        self.emit_return(self.previous.line);

        // if let None = self.result {
        //     self.compiler
        //         .function
        //         .disassemble_chunk(&mut (Box::new(io::stdout()) as Box<dyn Write>));
        //
        //     return Ok(&mut self.compiler.function);
        // }
        let res = &self.result;

        // write!(io::stdout(), "{}\n", "CHUNKS");

        // self.compiler
        //     .function
        //     .disassemble_chunk(&mut (Box::new(io::stdout()) as Box<dyn Write>));

        match res {
            None => Ok(&mut self.compiler.function),
            Some(err) => Err(err.clone()),
        }
    }

    ///
    ///
    pub(crate) fn expression(&mut self) {
        self.parse_precedence(&Precedence::Assigment)
    }

    pub(crate) fn parse_precedence(&mut self, precedence: &Precedence) {
        self.advance();
        let prefix_rule = ParserRule::get_rule(&self.previous.token_type).prefix;

        let can_assign = *precedence <= Precedence::Assigment;
        if prefix_rule.is_none() {
            self.error("Expect expression");
            return;
        } else {
            prefix_rule.unwrap()(self, can_assign);
        }

        while precedence <= ParserRule::get_rule(&self.current.token_type).precedence {
            self.advance();
            let infix_rule = ParserRule::get_rule(&self.previous.token_type).infix;
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
    pub(crate) fn resolve_local(&mut self) -> Option<usize> {
        let result = self
            .compiler
            .resolve_local(&self.previous, &mut self.resolver_errors);
        while let Some(e) = self.resolver_errors.pop() {
            self.error(e);
        }
        result
    }

    ///
    ///
    pub(crate) fn add_local(&mut self, token: Token) {
        let _result = self.compiler.add_local(
            Local {
                token: Some(token),
                depth: -1, //self.scope_depth
            },
            &mut self.resolver_errors,
        );

        while let Some(e) = self.resolver_errors.pop() {
            self.error(e);
        }
    }

    ///
    ///
    pub(crate) fn mark_initialized(&mut self) {
        if self.compiler.scope_depth == 0 {
            return;
        }
        self.compiler
            .locals
            .last_mut()
            // .get_mut(self.compiler.local_count - 1)
            .unwrap()
            .depth = self.compiler.scope_depth;
    }

    ///
    ///
    pub(crate) fn function(&mut self, kind: FunctionType) {
        self.push_compiler(kind);
        self.begin_scope();
        self.consume(TokenType::LeftParen, "Expect '(' after function name");
        if !self.check(TokenType::RightParen) {
            loop {
                self.compiler.function.arity += 1;
                if self.compiler.function.arity == 254 {
                    self.error_at_current("Can't have more than 255 parameters")
                }
                let index = self.parse_variable("Expect parameter name");
                self.define_variable(index, self.previous.line);
                if !self.match_token(Comma) {
                    break;
                }
            }
        }
        self.consume(TokenType::RightParen, "Expect ')' after function name");
        self.consume(TokenType::LeftBrace, "Expect '{' before function body");
        self.block();

        let compiler = self.pop_compiler();
        let function = compiler.function;
        let v = Value::Function(*function);
        let _ = &self.emit_constant(v, self.previous.line);
    }

    ///
    ///
    ///
    pub fn argument_list(&mut self) -> u8 {
        let mut count = 0u8;
        if !self.check(TokenType::RightParen) {
            loop {
                self.expression();
                if count == 255 {
                    self.error("Can't have more than 255 arguments")
                }
                count += 1;
                if !self.match_token(TokenType::Comma) {
                    // dbg!(format!("NOT A COMMA {:?}", self.current.token_type));
                    break;
                }
            }
        }
        self.consume(TokenType::RightParen, "Expect ')' after arguments");
        //  dbg!(count);
        count
    }

    ///
    ///
    pub(crate) fn begin_scope(&mut self) {
        self.compiler.scope_depth += 1;
    }

    ///
    ///
    pub(crate) fn end_scope(&mut self) {
        self.compiler.scope_depth -= 1;

        while let Some(local) = self.compiler.locals.last() {
            if local.depth > self.compiler.scope_depth {
                self.emit_byte(Opcode::OpPop, self.previous.line);

                self.compiler.locals.pop();
            } else {
                break;
            }
        }

        // while self.compiler.local_count > 0
        //     && self
        //         .compiler
        //         .locals
        //         .get(self.compiler.local_count - 1)
        //         .unwrap()
        //         .depth
        //         > self.compiler.scope_depth
        // {
        //     self.compiler
        //         .function
        //         .emit_byte(Opcode::OpPop, self.previous.line);
        //     self.compiler.local_count.sub_assign(1);
        // }
    }

    ///
    ///
    fn error_at_current(&mut self, msg: &'a str) {
        self.error_at(&self.current.clone(), msg);
    }

    ///
    ///
    fn error(&mut self, msg: &'a str) {
        self.error_at(&mut self.previous.clone(), msg);
    }

    ///
    ///
    fn error_at(&mut self, token: &Token, msg: &'a str) {
        if self.panic_mode {
            return;
        }
        self.panic_mode = true;
        eprint!(" [line {}] Error", token.line);
        match token.token_type {
            TokenType::EOF => eprint!(" at end"),
            TokenType::Error => {}
            _ => {
                eprint!(" at {} {}", token.len, token.start);
            }
        }
        eprintln!(": {}", msg);

        self.result = Some(ParserError {
            line: token.line,
            start: token.start,
            len: token.len,
            msg: msg.to_string(),
        });
        // self.result = Some(Box::new(LoxCompileError {
        //     src: NamedSource::new("bad_file.rs", self.scanner.get_input()),
        //     bad_bit: (9, 4).into(),
        // }));
    }

    fn chunk_index(&self) -> ChunkIndex {
        self.compiler.function.chunk_index
    }

    ///
    ///
    pub fn emit_byte(&mut self, byte: Opcode, line: isize) {
        self.write_chunk(byte, line);
    }
    ///
    ///
    pub(crate) fn emit_bytes(&mut self, byte1: Opcode, byte2: Opcode, line: isize) {
        self.emit_byte(byte1, line);
        self.emit_byte(byte2, line);
    }
    ///
    ///
    pub(crate) fn emit_return(&mut self, line: isize) {
        self.emit_byte(Opcode::OpNil, line);
        self.emit_byte(Opcode::OpReturn, line);
    }
    ///
    ///
    pub fn emit_constant(&mut self, value: Value, line: isize) {
        let idx = self.make_constant(value);
        self.emit_byte(Opcode::OpConstant(idx), line)
    }
    pub fn chunk(&mut self) -> &mut Chunk {
        let index = self.chunk_index();
        self.chunks.chunks.get_mut(index).unwrap()
    }

    pub fn chunk_at(&self, index: ChunkIndex) -> &Chunk {
        self.chunks.chunks.get(index).unwrap()
    }

    fn write_chunk(&mut self, byte: Opcode, _line: isize) {
        let index = self.chunk_index();
        self.chunk().write_chunk(byte);
    }
    pub fn make_constant(&mut self, value: Value) -> usize {
        self.chunk().add_constant(value)
    }

    pub fn disassemble_chunk(&mut self, writer: &mut Box<dyn Write>) {
        self.chunk().disassemble_chunk(writer);
        self.chunk().disassemble_chunk_constants(writer);
    }

    pub fn length(&mut self) -> usize {
        self.chunk().op_codes.len()
    }

    pub fn replace_opcode(&mut self, index: usize, bytes: Opcode) {
        self.chunk().replace_opcode(index, bytes);
    }
}
