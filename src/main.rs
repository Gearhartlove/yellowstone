#![allow(warnings)]
extern crate core;

use crate::chunk::Chunk;
use crate::debug::disassemble_chunk;
use crate::vm::VM;
use std::env;
use std::fs;
use std::io::Write;
use std::io::stdout;

mod chunk;
mod compiler;
mod debug;
mod error;
mod scanner;
mod table;
mod util;
mod value;
mod vm;

fn main() {
    let args: Vec<String> = env::args().collect();
    let vm = VM::default();

    match args.len() {
        1 => repl(vm),
        2 => run_file(vm, &args[1]),
        _ => {
            println!("Usage: clox [path]")
        }
    }
}

fn run_file(mut vm: VM, path: &String) {
    let source =
        fs::read_to_string(path).expect(format!("The file at {path} does not exist").as_str());

    println!("{}", source);
    let result = vm.interpret(&source);

    match result {
        Err(e) => {
            println!("{:?}", e)
        }
        Ok(_) => {}
    }

    vm.free_objects();
}

fn repl(mut vm: VM) {
    println!("[yellowstone repl]");
    println!("(type `exit` or `quit` to stop session)");

    let mut line = String::new();
    loop {
        print!(">> ");
        let _ = stdout().flush(); // Flushed the input buffering. Used for formatting.

        // request std input stream and print result
        match std::io::stdin().read_line(&mut line) {
            Ok(_) => match line.as_str() {
                "exit" | "Exit" | "Quit" | "quit" | "q" => break,
                _ => {}
            },
            Err(error) => println!("> error: {error}"),
        }

        let result = &vm.interpret(&line);
        line.clear(); // clear buffer for next repl
    }

    vm.free_objects();
}
