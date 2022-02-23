#[derive(Debug)]
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
    Identifier,
    String(Option<String>),
    Number(Option<f64>),

    // Keywords.
    And, Class, Else, False, Fun, For, If, Nil, Or,
    Print, Return, Super, This, True, Var, While,

    EOF
}
#[derive(Debug)]
pub struct Token {
    token_type: TokenType,
    lexeme: String,
    line: usize
}







impl Token {
    pub fn new(token_type:TokenType, lexeme: String,   line: usize)-> Self {
        Token {
            token_type, lexeme,  line
        }
    }
}