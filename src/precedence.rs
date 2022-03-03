#[derive(Debug,PartialOrd, Ord, PartialEq, Eq)]
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


#[cfg(test)]
mod tests {
    use crate::precedence::Precedence;

    #[test]
    fn test_prec() {
        println!(" {:?} {}", Precedence::Primary, Precedence::Primary as u8);
        println!(" {:?} {}", Precedence::Call, Precedence::Call as u8);
        println!(" {:?} {}", Precedence::None, Precedence::None as u8);
        assert!((Precedence::Primary ) > (Precedence::Call ));
        assert!(!((Precedence::None ) > (Precedence::Call )));
    }
}