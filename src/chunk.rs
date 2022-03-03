use crate::opcode::Opcode;

pub type Value = f64;
pub type ConstantPool = Vec<f64>;

pub struct Chunk {
    pub op_codes: Vec<Opcode>,
    pub constants: Vec<f64>
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
    fn disassemble_chunk(&mut self);
    fn disassemble_instruction(&mut self, offset: usize) -> usize;
    fn simple_instruction(&mut self, name: &str, offset: usize) -> usize;
    fn constant_instruction(&mut self, name: &str, offset: usize, const_idx: usize) -> usize;

}

impl WritableChunk for Chunk {

    fn write_chunk(&mut self, bytes: Opcode) {
       self.op_codes.push(bytes);
    }

    fn add_constant(&mut self, value: Value) -> usize {
        self.constants.push(value);
        self.constants.len() - 1
    }

    fn disassemble_chunk(&mut self) {

        let mut offset: usize = 0;
        while offset < self.op_codes.len() {
            offset = self.disassemble_instruction(offset);
        }

    }

    fn disassemble_instruction(&mut self, offset: usize) -> usize {
        print!("{:04} ", offset);
        let opcode = self.op_codes.get(offset).unwrap();
        match opcode {
            Opcode::OpReturn => {
                return self.simple_instruction("OP_RETURN", offset);
            },
            Opcode::OpNegate => {
                return self.simple_instruction("OP_NEGATE", offset);
            },
            Opcode::OpConstant(size) => {
                return self.constant_instruction("OP_CONSTANT", offset, *size);
            },

            Opcode::OpAdd => {
                return self.simple_instruction("OP_ADD", offset);
            },
            Opcode::OPSubtract => {
                return self.simple_instruction("OP_SUBTRACT", offset);
            },
            Opcode::OPMultiply => {
                return self.simple_instruction("OP_MULTIPLY", offset);
            },
            Opcode::OpDivide => {
                return self.simple_instruction("OP_DIVIDE", offset);
            },
            _ => {
                return offset + 1;
            }
        }
    }

    fn simple_instruction(&mut self, name: &str, offset: usize) -> usize {
        print!("{: <12}\n", name);
        offset + 1
    }

    fn constant_instruction(&mut self, name: &str, offset: usize, const_idx: usize) -> usize {
        let value = self.constants.get(const_idx).unwrap();
        print!("{: <12} {} '{}'\n", name, const_idx, value);
        offset + 1
    }
}


#[cfg(test)]
mod tests {
    use crate::chunk::{Chunk, WritableChunk};
    use crate::opcode::Opcode;
    use crate::vm::VM;

    #[test]
    fn negate() {
        let mut chunk : Chunk = Chunk::new();
        let idx = chunk.add_constant(3.14);

        chunk.write_chunk(Opcode::OpConstant(idx));
        chunk.write_chunk(Opcode::OpNegate);
        chunk.write_chunk(Opcode::OpReturn);

       chunk.disassemble_chunk();

        let mut vm = VM::new();
        vm.run(&chunk);

    }

    #[test]
    fn basic_sum() {
        let mut chunk : Chunk = Chunk::new();

        let mut constant = chunk.add_constant(1.2);
        chunk.write_chunk(Opcode::OpConstant(constant));

        constant = chunk.add_constant(3.4);
        chunk.write_chunk(Opcode::OpConstant(constant));

        chunk.write_chunk(Opcode::OpAdd);

        constant = chunk.add_constant(5.6);
        chunk.write_chunk(Opcode::OpConstant(constant));

        chunk.write_chunk(Opcode::OpDivide);
        chunk.write_chunk(Opcode::OpNegate);
        chunk.write_chunk(Opcode::OpReturn);



        chunk.disassemble_chunk();

        let mut vm = VM::new();
        vm.run(&chunk);

    }

}
