use std::convert::Infallible;
use crate::{Chunk, OpCode};
use OpCode::*;

pub fn disassemble_chunk(chunk: Chunk, name: &str) {
    println!("== {} ==", name);

    let mut offset: u8 = 0;
    for instruction in chunk.code {
        disassemble_instruction(instruction, &mut offset);
    }
}

fn simple_instruction(name: &str, offset: &mut u8) {
    println!("{name}");
    *offset += 1;
}

pub fn disassemble_instruction(instruction: OpCode, offset: &mut u8) {
    print!("{offset} ");
    match instruction {
        OP_RETURN => {
            simple_instruction("OP_RETURN", offset);
        }
        OP_DEBUG => todo!()
    }
}