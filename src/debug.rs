use std::convert::Infallible;
use crate::Chunk;
use crate::op_code::OpCode;
use crate::op_code::OpCode::*;

pub fn disassemble_chunk(chunk: &Chunk, name: &str) {
    println!("== {} ==", name);

    let mut offset: u8 = 0;
    for instruction in chunk.code.iter() {
        disassemble_instruction(instruction, &mut offset, &chunk.lines);
    }
}

fn simple_instruction(name: &str, offset: &mut u8) {
    println!("{name}");
    *offset += 1;
}

fn constant_instruction(instruction: &OpCode, offset: &mut u8) {
    if let OP_CONSTANT(constant) = instruction {
        println!("OP_CONSTANT {constant}");
        *offset += 1; // not offset += 2 because the data is
        // on the enum
    }
    else if let OP_CONSTANT_LONG(constant) = instruction {
        println!("OP_CONSTANT {constant}");
        *offset += 1; // not offset += 2 because the data is
        // on the enum
    }
    else {
        panic!("The instruction at offset {} is not a constant instruction.", offset);
    }
}

fn disassemble_instruction(instruction: &OpCode, offset: &mut u8, lines: &Vec<u8>) {
    print!("{:04}", offset);
    if *offset > 0 {
        if let Some(foo) = lines.get(*offset as usize - 1) {
            if *foo == lines[*offset as usize] {
                print!("   {} ", '|');
            }
        }
    }
    else {
        print!("{:4} ", lines[*offset as usize]);
    }
    match instruction {
        OP_CONSTANT_LONG(constant) => {
            constant_instruction(instruction, offset);
        }
        OP_CONSTANT(constant) => {
            constant_instruction(instruction, offset);
        }
        OP_RETURN => {
            simple_instruction("OP_RETURN", offset);
        }
        OP_DEBUG => todo!(),
    }
}