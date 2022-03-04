

use std::collections::HashMap;
use lazy_static::lazy_static;
use crate::token::TokenType;

lazy_static! {

    static ref PARSER_RULES: HashMap<TokenType, ParserRule<'static>> = {
        let mut m = HashMap::new();

        m.insert(TokenType::LeftParen  , ParserRule::new(None, None, &Precedence::None));
        m.insert(TokenType::RightParen , ParserRule::new(None, None, &Precedence::None));
        m.insert(TokenType::LeftBrace , ParserRule::new(None, None, &Precedence::None));
        m.insert(TokenType::RightBrace , ParserRule::new(None, None, &Precedence::None));
        m.insert(TokenType::Comma , ParserRule::new(None, None, &Precedence::None));
        m.insert(TokenType::Dot , ParserRule::new(None, None, &Precedence::None));
        m.insert(TokenType::Minus , ParserRule::new(None, None, &Precedence::None));
        m.insert(TokenType::Plus , ParserRule::new(None, None, &Precedence::None));
        m.insert(TokenType::SemiColon , ParserRule::new(None, None, &Precedence::None));
        m.insert(TokenType::Slash , ParserRule::new(None, None, &Precedence::None));
        m.insert(TokenType::Star , ParserRule::new(None, None, &Precedence::None));
        m.insert(TokenType::Bang , ParserRule::new(None, None, &Precedence::None));
        m.insert(TokenType::BangEqual , ParserRule::new(None, None, &Precedence::None));
        m.insert(TokenType::Equal , ParserRule::new(None, None, &Precedence::None));
        m.insert(TokenType::EqualEqual , ParserRule::new(None, None, &Precedence::None));
        m.insert(TokenType::Greater , ParserRule::new(None, None, &Precedence::None));
        m.insert(TokenType::GreaterEqual , ParserRule::new(None, None, &Precedence::None));
        m.insert(TokenType::Less , ParserRule::new(None, None, &Precedence::None));
        m.insert(TokenType::LessEqual , ParserRule::new(None, None, &Precedence::None));
        m.insert(TokenType::Identifier("".to_string()) , ParserRule::new(None, None, &Precedence::None));
        m.insert(TokenType::String("".to_string()) , ParserRule::new(None, None, &Precedence::None));
        m.insert(TokenType::Number(0.) , ParserRule::new(None, None, &Precedence::None));
        m.insert(TokenType::And , ParserRule::new(None, None, &Precedence::None));
        m.insert(TokenType::Class , ParserRule::new(None, None, &Precedence::None));
        m.insert(TokenType::Else , ParserRule::new(None, None, &Precedence::None));
        m.insert(TokenType::False , ParserRule::new(None, None, &Precedence::None));
        m.insert(TokenType::Fun , ParserRule::new(None, None, &Precedence::None));
        m.insert(TokenType::For , ParserRule::new(None, None, &Precedence::None));
        m.insert(TokenType::If , ParserRule::new(None, None, &Precedence::None));
        m.insert(TokenType::Nil , ParserRule::new(None, None, &Precedence::None));
        m.insert(TokenType::Or , ParserRule::new(None, None, &Precedence::None));
        m.insert(TokenType::Print , ParserRule::new(None, None, &Precedence::None));
        m.insert(TokenType::Return , ParserRule::new(None, None, &Precedence::None));
        m.insert(TokenType::Super , ParserRule::new(None, None, &Precedence::None));
        m.insert(TokenType::This , ParserRule::new(None, None, &Precedence::None));
        m.insert(TokenType::True , ParserRule::new(None, None, &Precedence::None));
        m.insert(TokenType::Var , ParserRule::new(None, None, &Precedence::None));
        m.insert(TokenType::While , ParserRule::new(None, None, &Precedence::None));
        m.insert(TokenType::Error , ParserRule::new(None, None, &Precedence::None));
        m.insert(TokenType::EOF , ParserRule::new(None, None, &Precedence::None));
        m
    };
}


#[derive(Debug,PartialOrd, Ord, PartialEq, Eq, Hash)]
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
    Primary
}
#[derive(Debug)]
pub struct  ParserRule<'a> {
    prefix: Option<ParseFn>,
    infix: Option<ParseFn>,
    precedence: &'a Precedence,

}

impl<'a> ParserRule<'a> {
    pub fn new(prefix: Option<ParseFn>, infix: Option<ParseFn>,  precedence: &'a Precedence) ->Self {
        ParserRule {
            prefix, infix, precedence
        }

    }
}

pub type ParseFn = fn();




#[cfg(test)]
mod tests {
    use crate::precedence::{PARSER_RULES, ParserRule, Precedence};
    use crate::token::TokenType;

    #[test]
    fn test_prec() {


        let t : TokenType = TokenType::And;
        


            println!(" {:?} {}", Precedence::Primary, Precedence::Primary as u8);
        println!(" {:?} {}", Precedence::Call, Precedence::Call as u8);
        println!(" {:?} {}", Precedence::None, Precedence::None as u8);
        assert!((Precedence::Primary ) > (Precedence::Call ));
        assert!(!((Precedence::None ) > (Precedence::Call )));
    }

    #[test]
    fn test_map() {



        match  &PARSER_RULES.get(&TokenType::And).unwrap() {
            &rule => {
                dbg!(rule);
            }
        }

        match  &PARSER_RULES.get(&TokenType::Number(0.)).unwrap() {
            &rule => {
                dbg!(rule);
            }
        }

        match  &PARSER_RULES.get(&TokenType::Number(1.)).unwrap() {
            &rule => {
                dbg!(rule);
            }
        }


    }

    }