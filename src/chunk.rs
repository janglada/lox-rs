use std::io::Write;
use crate::opcode::Opcode;
use crate::value::Value;


#[derive(Debug)]
pub struct Chunk {
    pub op_codes: Vec<Opcode>,
    pub constants: Vec<Value>
}

impl Chunk {
    pub fn new() -> Chunk {
        Chunk {
            op_codes: Vec::new(),
            constants: Vec::new(),
        }
    }
}

pub trait WritableChunk {
    fn write_chunk(&mut self, bytes: Opcode);
    fn add_constant(&mut self, value: Value) -> usize;
    fn read_constant(&self, index : usize) -> Option<&Value>;
    fn disassemble_chunk(&mut self, writer: &mut dyn Write);
    fn disassemble_instruction(&mut self, offset: usize, writer: &mut dyn Write) -> usize;
    fn simple_instruction(&mut self, name: &str, offset: usize, writer: &mut dyn Write) -> usize;
    fn constant_instruction(&mut self, name: &str, offset: usize, const_idx: usize, writer: &mut dyn Write) -> usize;
    fn byte_instruction(&mut self, name: &str, offset: usize, const_idx: usize, writer: &mut dyn Write) -> usize;

}

impl WritableChunk for Chunk {

    fn write_chunk(&mut self, bytes: Opcode) {
       self.op_codes.push(bytes);
    }

    fn add_constant(&mut self, value: Value) -> usize {
        self.constants.push(value);
        self.constants.len() - 1
    }

    fn read_constant(&self, index : usize) -> Option<&Value>{
        self.constants.get(index)
    }

    fn disassemble_chunk(&mut self, writer: &mut dyn Write) {
        let mut offset: usize = 0;
        while offset < self.op_codes.len() {
            offset = self.disassemble_instruction(offset, writer);
        }
    }

    fn disassemble_instruction(&mut self, offset: usize, writer: &mut dyn Write) -> usize {
        write!(writer, "{:04} ", offset);
        let opcode = self.op_codes.get(offset).unwrap();
        match opcode {
            Opcode::OpReturn => {
                self.simple_instruction("OP_RETURN", offset, writer)
            },
            Opcode::OpNegate => {
                self.simple_instruction("OP_NEGATE", offset, writer)
            },
            Opcode::OpNot => {
                self.simple_instruction("OP_NOT", offset, writer)
            },
            Opcode::OpConstant(size) => {
                self.constant_instruction("OP_CONSTANT", offset, *size, writer)
            },
            Opcode::OpDefineGlobal(size) => {
                self.constant_instruction("OP_DEFINE_GLOBAL", offset, *size, writer)
            },
            Opcode::OpGetGlobal(size) => {
                self.constant_instruction("OP_GET_GLOBAL", offset, *size, writer)
            },
            Opcode::OpSetGlobal(size) => {
                self.constant_instruction("OP_SET_GLOBAL", offset, *size, writer)
            },

            Opcode::OpSetLocal(size) => {
                self.byte_instruction("OP_GET_LOCAL", offset, *size, writer)
            },
            Opcode::OpGetLocal(size) => {
                self.byte_instruction("OP_SET_LOCAL", offset, *size, writer)
            },
            Opcode::OpAdd => {
                self.simple_instruction("OP_ADD", offset, writer)
            },
            Opcode::OPSubtract => {
                self.simple_instruction("OP_SUBTRACT", offset, writer)
            },
            Opcode::OPMultiply => {
                self.simple_instruction("OP_MULTIPLY", offset, writer)
            },
            Opcode::OpDivide => {
                self.simple_instruction("OP_DIVIDE", offset, writer)
            },
            Opcode::OpFalse => self.simple_instruction("OP_FALSE", offset, writer),
            Opcode::OpNil=> self.simple_instruction("OP_NIL", offset, writer),
            Opcode::OpTrue => self.simple_instruction("OP_TRUE", offset, writer),

            Opcode::OpEqual => self.simple_instruction("OP_EQUAL", offset, writer),
            Opcode::OpGreater => self.simple_instruction("OP_GREATER", offset, writer),
            Opcode::OpLess => self.simple_instruction("OP_LESS", offset, writer),
            Opcode::OpPrint => self.simple_instruction("OP_PRINT", offset, writer),
            Opcode::OpPop => self.simple_instruction("OP_POP", offset, writer),
            _ => {
                offset + 1
            }
        }
    }

    fn simple_instruction(&mut self, name: &str, offset: usize, writer: &mut dyn Write) -> usize {
        write!(writer, "{: <12}\n", name);
        offset + 1
    }

    fn constant_instruction(&mut self, name: &str, offset: usize, const_idx: usize, writer: &mut dyn Write) -> usize {
        let value = self.constants.get(const_idx).unwrap();
        write!(writer, "{: <12} {} '{}' \n", name, const_idx, value);
        offset + 1
    }

    fn byte_instruction(&mut self, name: &str, offset: usize, const_idx: usize, writer: &mut dyn Write) -> usize {
        let op_code = self.op_codes.get(offset ).unwrap();
        match op_code {
            Opcode::OpGetLocal(idx) => {
                write!(writer, "{: <12} {}  \n", name, idx);
            }
            Opcode::OpSetLocal(idx) => {
                write!(writer, "{: <12} {}  \n", name, idx);
            }
            _ => {
                panic!("INVALID")
            }
        }

        offset + 2
    }
}


#[cfg(test)]
mod tests {
    use std::io;
    use std::io::Write;
    use crate::chunk::{Chunk, WritableChunk};
    use crate::opcode::Opcode;
    use crate::value::Value;
    use crate::vm::VM;

    #[test]
    fn negate() {
        let mut chunk : Chunk = Chunk::new();
        let idx = chunk.add_constant(Value::Number(3.14));

        chunk.write_chunk(Opcode::OpConstant(idx));
        chunk.write_chunk(Opcode::OpNegate);
        chunk.write_chunk(Opcode::OpReturn);

       chunk.disassemble_chunk(&mut (Box::new(io::stdout()) as Box<dyn Write>));

        let mut vm = VM::new();
        vm.run(&chunk);

    }

    #[test]
    fn basic_sum() {
        let mut chunk : Chunk = Chunk::new();

        let mut constant = chunk.add_constant(Value::Number(1.2));
        chunk.write_chunk(Opcode::OpConstant(constant));

        constant = chunk.add_constant(Value::Number(3.4));
        chunk.write_chunk(Opcode::OpConstant(constant));

        chunk.write_chunk(Opcode::OpAdd);

        constant = chunk.add_constant(Value::Number(5.6));
        chunk.write_chunk(Opcode::OpConstant(constant));

        chunk.write_chunk(Opcode::OpDivide);
        chunk.write_chunk(Opcode::OpNegate);
        chunk.write_chunk(Opcode::OpReturn);



        chunk.disassemble_chunk(&mut (Box::new(io::stdout()) as Box<dyn Write>));

        let mut vm = VM::new();
        vm.run(&chunk);

    }

}
