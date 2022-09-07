use crate::{Chunk, disassemble_chunk};
use crate::op_code::OpCode;
use crate::op_code::OpCode::*;
use crate::scanner::{Scanner, Token};
use crate::scanner::TokenKind::TOKEN_EOF;
use crate::vm::InterpretResult::{INTERPRET_CONTINUE, INTERPRET_OK};

#[derive(PartialEq)]
pub enum InterpretResult {
    INTERPRET_CONTINUE,
    INTERPRET_OK,
    INTERPRET_COMPILE_ERROR,
    INTERPRET_RUNTIME_ERROR,
}

const STACK_MAX: usize = 256;

#[derive(Default)]
pub struct VM {
    pub chunk: Chunk,
    pub ip: usize, // instruction pointer, points at bytecode about to be executed
    pub stack: Vec<f32>,
}

impl VM {
    pub const DEBUG_EXECUTION_TRACING: bool = false;

    pub fn interpret(&mut self) -> InterpretResult {
        return self.run();
        //self.compile(&source);
        return INTERPRET_OK;
    }

    fn push(&mut self, value: f32) {
        self.stack.push(value);
    }
    fn pop(&mut self) -> f32 {
        self.stack.pop().unwrap()
    }

    pub fn compile(&mut self, source: &String) {
        let mut scanner = Scanner::from(source);
        let mut line = 0; //todo: why start at -1 in book ?
        loop {
            let token: Token = scanner.scan_token(); //todo scanner.scan_token() implementation
            if token.line != line {
                print!("{:4}", token.line);
                line = token.line;
            } else {
                print!("   | ")
            }
            // todo create a print word
            print!("{:2} ", token.kind);
            print!("'");
            for i in 0..token.length {
                unsafe {
                    let c = *token.start.add(i) as char;
                    print!("{}", c);
                }
            }
            print!("'");
            println!();

            if token.kind == TOKEN_EOF {
                break;
            }
        }
    }

    //Q: what happens when there are multiple chunks?
    pub fn run(&mut self) -> InterpretResult {
        let binary_operator = |vm: &mut VM, op: char| {
            let b: f32 = vm.stack.pop().unwrap();
            let a: f32 = vm.stack.pop().unwrap();
            match op {
                '+' => { vm.stack.push(a + b) }
                '-' => { vm.stack.push(a - b) }
                '/' => { vm.stack.push(a / b) }
                '*' => { vm.stack.push(a * b) }
                _ => { println!("invalid operation {}", op) }
            }
        };

        loop {
            // if debug flag enabled, print each instruction before execution
            if VM::DEBUG_EXECUTION_TRACING {
                println!("           ");
                for val in self.stack.iter() {
                    println!("[{}]", val);
                }
                disassemble_chunk(&self.chunk, "chunk")
            }

            let instruction = self.read_byte();
            let result = match instruction {
                OP_RETURN => {
                    if let Some(v) = self.stack.pop() {
                        println!("{}", v);
                    } else {
                        println!("Stack is empty, nothing to pop")
                    }
                    INTERPRET_OK
                }
                OP_CONSTANT(c) => {
                    // println!("{c}");
                    let c = c.clone();
                    self.stack.push(c);
                    INTERPRET_CONTINUE
                }
                OP_NEGATE => {
                    let mut pop_val = self.stack.pop().unwrap();
                    self.stack.push(
                        pop_val * -1. // negating
                    );
                    INTERPRET_CONTINUE //is this right?
                }
                OP_ADD => {
                    binary_operator(self, '+');
                    INTERPRET_CONTINUE
                }
                OP_SUBTRACT => {
                    binary_operator(self, '-');
                    INTERPRET_CONTINUE
                }
                OP_MULTIPLY => {
                    binary_operator(self, '*');
                    INTERPRET_CONTINUE
                }
                OP_DIVIDE => {
                    binary_operator(self, '/');
                    INTERPRET_CONTINUE
                }
                OP_CONSTANT_LONG(_) => { unimplemented!() }
                OP_DEBUG => { unimplemented!() }
            };

            if result == INTERPRET_OK {
                return result;
            }
        }
    }

    fn read_byte(&mut self) -> &OpCode {
        let instruction = self.chunk.code.get(self.ip);
        match instruction {
            None => {unimplemented!()}
            Some(instruction) => {
                self.ip += 1;
                return instruction;
            }
        }
    }

    pub fn with_chunk(mut self, chunk: Chunk) -> Self {
        self.chunk = chunk;
        return self;
    }
}
