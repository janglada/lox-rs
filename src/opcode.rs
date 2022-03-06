#[derive(Debug)]
pub enum Opcode {
    OpConstant(usize),
    OpNil,
    OpTrue,
    OpFalse,

    OpReturn,

    /// unary
    OpNot,
    OpNegate,


    /// binary
    OpAdd,
    OPSubtract,
    OPMultiply,
    OpDivide
}