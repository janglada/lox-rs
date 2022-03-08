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

    fn read_constant(&self, index : usize) -> Option<&Value>{
        self.constants.get(index)
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
                self.simple_instruction("OP_RETURN", offset)
            },
            Opcode::OpNegate => {
                self.simple_instruction("OP_NEGATE", offset)
            },
            Opcode::OpNot => {
                self.simple_instruction("OP_NOT", offset)
            },
            Opcode::OpConstant(size) => {
                self.constant_instruction("OP_CONSTANT", offset, *size)
            },
            Opcode::OpDefineGlobal(size) => {
                self.constant_instruction("OP_DEFINE_GLOBAL", offset, *size)
            },
            Opcode::OpGetGlobal(size) => {
                self.constant_instruction("OP_GET_GLOBAL", offset, *size)
            },
            Opcode::OpAdd => {
                self.simple_instruction("OP_ADD", offset)
            },
            Opcode::OPSubtract => {
                self.simple_instruction("OP_SUBTRACT", offset)
            },
            Opcode::OPMultiply => {
                self.simple_instruction("OP_MULTIPLY", offset)
            },
            Opcode::OpDivide => {
                self.simple_instruction("OP_DIVIDE", offset)
            },
            Opcode::OpFalse => self.simple_instruction("OP_FALSE", offset),
            Opcode::OpNil=> self.simple_instruction("OP_NIL", offset),
            Opcode::OpTrue => self.simple_instruction("OP_TRUE", offset),

            Opcode::OpEqual => self.simple_instruction("OP_EQUAL", offset),
            Opcode::OpGreater => self.simple_instruction("OP_GREATER", offset),
            Opcode::OpLess => self.simple_instruction("OP_LESS", offset),
            Opcode::OpPrint => self.simple_instruction("OP_PRINT", offset),
            Opcode::OpPop => self.simple_instruction("OP_POP", offset),
            _ => {
                offset + 1
            }
        }
    }

    fn simple_instruction(&mut self, name: &str, offset: usize) -> usize {
        println!("{: <12}", name);
        offset + 1
    }

    fn constant_instruction(&mut self, name: &str, offset: usize, const_idx: usize) -> usize {
        let value = self.constants.get(const_idx).unwrap();
        println!("{: <12} {} '{}'", name, const_idx, value);
        offset + 1
    }
}


#[cfg(test)]
mod tests {
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

       chunk.disassemble_chunk();

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



        chunk.disassemble_chunk();

        let mut vm = VM::new();
        vm.run(&chunk);

    }

}
