extern crate core;

use std::env;
use crate::chunk::Chunk;
use crate::debug::disassemble_chunk;
use crate::vm::VM;

mod chunk;
mod common;
mod debug;
mod op_code;
mod vm;
mod scanner;
mod util;


fn main() {
    let args: Vec<String> = env::args().collect();
    let mut vm = VM::default();

    match args.len() {
        1 => { repl(vm) },
        2 => { run_file(vm, args[1]) },
        _ => { println!("Usage: clox [path]") }
    }

    // let args: Vec<String> = env::args().collect();
    //
    // let mut vm: VM = VM {
    //     chunk: None,
    //     ip: 0,
    //     stack: Stack::default()
    // };
    //
    // if args.len() == 1 {
    //     repl(&mut vm);
    // } else if args.len() == 2 {
    //     run_file(args.get(1).unwrap(), &mut vm)
    // } else {
    //     eprint!("Usage: clox [path]\n");
    //     exit(64);
    // }
}

fn run_file(vm: mut VM path: String) {
    let source = fs::read_to_string(path)
        .expect(format!("The file at {path} does not exist").as_str());
    
    let result = vm.interpret(&source);
    
    // couldo: custom exit 'enums'
    // ref: https://blog.rust-lang.org/2022/05/19/Rust-1.61.0.html
    match result {
        Err(e) => { println!(Err(e.into())) },
        Ok(_) => {},
    }
}

fn repl(vm: mut VM) {
    let mut line = String::new();
    loop {
        // request std input stream and print result
        match std::io::stdin().read_line(&mut line) {
            Ok(_) => {
                print!("> {line}");
            }
            Err(error) => println!("> error: {error}"),
        }
        &vm.interpret(&line);
        line.clear(); // clear buffer for next repl
    }
}
