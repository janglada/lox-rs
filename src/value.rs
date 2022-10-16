use std::fmt::{Display, Formatter};

use crate::function::ObjectFunction;
use crate::native::ObjectNative;

#[derive(Debug, Clone, PartialEq)]
pub enum Value {
    Boolean(bool),
    Nil,
    Number(f64),
    String(String),
    Function(ObjectFunction),
    NativeFunction(ObjectNative),
}

impl Value {
    pub fn new_string(str: &str) -> Value {
        Value::String(str.to_owned())
    }

    pub fn is_number(&self) -> bool {
        match self {
            Value::Number(_) => true,
            _ => false,
        }
    }
    pub fn is_bool(&self) -> bool {
        match self {
            Value::Boolean(_) => true,
            _ => false,
        }
    }

    pub fn is_string(&self) -> bool {
        match self {
            Value::String(_str) => true,
            _ => false,
        }
    }

    pub fn is_object(&self) -> bool {
        match self {
            Value::Function(_s) => true,
            Value::NativeFunction(_s) => true,
            _ => false,
        }
    }

    pub fn is_function(&self) -> bool {
        match self {
            Value::Function(_s) => true,
            _ => false,
        }
    }

    pub fn is_native(&self) -> bool {
        match self {
            Value::NativeFunction(_s) => true,
            _ => false,
        }
    }

    pub fn as_number(&self) -> Result<&f64, &str> {
        match self {
            Value::Number(c) => Ok(c),
            _ => Err("Must be a number"),
        }
    }

    pub fn as_bool(&self) -> Result<&bool, &str> {
        match self {
            Value::Boolean(c) => Ok(c),
            _ => Err("Must be a boolean"),
        }
    }

    pub fn as_string(&self) -> Result<&String, &str> {
        match self {
            Value::String(s) => Ok(s),
            _ => Err("Must be a obj string"),
        }
    }

    pub fn as_function(&self) -> Result<ObjectFunction, &str> {
        match self {
            Value::Function(objFn) => {
                //   let x = unsafe { &mut (*(*objFn)) };
                Ok(objFn.clone())
            }
            _ => Err("Must be a obj string"),
        }
    }

    pub fn as_native(&self) -> Result<ObjectNative, &str> {
        match self {
            Value::NativeFunction(objFn) => {
                //   let x = unsafe { &mut (*(*objFn)) };
                Ok(objFn.clone())
            }
            _ => Err("Must be a obj string"),
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

            Value::String(s) => {
                write!(f, "{}", s)
            }
            Value::Function(obj) => {
                // let name = unsafe { &(*(*obj)).name };
                write!(f, "<fn {}>", obj.name)
            }
            Value::NativeFunction(obj) => {
                write!(f, "<native fn {}>", obj.name)
            }
        }
    }
}
#[cfg(test)]
mod tests {
    use crate::value::Value;

    #[test]
    fn assert_eqs() {
        assert_eq!(
            Value::String("A".to_string()),
            Value::String("A".to_string())
        );

        assert_ne!(
            Value::String("A".to_string()),
            Value::String("B".to_string())
        );

        assert_ne!(Value::String("A".to_string()), Value::Number(1f64));

        assert_eq!(Value::Number(3.1), Value::Number(3.1),);

        assert_ne!(Value::Number(2.0), Value::Number(1f64));
    }
}
