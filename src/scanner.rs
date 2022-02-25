

use crate::token::{ Token, TokenType};

struct Scanner {
    source: Vec<char>,
    tokens: Vec<Token>,


    start:usize,
    current:usize,
    line:usize,
}

impl Scanner {

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

    fn peek_next(&mut self) -> char {
        if self.current + 1 >= self.source.len() {
            return '\0';
        }
        return *self.source.get(self.current +1 ).unwrap()
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
        let str_literal =
        self.add_token(TokenType::String(Some(self.text(self.start+1, self.current-1))));
    }

    ///
    ///
    ///
    fn  number(&mut self) {
        while self.peek().is_digit(10) {
            self.advance();
        }

        // Look for a fractional part.
        if self.peek() == '.' && self.peek_next().is_digit(10)
        {
            // Consume the "."
            self.advance();

            while self.peek().is_digit(10) {
                self.advance();
            }
        }


        self.add_token(TokenType::Number(self.text(self.start, self.current).parse::<f64>().ok()));

    }

    ///
    ///
    ///

    fn identifier(&mut self) {
        while self.peek().is_alphanumeric() {
            self.advance();
        }

        let text = self.text(self.start, self.current);
        let token = match text.as_str() {
            "and" => TokenType::And,
            "class" => TokenType::Class,
            "else" => TokenType::Else,
            "false" => TokenType::False,
            "for" => TokenType::For,
            "fun" => TokenType::Fun,
            "if" => TokenType::If,
            "nil" => TokenType::Nil,
            "or" => TokenType::Or,
            "print" => TokenType::Print,
            "return" => TokenType::Return,
            "super" => TokenType::Super,
            "this" => TokenType::This,
            "true" => TokenType::True,
            "var" => TokenType::Var,
            "while" => TokenType::While,
            _ => TokenType::Identifier
        };

        self.add_token(token);



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
             ';' => self.add_token(TokenType::SemiColon),
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


            '"' => self.string(),

            _ => {
                if c.is_digit(10) {
                    self.number();
                } else if c.is_alphabetic() {
                    self.identifier();
                }else {
                   panic!("{} Unexpected character.", self.line);
                }
            }
        }

    }

}


#[cfg(test)]
mod tests {
    use crate::scanner::Scanner;

    #[test]
    fn basic_sum() {
       let mut scanner = Scanner::new("1 + 1".to_string());
        scanner.scan_tokens();
        println!("{:?}", scanner.tokens);
    }

    #[test]
    fn var_set() {
        let mut scanner = Scanner::new("var a = !((1+1)< 0)".to_string());
        scanner.scan_tokens();
        println!("{:?}", scanner.tokens);
    }

    #[test]
    fn multiline() {
        let mut scanner = Scanner::new("var a = 1+1\nvar b =  false".to_string());
        scanner.scan_tokens();
        println!("{:?}", scanner.tokens);
    }

    #[test]
    fn string() {
        let mut scanner = Scanner::new("var a = \"hello world\"".to_string());
        scanner.scan_tokens();
        println!("{:?}", scanner.tokens);
    }

}