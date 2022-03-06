use std::fmt::{Display, Formatter};

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum ObjectValue {
    String(String)
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Value {
    Boolean(bool),
    Nil,
    Number(f64),
    Object(ObjectValue)
}

impl Value {

    pub fn is_number(&self) -> bool {
        match self {

            Value::Number(c) => {
                true
            },
            _ => false,
        }
    }
    pub fn is_bool(&self) -> bool {
        match self {

            Value::Boolean(c) => {
                true
            },
            _ => false,
        }
    }
    pub fn as_number(&self) -> Result<&f64, &str> {
        match self {
            Value::Number(c) => {
                Ok(c)
            },
            _ => Err("Must be a number"),
        }
    }

    pub fn as_bool(&self) -> Result<&bool, &str> {
        match self {

            Value::Boolean(c) => {
                Ok(c)
            },
            _ => Err("Must be a boolean"),
        }
    }

}


impl Display for Value {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Value::Boolean(t) => {
                write!(f, "{}", t)
            }
            Value::Nil => {
                write!(f, "nil")
            }
            Value::Number(n) => {
                write!(f, "{}", n)
            }
            Value::Object(v) => {
                write!(f, "Object {}", "[obj]]")
            }
        }
    }
}
