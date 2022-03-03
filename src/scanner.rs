
use crate::token::{Token, TokenType};

pub struct Scanner<'a> {
    input: &'a str,
    source: Vec<char>,
    // The starting index of the next character.
    start: usize,
    current: usize,
    pub line: isize
}

impl<'a> Scanner<'a> {
    pub fn new(source: &'a str) -> Self{
        let v = source.chars().collect::<Vec<char>>();
        Self {
            input: source,
            source: source.chars().collect::<Vec<char>>(),
            start: 0,
            current: 0,
            line: 0
        }
    }

    pub fn start(&mut self)
    {
        let mut line: isize =  -1;
        loop {
            let token = self.scan_token();
            if token.line != line {
                print!("\n{:04}\t", token.line);
                line = token.line;
            } else {
                print!("\t| ");
            }
            print!("{:?} ", token.token_type);


            if token.token_type == TokenType::EOF {
                break;
            }
        }
    }

    fn is_at_end(&mut self) -> bool {
        self.peek() == '\0'
    }
    ///
///
    fn peek(&mut self) -> char {
        unsafe {
            *self.source.get_unchecked(self.current)
        }
    }

    ///
    ///
    fn peek_next(&mut self) -> char {
        unsafe {
            *self.source.get_unchecked(self.current + 1)
        }
    }

    fn advance(&mut self) -> char {
        let idx = self.current;
        self.current = self.current + 1;
        unsafe {
            *self.source.get_unchecked(idx)
        }

    }

    ///
    ///
    ///
    pub fn scan_token(&mut self) ->Token {
        self.skip_whitespace();

        self.start = self.current;

        if self.is_at_end() {
            return self.make_token(TokenType::EOF);
        }
        let c = self.advance();

        match c {
            '\0' => self.make_token(TokenType::EOF),
            '(' => self.make_token(TokenType::LeftParen),
            ')' => self.make_token(TokenType::RightParen),
            '{' => self.make_token(TokenType::LeftBrace),
            '}' => self.make_token(TokenType::RightBrace),
            ',' => self.make_token(TokenType::Comma),
            '.' => self.make_token(TokenType::Dot),
            '-' => self.make_token(TokenType::Minus),
            '+' => self.make_token(TokenType::Plus),
            ';' => self.make_token(TokenType::SemiColon),
            '*' => self.make_token(TokenType::Star),

            /// operators
            '!' => {
                if self.match_char('=') {
                    return self.make_token(TokenType::BangEqual)
                } else {
                    return self.make_token(TokenType::Bang)
                }
            }
            '=' => {
                if self.match_char('=') {
                    return self.make_token(TokenType::EqualEqual)
                } else {
                    return self.make_token(TokenType::Equal)
                }
            }
            '<' => {
                if self.match_char('=') {
                    return self.make_token(TokenType::LessEqual)
                } else {
                    return self.make_token(TokenType::Less)
                }
            }

            '>' => {
                if self.match_char('=') {
                    return self.make_token(TokenType::GreaterEqual)
                } else {
                    return self.make_token(TokenType::Greater)
                }
            }

            '"' => self.string(),

            _ => {
                if c.is_digit(10) {
                    return self.number();
                } else if Scanner::is_alpha(c)  {
                    return self.identifier(c);
                } else {
                    panic!("Line: {} [{}] Unexpected character.", self.line, c);
                }
            }
        }


    }

    fn is_alpha(c: char) -> bool{
        c.is_alphabetic() || c == '_'
    }


    fn make_string_token_type(&self) -> TokenType {
        TokenType::String(self.get_token_text())
    }

    fn make_number_token_type(&self) -> TokenType {
        TokenType::Number(self.get_token_text())
    }

    fn make_identifier_token_type(&self) -> TokenType {
        TokenType::Identifier(self.get_token_text())
    }
    


    ///
    ///
    ///
    fn make_token(&self, token_type: TokenType) -> Token {
        // dbg!("MAKE TOKEN {:?} {}...{}", token_type,  self.start, self.current);
        Token::new(
            token_type,
            self.start,
            self.current -self.start,
            self.line
        )

    }

    fn get_token_text(&self) -> String{


       // self.source[self.start..self.current].to_owned().as_slice()
        // let v = self.source[self.start..self.current];
        //
        //
        // .iter().collect::<String>()

        self.source.iter().skip(self.start).take(self.current- self.start).collect::<String>()
    }

    ///
    ///
    ///
    fn skip_whitespace(&mut self) {
        loop {
            let c = self.peek();

            match c {
                ' ' |  '\r' |  '\t'=> {
                    self.advance();
                }
                '\n' => {
                    self.line = self.line + 1;
                    self.advance();
                },
                '/' => {
                    if self.peek_next() == '/' {
                        while self.peek() != '\n' && !self.is_at_end() {
                            self.advance();
                        }
                    } else {
                        return
                    }
                },
                _ =>{
                    return
                }
            }

        }
    }

