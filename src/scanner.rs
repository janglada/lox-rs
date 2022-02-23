

use crate::token::{Token, TokenType};

struct Scanner<T> {
    source: Vec<char>,
    tokens: Vec<Token<T>>,


    start:usize,
    current:usize,
    line:usize,
}

impl<T> Scanner<T> {

    fn new(source: String) -> Self{
        Scanner {
            source: source.chars().collect::<Vec<char>>(),
            tokens: Vec::new(),
            start: 0,
            current: 0,
            line: 0,
        }
    }

    fn is_at_end(&self) -> bool {
        self.current >= self.source.len()
    }

    fn scan_tokens(&mut self)  {
        while !self.is_at_end() {
            self.start = self.current;
            self.scan_token();
        }

        self.tokens.push(Token::new(TokenType::EOF, "".to_string(), self.line));

    }
    /// The advance() method consumes the next character in the source file and returns it.
    ///
    fn advance(&mut self,) -> char {
        let c = self.source[self.current];
        self.current =  self.current + 1;
        c
    }

    ///
    ///
    fn peek(&mut self) -> char {
        if self.is_at_end() {
            return '\0'
        }
        *self.source.get(self.current).unwrap()
    }

    /// It’s like a conditional advance(). We only consume the current character if it’s what we’re looking for.
    ///
    ///
    fn match_char( &mut self, expected: char) -> bool{
        if self.is_at_end()  {
            return false
        }
        if *self.source.get(self.current).unwrap() != expected {
            return false
        }

        self.current = self.current + 1;

        true
    }

    fn add_token(&mut self, token: TokenType) {
        self.tokens.push(Token::new(token, self.current_text(), self.line))
    }

    fn add_token_literal(&mut self, token: TokenType, literal: T) {
        self.tokens.push(Token::new_literal(token, self.current_text(), literal, self.line))
    }

    fn current_text(&self) -> String {
        self.text(self.start, self.current)
    }

    fn text(&self, start: usize, end: usize) -> String {
        self.source[start..end].into_iter().collect()
    }

    fn string(&mut self) {
        while self.peek() != '"' && !self.is_at_end()
        {
            if self.peek() == '\n' {
                self.line = self.line + 1
            }

            self.advance();
        }

        if self.is_at_end() {
            panic!( " {} Unterminated string.", self.line);
        }

        // The closing ".
        self.advance();

        // Trim the surrounding quotes.
        self.add_token_literal(TokenType::String, self.text(self.start+1, self.current-1));
    }


    fn scan_token(&mut self) {
        let c: char = self.advance();

        match c {
             '(' => self.add_token(TokenType::LeftParen),
             ')' => self.add_token(TokenType::RightParen),
             '{' => self.add_token(TokenType::LeftBrace),
             '}' => self.add_token(TokenType::RightBrace),
             ',' => self.add_token(TokenType::Comma),
             '.' => self.add_token(TokenType::Dot),
             '-' => self.add_token(TokenType::Minus),
             '+' => self.add_token(TokenType::Plus),
             ';' => self.add_token(TokenType::SemiColoon),
             '*' => self.add_token(TokenType::Star),

            /// operators
            '!' => {
                if self.match_char('=') {
                    self.add_token(TokenType::BangEqual)
                } else {
                    self.add_token(TokenType::Bang)
                }
            }
            '=' => {
                if self.match_char('=') {
                    self.add_token(TokenType::EqualEqual)
                } else {
                    self.add_token(TokenType::Equal)
                }
            }
            '<' => {
                if self.match_char('=') {
                    self.add_token(TokenType::LessEqual)
                } else {
                    self.add_token(TokenType::Less)
                }
            }

            '>' => {
                if self.match_char('=') {
                    self.add_token(TokenType::GreaterEqual)
                } else {
                    self.add_token(TokenType::Greater)
                }
            }

            /// Whitespace
            ' ' => {
                // ignore
            }
            '\t' => {
                // ignore
            }
            '\n' => {
                self.line = self.line + 1;
            }

             '/' => {
                 if self.match_char('/') {
                     // A comment goes until the end of the line.
                     while self.peek() != '\n' && !self.is_at_end() {
                         self.advance();
                     }

                 } else {
                     self.add_token(TokenType::Slash)
                 }
            }



            _ => {
                panic!("{} {} Unexpected error", self.line, c);
            }
        }

    }

}