use crate::chunk::{Chunk, ChunkWriterTrait};
use crate::opcode::Opcode;
use crate::value::Value;
use std::io::Write;

#[derive(Debug, Clone, PartialEq)]
pub enum FunctionType {
    Function,
    Script,
}

#[derive(Debug, Clone)]
pub struct ObjectFunction {
    ftype: FunctionType,
    pub(crate) chunk: Chunk,
    pub(crate) arity: u8,
    pub name: String,
}

impl ObjectFunction {
    pub fn new(ftype: FunctionType, name: String) -> Self {
        ObjectFunction {
            name,
            ftype,
            chunk: Chunk::new(),
            arity: 0,
        }
    }
}

impl PartialEq for ObjectFunction {
    fn eq(&self, other: &Self) -> bool {
        self.arity == other.arity && self.name == other.name && self.ftype == other.ftype
    }
}

impl ChunkWriterTrait for ObjectFunction {
    ///
    ///
    fn emit_byte(&mut self, byte: Opcode, line: isize) {
        self.write_chunk(byte, line);
    }
    ///
    ///
    fn emit_bytes(&mut self, byte1: Opcode, byte2: Opcode, line: isize) {
        self.emit_byte(byte1, line);
        self.emit_byte(byte2, line);
    }
    ///
    ///
    fn emit_return(&mut self, line: isize) {
        self.emit_byte(Opcode::OpNil, line);
        self.emit_byte(Opcode::OpReturn, line);
    }
    ///
    ///
    fn emit_constant(&mut self, value: Value, line: isize) {
        let idx = self.make_constant(value);
        self.emit_byte(Opcode::OpConstant(idx), line)
    }
    fn write_chunk(&mut self, byte: Opcode, _line: isize) {
        self.chunk.write_chunk(byte);
    }
    fn make_constant(&mut self, value: Value) -> usize {
        self.chunk.add_constant(value)
    }
    fn disassemble_chunk(&mut self, writer: &mut Box<dyn Write>) {
        self.chunk.disassemble_chunk(writer)
    }

    fn len(&self) -> usize {
        self.chunk.op_codes.len()
    }

    fn replace_opcode(&mut self, index: usize, bytes: Opcode) {
        self.chunk.replace_opcode(index, bytes);
    }
}
