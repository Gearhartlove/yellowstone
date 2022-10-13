use crate::chunk::{Chunk, OpCode::*, OpCode};
use crate::compiler::compile;
use crate::debug::disassemble_chunk;
use crate::error::InterpretError;
use crate::error::InterpretError::INTERPRET_RUNTIME_ERROR;
use crate::value::{Value};

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
    pub stack: Vec<Value>,
}

impl VM {
    pub const DEBUG_EXECUTION_TRACING: bool = true;

    pub fn interpret(&mut self, source: &String) -> Result<Option<Value>, InterpretError> {
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

    fn push(&mut self, value: Value) {
        self.stack.push(value);
    }
    fn pop(&mut self) -> Value {
        self.stack.pop().unwrap()
    }
    fn peek(&mut self, from_top: usize) -> Option<&Value> {
        self.stack.get(self.stack.len() - from_top - 1)
    }

    // nil and false are falsey and every other value behaves like true
    fn is_falsey(value: Value) -> bool {
        Value::is_nil(&value)
            || (Value::is_bool(&value) && !Value::as_bool(&value).unwrap())
    }

    //Q: what happens when there are multiple chunks?
    pub fn run(&mut self) -> Result<Option<Value>, InterpretError> {
        // if debug flag enabled, print each instruction before execution
        if VM::DEBUG_EXECUTION_TRACING {
            println!("           ");
            for val in self.stack.iter() {
                println!("[{:?}]", val);
            }
            disassemble_chunk(&self.chunk, "chunk");
            println!();
        }

        loop {
            let instruction = self.read_byte();
            let result = match instruction {
                OP_RETURN => {
                    return if let Some(v) = self.stack.pop() {
                        println!("chunk result: {:?}", v);
                        Ok(Some(v))
                    } else {
                        println!("Stack is empty, nothing to pop");
                        Ok(None)
                    }
                }
                OP_CONSTANT(c) => {
                    let c = c.clone();

                    self.stack.push(c);
                    Ok(())
                }
                OP_NEGATE => {
                    if !Value::is_number(self.peek(0).unwrap()) {
                        eprintln!("Operand must be a number.");
                        return Err(INTERPRET_RUNTIME_ERROR);
                    }
                    let pop_val = self.stack.pop().unwrap();
                    let mut number = pop_val.as_number().unwrap();
                    number *= -1.;
                    self.push(Value::number_value(number));
                    Ok(())
                }
                OP_NOT => {
                    if Value::is_number(self.peek(0).unwrap()) {
                        eprintln!("Operand cannot be a number.");
                        return Err(INTERPRET_RUNTIME_ERROR);
                    }
                    let val = self.pop();
                    self.push(Value::bool_val(VM::is_falsey(val)));
                    Ok(())
                }
                OP_NIL => {
                    self.push(Value::nil_value());
                    Ok(())
                }
                OP_TRUE => {
                    self.push(Value::bool_val(true));
                    Ok(())
                },
                OP_EQUAL => {
                    let a: Value = self.pop();
                    let b: Value = self.pop();
                    self.push(Value::bool_val(Value::values_equal(a,b)));
                    Ok(())
                }
                OP_FALSE => {
                    self.push(Value::bool_val(false));
                    Ok(())
                },
                OP_GREATER => {
                    binary_operator(self, '>')
                }
                OP_LESS => {
                    binary_operator(self, '<')
                }
                OP_ADD => {
                    binary_operator(self, '+')
                }
                OP_SUBTRACT => {
                    binary_operator(self, '-')
                }
                OP_MULTIPLY => {
                    binary_operator(self, '*')
                }
                OP_DIVIDE => {
                    binary_operator(self, '/')
                }
                OP_DEBUG => {
                    unimplemented!()
                }
            };

            if let Err(e) = result {
                return Err(e);
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

fn binary_operator(vm: &mut VM, op: char) -> Result<(), InterpretError> {
    if !Value::is_number(vm.peek(0).unwrap()) || !Value::is_number(vm.peek(1).unwrap()) {
        eprintln!("Operands must be numbers.");
        return Err(INTERPRET_RUNTIME_ERROR);
    }
    let b: f32 = Value::as_number(&vm.stack.pop().unwrap()).unwrap();
    let a: f32 = Value::as_number(&vm.stack.pop().unwrap()).unwrap();
    match op {
        '+' => vm.stack.push(Value::number_value(a + b)),
        '-' => vm.stack.push(Value::number_value(a - b)),
        '/' => vm.stack.push(Value::number_value(a / b)),
        '*' => vm.stack.push(Value::number_value(a * b)),
        '>' => vm.stack.push(Value::bool_val(a > b)),
        '<' => vm.stack.push(Value::bool_val(a < b)),
        _ => {
            println!("invalid operation {}", op)
        }
    }
    Ok(())
}
