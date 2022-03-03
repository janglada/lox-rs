#[derive(Debug, PartialEq,  Clone)]
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
    Number(String),

    // Keywords.
    And, Class, Else, False, Fun, For, If, Nil, Or,
    Print, Return, Super, This, True, Var, While,

    Error,

    EOF
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