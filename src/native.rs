

use crate::value::Value;
use std::fmt;

pub type NativeFn = fn(arg_count: u8, args: *const Value) -> Value;

#[derive(Clone)]
pub struct ObjectNative {
    pub name: String,
    pub function: NativeFn,
}
impl ObjectNative {
    pub fn new(name: String, function: NativeFn) -> Self {
        ObjectNative { name, function }
    }
}
impl PartialEq for ObjectNative {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
    }
}

impl fmt::Debug for ObjectNative {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Object native function {}]", self.name)
    }
}
