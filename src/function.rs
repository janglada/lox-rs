use crate::chunk::{Chunk, ChunkIndex, ChunkWriterTrait};
use crate::opcode::Opcode;
use crate::value::Value;
use std::fmt;
use std::io::Write;

#[derive(Debug, Clone, PartialEq)]
pub enum FunctionType {
    Function,
    Script,
}

impl fmt::Display for FunctionType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Clone)]
pub struct ObjectFunction {
    ftype: FunctionType,
    pub(crate) chunk_index: ChunkIndex,
    pub(crate) arity: u8,
    pub(crate) upvalue_count: usize,
    pub name: String,
}

impl ObjectFunction {
    ///
    ///
    ///
    pub fn new(ftype: FunctionType, name: String, chunk_index: usize) -> Self {
        ObjectFunction {
            name,
            ftype,
            chunk_index,
            upvalue_count : 0,
            arity: 0,
        }
    }
}

impl PartialEq for ObjectFunction {
    fn eq(&self, other: &Self) -> bool {
        self.arity == other.arity && self.name == other.name && self.ftype == other.ftype
    }
}

impl fmt::Debug for ObjectFunction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            // "Object function {}, type= {}, arity ={},  chunk = {:?}]",
            "Object function '{}',  arity ={}",
            self.name,
            // self.ftype.to_string(),
            self.arity,
            //  self.chunk
        )
    }
}
//
// impl ChunkWriterTrait for ObjectFunction {
//     ///
//     ///
//     fn emit_byte(&mut self, chunk_index: ChunkIndex, byte: Opcode, line: isize) {
//         self.write_chunk(chunk_index,byte, line);
//     }
//     ///
//     ///
//     fn emit_bytes(&mut self, chunk_index: ChunkIndex, byte1: Opcode, byte2: Opcode, line: isize) {
//         self.emit_byte(chunk_index,byte1, line);
//         self.emit_byte(chunk_index,byte2, line);
//     }
//     ///
//     ///
//     fn emit_return(&mut self,, line: isize) {
//         self.emit_byte(chunk_index,Opcode::OpNil, line);
//         self.emit_byte(chunk_index,Opcode::OpReturn, line);
//     }
//     ///
//     ///
//     fn emit_constant(&mut self, chunk_index: ChunkIndex, value: Value, line: isize) {
//         let idx = self.make_constant(chunk_index,value);
//         self.emit_byte(chunk_index,Opcode::OpConstant(idx), line)
//     }
//     fn write_chunk(&mut self, chunk_index: ChunkIndex, byte: Opcode, _line: isize) {
//         self
//         self.chunk.write_chunk(byte);
//     }
//     fn make_constant(&mut self, chunk_index: ChunkIndex, value: Value) -> usize {
//         self.chunk.add_constant(value)
//     }
//     fn disassemble_chunk(&mut self, chunk_index: ChunkIndex, writer: &mut Box<dyn Write>) {
//         self.chunk.disassemble_chunk(writer);
//         self.chunk.disassemble_chunk_constants(writer);
//     }
//
//     fn len(&self, chunk_index: ChunkIndex) -> usize {
//         self.chunk.op_codes.len()
//     }
//
//     fn replace_opcode(&mut self, chunk_index: ChunkIndex, index: usize, bytes: Opcode) {
//         self.chunk.replace_opcode(index, bytes);
//     }
// }
