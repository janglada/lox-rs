pub enum TokenType {
    // Single-character tokens.
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    Comma, Dot, Minus, Plus, SemiColoon, Slash, Star,

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
    Identifier, String, Number,

    // Keywords.
    And, Class, Else, False, Fun, For, If, Nil, Or,
    Print, Return, Super, This, True, Var, While,

    EOF
}
pub struct Token<T> {
    token_type: TokenType,
    lexeme: String,
    literal: Option<T>,
    line: usize
}

impl<T> Token<T> {

    pub fn new(token_type:TokenType, lexeme: String,   line: usize)-> Self {
        Token {
            token_type, lexeme, literal: None, line
        }
    }

    pub fn new_literal(token_type:TokenType, lexeme: String,  literal: T, line: usize)-> Self {
        Token {
            token_type, lexeme, literal: Some(literal), line
        }
    }

}