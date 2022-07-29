extern crate core;

use std::borrow::BorrowMut;
use std::mem;
use std::mem::{size_of, size_of_val};
use crate::chunk::Chunk;
use crate::debug::disassemble_chunk;
use crate::op_code::OpCode::*;
use crate::stack::Stack;
use crate::vm::VM;

mod chunk;
mod common;
mod memory;
mod debug;
mod value;
mod op_code;
mod vm;
mod stack;

fn main() {
    let mut chunk = Chunk::default();
    let foo = 1.2;
    let bar = 3.4;

    let sop = 5.6;

    chunk.add_constant(foo);
    chunk.write_chunk(OP_CONSTANT(foo), 123); // 1.2
    chunk.write_chunk(OP_CONSTANT(bar), 123); // 3.4
    chunk.write_chunk(OP_ADD, 123); // a(1.2) + b(3.4) = 4.6
    chunk.write_chunk(OP_CONSTANT(sop), 123); // 5.6
    chunk.write_chunk(OP_DIVIDE, 123); // a(4.6) + b(5.6) = 0.8
    chunk.write_chunk(OP_NEGATE, 123); // -0.8
    chunk.write_chunk(OP_RETURN, 123); // -0.8
    disassemble_chunk(&chunk, "test chunk");

    let mut vm: VM = VM {
        chunk: Some(&chunk),
        ip: 0,
        stack: Stack::default()
    };

    vm.run();
}