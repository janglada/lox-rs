#[derive(Debug)]
pub enum Opcode {
    OpConstant(usize),
    OpDefineGlobal(usize),
    OpGetGlobal(usize),
    OpSetGlobal(usize),
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