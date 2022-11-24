use crate::function::ObjectFunction;
#[derive(Debug, Clone, PartialEq)]
pub struct ObjectClosure {
    pub(crate) function: ObjectFunction,
}

impl ObjectClosure {
    ///
    ///
    ///
    pub fn new(function: ObjectFunction) -> Self {
        ObjectClosure { function }
    }
}
