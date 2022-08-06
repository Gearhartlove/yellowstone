extern crate core;

use std::borrow::BorrowMut;
use std::{env, mem};
use std::mem::{size_of, size_of_val};
use std::process::exit;
use std::fs;
use crate::chunk::Chunk;
use crate::debug::disassemble_chunk;
use crate::InterpretResult::{INTERPRET_COMPILE_ERROR, INTERPRET_RUNTIME_ERROR};
use crate::op_code::OpCode::*;
use crate::stack::Stack;
use crate::vm::{InterpretResult, VM};

mod chunk;
mod common;
mod memory;
mod debug;
mod value;
mod op_code;
mod vm;
mod stack;
mod scanner;
mod util;


fn main() {
    let args: Vec<String> = env::args().collect();

    let mut vm: VM = VM {
        chunk: None,
        ip: 0,
        stack: Stack::default()
    };

    if args.len() == 1 {
        repl(&mut vm);
    } else if args.len() == 2 {
        run_file(args.get(1).unwrap(), &mut vm)
    } else {
        eprint!("Usage: clox [path]\n");
        exit(64);
    }
}

fn run_file(path: &str, vm: &mut VM) {
    let source = fs::read_to_string(path)
        .expect(format!("The file at {path} does not exist").as_str());

    let result = vm.interpret(&source);

    // couldo: custom exit 'enums'
    // ref: https://blog.rust-lang.org/2022/05/19/Rust-1.61.0.html
    if result == INTERPRET_COMPILE_ERROR { exit(65) }
    if result == INTERPRET_RUNTIME_ERROR { exit(70) }
}

fn repl(vm: &mut VM) {
    let mut line = String::new();
    loop {
        // request std input stream and print result
        match std::io::stdin().read_line(&mut line) {
            Ok(_) => {
                print!("> {line}");
            }
            Err(error) => println!("> error: {error}"),
        }
        vm.interpret(&line);
        line.clear(); // clear buffer for next repl
    }
}