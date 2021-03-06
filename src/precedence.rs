use crate::compiler::{and, binary, call, grouping, literal, number, or, string, unary, variable};
use crate::parser::Parser;
use crate::token::TokenType;
use lazy_static::lazy_static;
use num_derive::FromPrimitive;
use std::collections::HashMap;
use std::fmt::{Debug, Formatter};
lazy_static! {

    static ref PARSER_RULES: HashMap<TokenType, ParserRule<'static>> = {
        let mut m = HashMap::new();

        // @formatter:off
        m.insert(TokenType::LeftParen  ,                    ParserRule::new(Some(grouping), Some(call),     &Precedence::Call));
        m.insert(TokenType::RightParen ,                    ParserRule::new(None,           None,           &Precedence::None));
        m.insert(TokenType::LeftBrace ,                     ParserRule::new(None,           None,           &Precedence::None));
        m.insert(TokenType::RightBrace ,                    ParserRule::new(None,           None,           &Precedence::None));
        m.insert(TokenType::Comma ,                         ParserRule::new(None,           None,           &Precedence::None));
        m.insert(TokenType::Dot ,                           ParserRule::new(None,           None,           &Precedence::None));
        m.insert(TokenType::Minus ,                         ParserRule::new(Some(unary),    Some(binary),   &Precedence::Term));
        m.insert(TokenType::Plus ,                          ParserRule::new(None,           Some(binary),   &Precedence::Term));
        m.insert(TokenType::SemiColon ,                     ParserRule::new(None,           None,           &Precedence::None));
        m.insert(TokenType::Slash ,                         ParserRule::new(None,           Some(binary),   &Precedence::Factor));
        m.insert(TokenType::Star ,                          ParserRule::new(None,           Some(binary),   &Precedence::Factor));
        m.insert(TokenType::Bang ,                          ParserRule::new(Some(unary),    None,           &Precedence::None));
        m.insert(TokenType::BangEqual ,                     ParserRule::new(None,           Some(binary),   &Precedence::Equality));
        m.insert(TokenType::Equal ,                         ParserRule::new(None,           None,           &Precedence::None));
        m.insert(TokenType::EqualEqual ,                    ParserRule::new(None,           Some(binary),   &Precedence::Equality));
        m.insert(TokenType::Greater ,                       ParserRule::new(None,           Some(binary),   &Precedence::Comparison));
        m.insert(TokenType::GreaterEqual ,                  ParserRule::new(None,           Some(binary),   &Precedence::Comparison));
        m.insert(TokenType::Less ,                          ParserRule::new(None,           Some(binary),   &Precedence::Comparison));
        m.insert(TokenType::LessEqual ,                     ParserRule::new(None,           Some(binary),   &Precedence::Comparison));
        m.insert(TokenType::Identifier("".to_string()),     ParserRule::new(Some(variable), None,           &Precedence::None));
        m.insert(TokenType::String("".to_string()) ,        ParserRule::new(Some(string),   None,           &Precedence::None));
        m.insert(TokenType::Number(0.) ,                    ParserRule::new(Some(number),   None,           &Precedence::None));
        m.insert(TokenType::And ,                           ParserRule::new(None,           Some(and),      &Precedence::And));
        m.insert(TokenType::Class ,                         ParserRule::new(None,           None,           &Precedence::None));
        m.insert(TokenType::Else ,                          ParserRule::new(None,           None,           &Precedence::None));
        m.insert(TokenType::False ,                         ParserRule::new(Some(literal),  None,           &Precedence::None));
        m.insert(TokenType::Fun ,                           ParserRule::new(None,           None,           &Precedence::None));
        m.insert(TokenType::For ,                           ParserRule::new(None,           None,           &Precedence::None));
        m.insert(TokenType::If ,                            ParserRule::new(None,           None,           &Precedence::None));
        m.insert(TokenType::Nil ,                           ParserRule::new(Some(literal),  None,           &Precedence::None));
        m.insert(TokenType::Or ,                            ParserRule::new(None,           Some(or),       &Precedence::Or));
        m.insert(TokenType::Print ,                         ParserRule::new(None,           None,           &Precedence::None));
        m.insert(TokenType::Return,                         ParserRule::new(None,           None,           &Precedence::None));
        m.insert(TokenType::Super ,                         ParserRule::new(None,           None,           &Precedence::None));
        m.insert(TokenType::This ,                          ParserRule::new(None,           None,           &Precedence::None));
        m.insert(TokenType::True ,                          ParserRule::new(Some(literal),  None,           &Precedence::None));
        m.insert(TokenType::Var ,                           ParserRule::new(None,           None,           &Precedence::None));
        m.insert(TokenType::While ,                         ParserRule::new(None,           None,           &Precedence::None));
        m.insert(TokenType::Error ,                         ParserRule::new(None,           None,           &Precedence::None));
        m.insert(TokenType::EOF ,                           ParserRule::new(None,           None,           &Precedence::None));
        // @formatter:on
        m
    };
}

#[derive(Debug, PartialOrd, Ord, PartialEq, Eq, Hash, FromPrimitive, Clone, Copy)]
pub enum Precedence {
    None,
    Assigment,
    Or,
    And,
    Equality,
    Comparison,
    Term,
    Factor,
    Unary,
    Call,
    Primary,
}

pub struct ParserRule<'a> {
    pub(crate) prefix: Option<ParseFn>,
    pub(crate) infix: Option<ParseFn>,
    pub(crate) precedence: &'a Precedence,
}

impl<'a> Debug for ParserRule<'a> {
    fn fmt(&self, _f: &mut Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}

impl<'a> ParserRule<'a> {
    fn new(prefix: Option<ParseFn>, infix: Option<ParseFn>, precedence: &'a Precedence) -> Self {
        ParserRule {
            prefix,
            infix,
            precedence,
        }
    }

    pub fn get_rule(token_type: &TokenType) -> &ParserRule {
        PARSER_RULES.get(token_type).unwrap()
    }
}

pub type ParseFn = fn(compiler: &mut Parser, can_assign: bool);

#[cfg(test)]
mod tests {
    use crate::precedence::{Precedence, PARSER_RULES};
    use crate::token::TokenType;
    use num_traits::FromPrimitive;

    #[test]
    fn test_prec() {
        let _t: TokenType = TokenType::And;

        println!(" {:?} {}", Precedence::Primary, Precedence::Primary as u8);
        println!(" {:?} {}", Precedence::Call, Precedence::Call as u8);
        println!(" {:?} {}", Precedence::None, Precedence::None as u8);
        assert!((Precedence::Primary) > (Precedence::Call));
        assert!((Precedence::None) <= (Precedence::Call));
    }

    #[test]
    fn test_next() {
        let precedence: Precedence = FromPrimitive::from_u8(Precedence::Or as u8 + 1).unwrap();

        match precedence {
            Precedence::And => {}
            _ => panic!("   "),
        }
    }

    #[test]
    fn test_map() {
        match &PARSER_RULES.get(&TokenType::And).unwrap() {
            &_rule => {}
        }

        match &PARSER_RULES.get(&TokenType::Number(0.)).unwrap() {
            &_rule => {}
        }

        match &PARSER_RULES.get(&TokenType::Number(1.)).unwrap() {
            &_rule => {}
        }
    }
}
