use crate::chunk::{Chunk, WritableChunk};
use crate::opcode::Opcode;
use crate::parser::Parser;
use crate::scanner::Scanner;
use crate::token::{Token, TokenType};


pub struct Compiler<'a> {
    scanner: Scanner<'a>,
    parser: Parser<'a>,
    compiling_chunk: &'a mut Chunk
}




impl<'a> Compiler<'a> {

    pub(crate) fn new(source:  &'a str, chunk: &'a mut Chunk) -> Self {
        Compiler {
            scanner: Scanner::new(source), parser:Parser::new(), compiling_chunk: chunk
        }
    }

    fn advance(&mut self) {
        self.parser.previous = self.parser.current;
        loop {
            self.parser.current = self.scanner.scan_token();
            match self.parser.current.token_type  {
                TokenType::Error => {  },
                _ => {

                    break;
                }
            }
        }
    }

    ///
    ///
    pub  fn compile(&mut self) -> bool {

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
    pub fn emit_byte(&mut self, byte: Opcode) {
        self.write_chunk( byte, self.parser.previous.line);
    }


    ///
    ///
    pub fn emit_bytes(&mut self, byte1: Opcode, byte2: Opcode) {
       self.emit_byte(byte1);
       self.emit_byte(byte2);
    }

    ///
    ///
    // fn current_chunk(&'a mut self) -> &'a mut Chunk {
    //     self.compiling_chunk
    // }
    ///
    ///
    fn write_chunk(&mut self, byte: Opcode, line: isize ) {
        self.compiling_chunk.write_chunk(byte);
        // self.current_chunk().write_chunk(byte);
    }

    ///
    ///
    fn end_compiler(&mut self, ) {
        self.emit_return();
    }

    ///
    ///
    fn emit_return(&mut self, ) {
        self.emit_byte(Opcode::OpReturn)
    }

    ///
    ///
    fn expression(&mut self) {

    }

    fn number(&mut self) {
        self.parser.previous.start
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
        eprint!(": {}\n", msg);
        self.parser.result = Err(msg);
    }
}



