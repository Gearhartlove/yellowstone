extern crate core;

use std::env;
use crate::chunk::Chunk;
use crate::debug::disassemble_chunk;
use crate::vm::VM;
use std::fs;

mod chunk;
mod compiler;
mod debug;
mod vm;
mod scanner;
mod util;
mod value;
mod error;


fn main() {
    let args: Vec<String> = env::args().collect();
    let vm = VM::default();

    match args.len() {
        1 => { repl(vm) },
        2 => { run_file(vm, &args[1]) },
        _ => { println!("Usage: clox [path]") }
    }
}

fn run_file(mut vm: VM, path: &String) {
    let source = fs::read_to_string(path)
        .expect(format!("The file at {path} does not exist").as_str());

    println!("{source}");
    let result = vm.interpret(&source);
    
    match result {
        Err(e) => { println!("{:?}", e) },
        Ok(_) => {},
    }

    vm.free_objects();
}

fn repl(mut vm: VM) {
    let mut line = String::new();
    loop {
        // request std input stream and print result
        match std::io::stdin().read_line(&mut line) {
            Ok(_) => {
                match line.as_str() {
                    "exit" | "Exit" | "Quit" | "quit" | "q" => { break }
                    _ => { print!("> {line}") }
                }
            },
            Err(error) => println!("> error: {error}"),
        }

        let result = &vm.interpret(&line);
        line.clear(); // clear buffer for next repl
    }

    vm.free_objects();
}
