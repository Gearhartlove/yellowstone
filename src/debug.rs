use crate::Chunk;
use crate::chunk::get_line;
use crate::op_code::OpCode;
use crate::op_code::OpCode::*;

pub fn disassemble_chunk(chunk: &Chunk, name: &str) {
    println!("== {} ==", name);

    let mut offset: u32 = 0;
    for instruction in chunk.code.iter() {
        disassemble_instruction(instruction, &mut offset, &chunk.lines);
    }
}

fn simple_instruction(name: &str, offset: &mut u32) {
    println!("{name}");
    *offset += 1;
}

fn constant_instruction(instruction: &OpCode, offset: &mut u32) {
    if let OP_CONSTANT(constant) = instruction {
        println!("OP_CONSTANT {constant}");
        *offset += 1;
    } else if let OP_CONSTANT_LONG(constant) = instruction {
        println!("OP_CONSTANT {constant}");
        *offset += 1;
    } else {
        panic!("The instruction at offset {} is not a constant instruction.", offset);
    }
}

fn disassemble_instruction(instruction: &OpCode, offset: &mut u32, lines: &String) {
    print!("{:04}", offset);
    let line = get_line(offset, &lines);
    if line == "same".to_string() {
        print!("   | ");
    } else {
        print!("{:>4} ", line);
    }

    match instruction {
        OP_CONSTANT_LONG(constant) => { constant_instruction(instruction, offset); }
        OP_CONSTANT(constant) => { constant_instruction(instruction, offset); }
        OP_RETURN => { simple_instruction("OP_RETURN", offset); }
        OP_NEGATE => { simple_instruction("OP_NEGATE", offset); }
        OP_ADD => { simple_instruction("OP_ADD", offset); }
        OP_SUBTRACT => { simple_instruction("OP_SUBTRACT", offset); }
        OP_MULTIPLY => { simple_instruction("OP_MULTIPLY", offset); }
        OP_DIVIDE => { simple_instruction("OP_DIVIDE", offset); }
        OP_DEBUG => todo!(),
    }
}