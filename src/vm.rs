use std::ops::Neg;
use crate::{Chunk, disassemble_chunk};
use crate::op_code::OpCode;
use crate::op_code::OpCode::*;
use crate::scanner::{Scanner, Token};
use crate::scanner::TokenKind::TOKEN_EOF;
use crate::vm::InterpretResult::{INTERPRET_CONTINUE, INTERPRET_OK};
use crate::stack::Stack;
use crate::value::Value;

pub const DEBUG_TRACE_EXECUTION: bool = false;

pub struct VM<'a> {
    pub chunk: Option<&'a Chunk>,
    pub ip: u8,
    // instruction pointer, points to the next byte of code to be used
    pub stack: Stack<Value>,
}

#[derive(PartialEq)]
pub enum InterpretResult {
    INTERPRET_CONTINUE,
    INTERPRET_OK,
    INTERPRET_COMPILE_ERROR,
    INTERPRET_RUNTIME_ERROR,
}

impl VM<'_> {
    pub fn interpret(&mut self, source: &String) -> InterpretResult {
        self.compile(&source);
        return INTERPRET_OK;
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
            let b: f32 = vm.stack.pop().unwrap().value;
            let a: f32 = vm.stack.pop().unwrap().value;
            match op {
                '+' => { vm.stack.push(Value::from(a + b)) }
                '-' => { vm.stack.push(Value::from(a - b)) }
                '/' => { vm.stack.push(Value::from(a / b)) }
                '*' => { vm.stack.push(Value::from(a * b)) }
                _ => { println!("invalid operation {}", op) }
            }
        };

        loop {
            // if debug flag enabled, print each instruction before execution
            if DEBUG_TRACE_EXECUTION {
                println!("           ");
                for slot in self.stack.stack.iter() {
                    println!("[{}]", slot.value);
                }
                disassemble_chunk(self.chunk.unwrap(), "chunk")
            }

            let instruction = self.read_byte();
            let result = match instruction {
                OP_CONSTANT(c) => {
                    // couldo: not push another Value struct (one already exists on the chunk)
                    self.stack.push(Value { value: *c });
                    INTERPRET_CONTINUE
                }
                OP_NEGATE => {
                    let mut pop_val = self.stack.pop().unwrap().value;
                    let neg_val = -pop_val; // negated value is the opposite of the popped val
                    self.stack.push(
                        Value { value: neg_val }
                    );
                    INTERPRET_CONTINUE //is this right?
                }
                OP_RETURN => {
                    if let Some(v) = self.stack.pop() {
                        println!("{}", v);
                    } else {
                        println!("Stack is empty, nothing to pop")
                    }
                    INTERPRET_OK
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
            self.increment_ip();
            if result == INTERPRET_OK {
                return result;
            }
        }
    }

    fn increment_ip(&mut self) {
        self.ip += 1;
    }

    fn read_byte(&self) -> &OpCode {
        let byte = self.chunk.unwrap().code.get(self.ip as usize).unwrap();
        return byte;
    }

    fn reset_stack(&mut self) {
        self.stack.reset()
    }
}
