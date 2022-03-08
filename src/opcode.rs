#[derive(Debug)]
pub enum Opcode {
    OpConstant(usize),
    OpDefineGlobal(usize),
    OpGetGlobal(usize),
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
    OpDivide,

    OpEqual,
    OpGreater,
    OpLess,

    OpPrint,
    OpPop,


}