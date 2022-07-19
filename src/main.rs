extern crate core;

use std::borrow::BorrowMut;
use std::mem;
use std::mem::{size_of, size_of_val};
use crate::chunk::Chunk;
use crate::debug::disassemble_chunk;
use crate::op_code::OpCode::*;

mod chunk;
mod common;
mod memory;
mod debug;
mod value;
mod op_code;

fn main() {
    let mut chunk = Chunk::default();
    let foo = 1.5;
    let bar = 3.4;
    chunk.add_constant(foo);
    chunk.write_chunk(OP_CONSTANT(foo), 123);
    chunk.write_chunk(OP_CONSTANT(bar), 123);
    chunk.write_chunk(OP_RETURN, 123);

    disassemble_chunk(chunk, "test chunk");
}