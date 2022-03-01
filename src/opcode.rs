
pub enum Opcode {
    OpConstant(usize),

    OpReturn,

    /// unary
    OpNegate,


    /// binary
    OpAdd,
    OPSubtract,
    OPMultiply,
    OpDivide
}