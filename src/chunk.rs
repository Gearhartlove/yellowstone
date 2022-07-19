use std::convert::Infallible;
use crate::op_code::OpCode;

pub struct Chunk {
    pub code: Vec<OpCode>,
    pub constants: Vec<f32>, // couldo: abstract Vec<u8> into struct and give it functionality
    pub lines: Vec<u8>,
}

impl Chunk {
    pub fn write_chunk(&mut self, op: OpCode, line: u8) {
        self.code.push(op);
        self.lines.push(line);
    }
    // todo: change from f32 to a generic data structure
    pub fn add_constant(&mut self, constant: f32) {
        self.constants.push(constant);
    }
}

impl Default for Chunk {
    fn default() -> Self {
        Chunk {
            code: Vec::default(),
            constants: Vec::default(),
            lines: Vec::default(),
        }
    }
}

