use crate::chunk::get_line;
use crate::chunk::OpCode::*;
use crate::chunk::*;

pub fn disassemble_chunk(chunk: &Chunk, name: &str) {
    println!("== {} ==", name);

    let mut offset: u32 = 0;
    for instruction in chunk.code.iter() {
        disassemble_instruction(instruction, &mut offset, &chunk.lines, chunk);
    }
}

fn simple_instruction(name: &str, offset: &mut u32) {
    println!("{name}");
    *offset += 1;
}

fn constant_instruction(instruction: &OpCode, offset: &mut u32) {
    if let OP_CONSTANT(const_val) = instruction {
        println!("OP_CONSTANT {:?}", const_val);
        *offset += 1;
    } else {
        panic!(
            "The instruction at offset {} is not a constant instruction.",
            offset
        );
    }
}

fn global_instruction(instruction: &OpCode, offset: &mut u32, chunk: &Chunk) {
    match instruction {
        OP_SET_GLOBAL(index) => {
            println!(
                "OP_SET_GLOBAL | {:?} {:?}",
                chunk.get_constant_name(index),
                chunk.constants.get(*index).unwrap()
            )
        }
        OP_GET_GLOBAL(index) => {
            println!("OP_GET_GLOBAL {:?}", chunk.constants.get(*index).unwrap())
        }
        OP_DEFINE_GLOBAL(index) => {
            println!(
                "OP_DEFINE_GLOBAL {:?}",
                chunk.constants.get(*index).unwrap()
            )
        }
        _ => {
            panic!(
                "The instruction at offset {} is not a constant instruction.",
                offset
            );
        }
    }
}

/// The slot number of the local variable. b/c the local variable's name never leaves the
/// compiler to make it into the chunk at all.
fn local_instruction(name: &'static str, offset: &mut u32, chunk: &Chunk) {
    println!("{} {}", name, offset); // TODO: is this the right thing to print?
    *offset += 1 
}

fn disassemble_instruction(instruction: &OpCode, offset: &mut u32, lines: &String, chunk: &Chunk) {
    print!("{:04}", offset);
    let line = get_line(offset, &lines);
    if line == "same".to_string() {
        print!("   | ");
    } else {
        print!("{:>4} ", line);
    }

    match instruction {
        OP_CONSTANT(_) => constant_instruction(instruction, offset),
        OP_DEFINE_GLOBAL(_) => global_instruction(instruction, offset, chunk),
        OP_GET_GLOBAL(_) => global_instruction(instruction, offset, chunk),
        OP_SET_GLOBAL(_) => global_instruction(instruction, offset, chunk),
        OP_SET_LOCAL(_) => local_instruction("OP_SET_LOCAL", offset, chunk),
        OP_GET_LOCAL(_) => local_instruction("OP_GET_LOCAL", offset, chunk),
        OP_TRUE => simple_instruction("OP_TRUE", offset),
        OP_NIL => simple_instruction("OP_NIL", offset),
        OP_FALSE => simple_instruction("OP_FALSE", offset),
        OP_EQUAL => simple_instruction("OP_EQUAL", offset),
        OP_GREATER => simple_instruction("OP_GREATER", offset),
        OP_LESS => simple_instruction("OP_LESS", offset),
        OP_RETURN => simple_instruction("OP_RETURN", offset),
        OP_NEGATE => simple_instruction("OP_NEGATE", offset),
        OP_NOT => simple_instruction("OP_NOT", offset),
        OP_ADD => simple_instruction("OP_ADD", offset),
        OP_SUBTRACT => simple_instruction("OP_SUBTRACT", offset),
        OP_MULTIPLY => simple_instruction("OP_MULTIPLY", offset),
        OP_DIVIDE => simple_instruction("OP_DIVIDE", offset),
        OP_PRINT => simple_instruction("OP_PRINT", offset),
        OP_POP => simple_instruction("OP_POP", offset),
        OP_DEBUG => {
            todo!()
        }
    }
}
