#[derive(Debug)]
pub enum Opcode {
    OpConstant(usize),

    OpDefineGlobal(usize),

    OpGetGlobal(usize),
    OpSetGlobal(usize),

    OpGetLocal(usize),
    OpSetLocal(usize),


    OpJumpIfFalse(u16),
    OpJump(u16),
    OpLoop(u16),


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