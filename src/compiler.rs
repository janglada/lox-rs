use num_traits::FromPrimitive;
use crate::chunk::{Chunk, WritableChunk};
use crate::opcode::Opcode;
use crate::parser::Parser;
use crate::precedence::{ParserRule, Precedence};
use crate::scanner::Scanner;
use crate::token::{Token, TokenType};
use crate::value::{Value};

#[derive(Debug)]
pub struct Compiler<'a> {
    scanner: Scanner<'a>,
    parser: Parser<'a>,
    writer: ChunkWriter<'a>
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

    fn disassemble_chunk(&mut self) {
        self.chunk.disassemble_chunk()
    }
}



impl<'a> Compiler<'a> {

    pub(crate) fn new(source:  &'a str, chunk: &'a mut Chunk) -> Self {
        Compiler {
            scanner: Scanner::new(source), parser:Parser::new(), writer: ChunkWriter::new(chunk)
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
        self.expression();
        self.consume(TokenType::EOF, "Expect end of expression");
        self.end_compiler();
        self.parser.result.is_ok()
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
            self.writer.disassemble_chunk();
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

        if prefix_rule.is_none() {
            self.error("Expect expression")
        } else {
            prefix_rule.unwrap()(self);
        }

        while precedence <= ParserRule::get_rule(&self.parser.current.token_type).precedence
        {
            self.advance();
            let infix_rule = ParserRule::get_rule(&self.parser.previous.token_type).infix;
            if infix_rule.is_some() {
                infix_rule.unwrap()(self);
            }
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


pub fn number(compiler: &mut Compiler) {
    match &compiler.parser.previous.token_type {
        TokenType::Number(num) => {
            compiler.writer.emit_constant(Value::Number(*num), compiler.parser.previous.line)
        }
        _ => panic!("unexpected token type")
    }
}


pub fn string(compiler: &mut Compiler) {
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
pub fn grouping(compiler: &mut Compiler) {
    compiler.expression();
    compiler.consume(TokenType::RightParen, "Expected ')' after expression")
}


///
///
pub fn literal(compiler: &mut Compiler) {
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
pub fn unary(compiler: &mut Compiler) {

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
pub fn binary(compiler: &mut Compiler) {
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