    ///
    ///
    ///
    fn string(&mut self) -> Token {
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

        return self.make_token(self.make_string_token_type())

    }

    ///
    ///
    ///
    fn  number(&mut self) -> Token {
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

        self.make_token(self.make_number_token_type())

    }



    ///
    ///
    ///
    fn check_keyword(&self, start: usize, len : usize, rest: &str, token_type: TokenType) -> TokenType{

        if (self.current - self.start == start +  len) && self.source[self.start + start..self.start + start+len].iter().collect::<String>().eq(rest) {

            token_type
        } else {
            self.make_identifier_token_type()
        }

        // println!("DEBUG REST {} ", rest);
        // if rest.chars().all(|c| {
        //     // let advance_char = self.advance();
        //     // println!("DEBUG {} {}", c, advance_char);
        //     if  c ==  self.advance() {
        //         true
        //     } else {
        //         false
        //     }
        // } ) && !self.advance().is_alphanumeric() {
        //     token_type
        // } else {
        //     while self.advance().is_alphanumeric() {
        //         // self.advance();
        //     }
        //     TokenType::Identifier
        // }
    }

    ///
    ///
    ///
    fn identifier(&mut self, c: char) -> Token {
        while Scanner::is_alpha(self.peek() )|| self.peek().is_digit(10) {
            self.advance();

        }

        let token_type = match c {
            '\0' => TokenType::EOF,
            'a' => self.check_keyword(1, 2,"nd", TokenType::And),
            'c' => self.check_keyword(1, 4,"lass", TokenType::Class),
            'e' => self.check_keyword(1, 3,"lse", TokenType::Else),
            'i' => self.check_keyword(1, 1,"f", TokenType::If),
            'n' => self.check_keyword(1, 2,"il", TokenType::Nil),
            'o' => self.check_keyword(1, 1,"r", TokenType::Or),
            'p' => self.check_keyword(1, 4,"rint", TokenType::Print),
            'r' => self.check_keyword(1, 5,"eturn", TokenType::Return),
            's' => self.check_keyword(1, 4,"super", TokenType::Super),
            'v' => self.check_keyword(1, 2,"ar", TokenType::Var),
            'w' => self.check_keyword(1, 4,"hile", TokenType::While),
            'f' => {
                //dbg!("{} {} {}", self.current, self.start, self.source[self.start]);
               if self.current - self.start > 1 {
                match self.source[self.start+1]{
                    'a' => self.check_keyword(2,3,"lse", TokenType::False),
                    'o' => self.check_keyword(2,1,"r", TokenType::For),
                    'u' => self.check_keyword(2,1,"n", TokenType::Fun),
                    _ => self.make_identifier_token_type()
                }
               } else {
                   self.make_identifier_token_type()
               }
            },
            't' => {
                if self.current - self.start > 1 {
                    match self.source[self.start+1] {
                        'h' => self.check_keyword(2, 2, "is", TokenType::This),
                        'r' => self.check_keyword(2, 2, "ue", TokenType::True),
                        _ => self.make_identifier_token_type()
                    }
                }else {
                    self.make_identifier_token_type()
                }
            }
            _ => self.make_identifier_token_type()
        };
        return self.make_token(token_type)

    }


    /// It’s like a conditional advance(). We only consume the current character if it’s what we’re looking for.
    ///
    ///
    fn match_char( &mut self, expected: char) -> bool{
        if self.is_at_end()  {
            return false
        }
        if self.peek() != expected {
            return false
        }

        self.advance();

        true
    }

}

#[cfg(test)]
mod tests {
    use std::slice;
    use std::str;
    use crate::scanner::Scanner;

    #[test]
     fn pointer_test() {


        let s: &str = "à23";
        let ptr: *const u8 = s.as_ptr();

        unsafe {
            let c1 = *ptr.offset(1) as u32;
            println!("{}", std::char::from_u32_unchecked(c1) );
            println!("{}", *ptr.offset(2) as char);
        }

    }


    #[test]
    fn basic_sum() {
        let mut scanner = Scanner::new("1 + 1");
        scanner.start();

    }

    #[test]
    fn var_set() {
        let mut scanner = Scanner::new("var a = !((1+1)< 0)");
        scanner.start();
    }

    #[test]
    fn multiline() {
        let mut scanner = Scanner::new("var a = 1+1\nvar b = false");
        scanner.start();
    }

    #[test]
    fn string() {
        let mut scanner = Scanner::new("var a = \"hello world\"");
        scanner.start();
    }

}