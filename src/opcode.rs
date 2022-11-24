use std::fs::File;
use std::io::Read;

#[derive(Debug, PartialEq, Clone)]
pub enum Opcode {
    OpConstant(usize),

    OpDefineGlobal(usize),

    OpGetGlobal(usize),
    OpSetGlobal(usize),

    OpGetLocal(usize),
    OpSetLocal(usize),

    OpCall(u8),

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

    OpClosure(usize),
}

impl Into<Vec<u8>> for &Opcode {
    fn into(self) -> Vec<u8> {
        let mut v = Vec::new();

        match &self {
            Opcode::OpConstant(_) => v.push(1),
            Opcode::OpDefineGlobal(_) => v.push(2),
            Opcode::OpGetGlobal(_) => v.push(3),
            Opcode::OpSetGlobal(_) => v.push(4),
            Opcode::OpGetLocal(_) => v.push(5),
            Opcode::OpSetLocal(_) => v.push(6),
            Opcode::OpJumpIfFalse(_) => v.push(7),
            Opcode::OpJump(_) => v.push(8),
            Opcode::OpLoop(_) => v.push(9),
            Opcode::OpNil => v.push(10),
            Opcode::OpTrue => v.push(11),
            Opcode::OpFalse => v.push(12),
            Opcode::OpReturn => v.push(13),
            Opcode::OpNot => v.push(14),
            Opcode::OpNegate => v.push(15),
            Opcode::OpAdd => v.push(16),
            Opcode::OPSubtract => v.push(17),
            Opcode::OPMultiply => v.push(18),
            Opcode::OpDivide => v.push(19),
            Opcode::OpEqual => v.push(20),
            Opcode::OpGreater => v.push(21),
            Opcode::OpLess => v.push(22),
            Opcode::OpPrint => v.push(23),
            Opcode::OpPop => v.push(24),
            Opcode::OpCall(_) => v.push(25),
            Opcode::OpClosure(_) => v.push(26),
        };

        match &self {
            // usize
            Opcode::OpClosure(idx)
            | Opcode::OpConstant(idx)
            | Opcode::OpDefineGlobal(idx)
            | Opcode::OpGetGlobal(idx)
            | Opcode::OpSetGlobal(idx)
            | Opcode::OpGetLocal(idx)
            | Opcode::OpSetLocal(idx) => {
                v.extend_from_slice(&idx.to_le_bytes());
                v
            }

            Opcode::OpCall(args) => {
                v.push(*args);
                v
            }

            // u16
            Opcode::OpJumpIfFalse(jump) | Opcode::OpJump(jump) | Opcode::OpLoop(jump) => {
                v.extend_from_slice(&jump.to_le_bytes());
                v
            }

            //
            _ => v,
        }
    }
}

fn usize_from_bytes(bytes: &[u8]) -> usize {
    let mut dst = [0u8; 8];
    dst.clone_from_slice(bytes);
    usize::from_le_bytes(dst)
}
fn usize_from_reader(reader: &mut File) -> usize {
    let mut buffer = [0_u8; std::mem::size_of::<usize>()];
    reader.read(&mut buffer).unwrap();
    usize::from_le_bytes(buffer)
}
fn u16_from_bytes(bytes: &[u8]) -> u16 {
    let mut dst = [0u8; 2];
    dst.clone_from_slice(&bytes[1..]);
    u16::from_le_bytes(dst)
}
fn u16_from_reader(reader: &mut File) -> u16 {
    let mut buffer = [0_u8; std::mem::size_of::<u16>()];
    reader.read(&mut buffer).unwrap();
    u16::from_le_bytes(buffer)
}
impl Opcode {
    pub(crate) fn from_file(reader: &mut File) -> Option<Self> {
        // reader.bytes().for_each(|b|  {
        //     if let Ok(r) = b {
        //         println!("{:?}", r)
        //     }
        // });
        //
        // return None;
        let mut buff = [0u8; 1];
        // let num_read = reader.read(&mut buff);
        if let Ok(n) = reader.read(&mut buff) {
            if n > 0 {
                let code_u8 = buff[0];
                let code = match code_u8 {
                    // usize
                    1 => Opcode::OpConstant(usize_from_reader(reader)),
                    2 => Opcode::OpDefineGlobal(usize_from_reader(reader)),
                    3 => Opcode::OpGetGlobal(usize_from_reader(reader)),
                    4 => Opcode::OpSetGlobal(usize_from_reader(reader)),
                    5 => Opcode::OpGetLocal(usize_from_reader(reader)),
                    6 => Opcode::OpSetLocal(usize_from_reader(reader)),

                    // jumps
                    7 => Opcode::OpJumpIfFalse(u16_from_reader(reader)),
                    8 => Opcode::OpJump(u16_from_reader(reader)),
                    9 => Opcode::OpLoop(u16_from_reader(reader)),

                    // rest
                    10 => Opcode::OpNil,
                    11 => Opcode::OpTrue,
                    12 => Opcode::OpFalse,
                    13 => Opcode::OpReturn,
                    14 => Opcode::OpNot,
                    15 => Opcode::OpNegate,
                    16 => Opcode::OpAdd,
                    17 => Opcode::OPSubtract,
                    18 => Opcode::OPMultiply,
                    19 => Opcode::OpDivide,
                    20 => Opcode::OpEqual,
                    21 => Opcode::OpGreater,
                    22 => Opcode::OpLess,
                    23 => Opcode::OpPrint,
                    24 => Opcode::OpPop,
                    26 => Opcode::OpClosure(usize_from_reader(reader)),

                    _ => panic!("Unknwon opcode {}", buff[0]),
                };
                return Some(code);
            }
        }
        None
    }
}
/*

#[cfg(test)]
mod tests {
    use crate::opcode::Opcode;

    #[test]
    fn simple_conversion() {
       // let mut  v = Vec::new();

        let bytes : Vec<u8>= (&Opcode::OpPop).into();
        let opcode = Opcode::from( bytes.as_slice());

        assert_eq!(opcode, Opcode::OpPop);

    }

    #[test]
    fn field_conversion1() {
        // let mut  v = Vec::new();

        let bytes : Vec<u8>= (&Opcode::OpConstant(137)).into();
        let opcode = Opcode::from( bytes.as_slice());

       match opcode {
           Opcode::OpConstant(s) => {
               assert_eq!(137, s);
           }
           _ => panic!("Invalid opcode")
       }
    }

    #[test]
    fn field_conversion2() {
        // let mut  v = Vec::new();

        let bytes : Vec<u8>= (&Opcode::OpJump(99)).into();
        let opcode = Opcode::from( bytes.as_slice());

        match opcode {
            Opcode::OpJump(s) => {
                assert_eq!(99, s);
            }
            _ => panic!("Invalid opcode")
        }

    }

    #[test]
    fn field_conversion3() {
        let mut  v: Vec<Opcode> = Vec::new();
        v.push(Opcode::OpJump(99));

        let bytes : Vec<u8>= (&Opcode::OpJump(99)).into();
        let opcode = Opcode::from( bytes.as_slice());

        match opcode {
            Opcode::OpJump(s) => {
                assert_eq!(99, s);
            }
            _ => panic!("Invalid opcode")
        }

    }


}
*/
