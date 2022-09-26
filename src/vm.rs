use crate::op_code::{OpCode, OpCode::*};
use crate::chunk::Chunk;
use crate::compiler::compile;
use crate::debug::disassemble_chunk;

#[allow(non_camel_case_types)]
#[derive(PartialEq, Debug)]
pub enum InterpretError {
    INTERPRET_COMPILE_ERROR,
    INTERPRET_RUNTIME_ERROR,
}

#[allow(non_camel_case_types)]
#[derive(PartialEq)]
pub enum InterpretOk{
    INTERPRET_CONTINUE,
    INTERPRET_OK,
}

const STACK_MAX: usize = 256;

#[allow(non_snake_case)]
#[derive(Default)]
pub struct VM {
    pub chunk: Chunk,
    pub ip: usize,
    // instruction pointer, points at bytecode about to be executed
    pub stack: Vec<f32>,
}

impl VM {
    pub const DEBUG_EXECUTION_TRACING: bool = false;

    pub fn interpret(&mut self, source: &String) -> Result<InterpretOk, InterpretError> {
        let result = compile(source);
        match result {
            Err(_) => {
                return Err(InterpretError::INTERPRET_COMPILE_ERROR)
            },
            Ok(chunk) => {
                self.chunk = chunk;
                self.ip = 0; // Q

                let result = self.run();

                return result;
            },
        }
    }

    fn push(&mut self, value: f32) {
        self.stack.push(value);
    }
    fn pop(&mut self) -> f32 {
        self.stack.pop().unwrap()
    }

    //Q: what happens when there are multiple chunks?
    pub fn run(&mut self) -> Result<InterpretOk, InterpretError> {
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
                    Ok(InterpretOk::INTERPRET_OK)
                }
                OP_CONSTANT(c) => {
                    // println!("{c}");
                    let c = c.clone();
                    self.stack.push(c);
                    Ok(InterpretOk::INTERPRET_OK)
                }
                OP_NEGATE => {
                    let pop_val = self.stack.pop().unwrap();
                    self.stack.push(
                        pop_val * -1., // negating
                    );
                    Ok(InterpretOk::INTERPRET_OK)
                }
                OP_ADD => {
                    binary_operator(self, '+');
                    Ok(InterpretOk::INTERPRET_OK)
                }
                OP_SUBTRACT => {
                    binary_operator(self, '-');
                    Ok(InterpretOk::INTERPRET_OK)
                }
                OP_MULTIPLY => {
                    binary_operator(self, '*');
                    Ok(InterpretOk::INTERPRET_OK)
                }
                OP_DIVIDE => {
                    binary_operator(self, '/');
                    Ok(InterpretOk::INTERPRET_OK)
                }
                OP_CONSTANT_LONG(_) => {
                    unimplemented!()
                }
                OP_DEBUG => {
                    unimplemented!()
                }
            };

            if let Ok(InterpretOk::INTERPRET_OK) = result {
                return result;
            }
        }
    }

    fn read_byte(&mut self) -> &OpCode {
        let instruction = self.chunk.code.get(self.ip);
        match instruction {
            None => {
                unimplemented!()
            }
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

fn binary_operator(vm: &mut VM, op: char) {
    let b: f32 = vm.stack.pop().unwrap();
    let a: f32 = vm.stack.pop().unwrap();
    match op {
        '+' => vm.stack.push(a + b),
        '-' => vm.stack.push(a - b),
        '/' => vm.stack.push(a / b),
        '*' => vm.stack.push(a * b),
        _ => {
            println!("invalid operation {}", op)
        }
    }
}
