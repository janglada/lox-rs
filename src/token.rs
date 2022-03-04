
use std::hash::{Hash, Hasher};


#[derive(Debug,    Clone)]
pub enum TokenType {
    // Single-character tokens.
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    Comma, Dot, Minus, Plus,
    SemiColon, Slash, Star,

    // One or two character tokens.
    Bang,
    BangEqual,
    Equal,
    EqualEqual,
    Greater,
    GreaterEqual,
    Less,
    LessEqual,

    // Literals.
    Identifier(String),
    String(String),
    Number(f64),

    // Keywords.
    And, Class, Else, False, Fun, For, If, Nil, Or,
    Print, Return, Super, This, True, Var, While,

    Error,

    EOF
}
/// https://stackoverflow.com/questions/63466669/check-partialeq-on-enum-without-check-the-attached-data-of-the-variant
impl std::cmp::PartialEq for TokenType {
    fn eq(&self, other: &Self) -> bool {
        use std::mem::discriminant;
        discriminant(self) == discriminant(other)
    }
}

impl Eq for TokenType {

}

impl Hash for TokenType {
    fn hash<H: Hasher>(&self, state: &mut H) {
        match self {
            TokenType::LeftParen => { state.write_u8(1);}
            TokenType::RightParen => {state.write_u8(2);}
            TokenType::LeftBrace => {state.write_u8(3);}
            TokenType::RightBrace => {state.write_u8(4);}
            TokenType::Comma => {state.write_u8(5);}
            TokenType::Dot => {state.write_u8(6);}
            TokenType::Minus => {state.write_u8(7);}
            TokenType::Plus => {state.write_u8(8);}
            TokenType::SemiColon => {state.write_u8(9);}
            TokenType::Slash => {state.write_u8(10);}
            TokenType::Star => {state.write_u8(11);}
            TokenType::Bang => {state.write_u8(12);}
            TokenType::BangEqual => {state.write_u8(13);}
            TokenType::Equal => {state.write_u8(14);}
            TokenType::EqualEqual => {state.write_u8(15);}
            TokenType::Greater => {state.write_u8(16);}
            TokenType::GreaterEqual => {state.write_u8(17);}
            TokenType::Less => {state.write_u8(18);}
            TokenType::LessEqual => {state.write_u8(19);}
            TokenType::Identifier(_) => {state.write_u8(20);}
            TokenType::String(_) => {state.write_u8(21);}
            TokenType::Number(_) => {state.write_u8(22);}
            TokenType::And => {state.write_u8(23);}
            TokenType::Class => {state.write_u8(24);}
            TokenType::Else => {state.write_u8(25);}
            TokenType::False => {state.write_u8(26);}
            TokenType::Fun => {state.write_u8(27);}
            TokenType::For => {state.write_u8(28);}
            TokenType::If => {state.write_u8(29);}
            TokenType::Nil => {state.write_u8(30);}
            TokenType::Or => {state.write_u8(31);}
            TokenType::Print => {state.write_u8(32);}
            TokenType::Return => {state.write_u8(33);}
            TokenType::Super => {state.write_u8(34);}
            TokenType::This => {state.write_u8(35);}
            TokenType::True => {state.write_u8(36);}
            TokenType::Var => {state.write_u8(37);}
            TokenType::While => {state.write_u8(38);}
            TokenType::Error => {state.write_u8(39);}
            TokenType::EOF => {state.write_u8(40);}
        }
    }
}



#[derive(Debug,  Clone)]
pub struct Token {
    pub token_type: TokenType,
    pub line: isize,
    pub start: usize,
    pub len: usize
}







impl Token {
    pub fn new(token_type:TokenType, start: usize, len: usize,   line: isize)-> Self {
        Token {
            token_type, start, len,  line
        }
    }

    pub fn dummy() -> Self {
        Token {
            token_type: TokenType::Nil, start: 0, len:0,  line:0
        }
    }
}


#[cfg(test)]
mod tests {
    use crate::precedence::Precedence;
    use crate::token::TokenType;

    #[test]
    fn test_prec() {

        println!(" {:?} {}", Precedence::Primary, Precedence::Primary as u8);
        println!(" {:?} {}", Precedence::Call, Precedence::Call as u8);
        println!(" {:?} {}", Precedence::None, Precedence::None as u8);
        assert!((Precedence::Primary ) > (Precedence::Call ));
        assert!(!((Precedence::None ) > (Precedence::Call )));
    }
}
