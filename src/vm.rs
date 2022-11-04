use crate::chunk::{Chunk, OpCode, OpCode::*};
use crate::compiler::compile;
use crate::debug::disassemble_chunk;
use crate::error::InterpretError;
use crate::error::InterpretError::INTERPRET_RUNTIME_ERROR;
use crate::table::Table;
use crate::value::{allocate_object, ObjectHandler, Value};
use std::collections::LinkedList;
use std::rc::Rc;

#[allow(non_camel_case_types)]
#[derive(PartialEq)]
pub enum InterpretOk {
    INTERPRET_CONTINUE,
    INTERPRET_OK,
}

const STACK_MAX: usize = 256;

#[allow(non_snake_case)]
#[derive(Default)]
pub struct VM {
    pub chunk: Chunk,
    /// instruction pointer, points at bytecode about to be executed
    pub ip: usize,
    pub stack: Vec<Value>,
    pub table: Table,
    pub objects: LinkedList<Rc<dyn ObjectHandler>>,
}

impl VM {
    pub const DEBUG_EXECUTION_TRACING: bool = true;

    pub fn interpret(&mut self, source: &String) -> Result<Option<Value>, InterpretError> {
        let result = compile(source);
        match result {
            Err(_) => return Err(InterpretError::INTERPRET_COMPILE_ERROR),
            Ok(chunk) => {
                self.chunk = chunk;
                self.ip = 0; // Q

                let result = self.run();
                return result;
            }
        }
    }

    pub fn free_objects(mut self) {
        loop {
            match self.objects.pop_front() {
                None => break,
                Some(_obj) => drop(_obj),
            }
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

    // Pushes the newly created object to the objects linked list. Ensures the Value is of type object.
    fn track_object(&mut self, val: &Value) {
        if !val.is_obj() {
            panic!("Cannot track a Value which is not an Object");
        } else {
            let obj = val.as_obj().unwrap();
            let rc = Rc::clone(&obj);
            self.objects.push_front(rc);
        }
    }

    // nil and false are falsey and every other value behaves like true
    fn is_falsey(value: Value) -> bool {
        Value::is_nil(&value) || (Value::is_bool(&value) && !Value::as_bool(&value).unwrap())
    }

    /// Pops the top two values off of the stack, joins them together, and then pushes the
    /// result onto the stack. Requires the two popped values to be strings.
    fn concatenate(&mut self) {
        let b = Value::as_string(&self.pop()).unwrap();
        let a = Value::as_string(&self.pop()).unwrap();

        let cat = format!("{}{}", a, b).replace("\"", "");
        let obj = allocate_object(cat);

        self.track_object(&obj);
        self.stack.push(obj);
    }

    //Q: what happens when there are multiple chunks?
    pub fn run(&mut self) -> Result<Option<Value>, InterpretError> {
        // if debug flag enabled, print each instruction before execution
        if VM::DEBUG_EXECUTION_TRACING {
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
                    //changed in the global variable chapter
                    return if let Some(v) = self.stack.pop() {
                        println!("chunk result: {:?}", v);
                        Ok(Some(v))
                    } else {
                        //println!("Stack is empty, nothing to pop");
                        Ok(None)
                    };
                }
                OP_CONSTANT(value) => {
                    let value: Value = value.clone();
                    self.stack.push(value);
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
                }
                OP_EQUAL => {
                    let a: Value = self.pop();
                    let b: Value = self.pop();
                    self.push(Value::bool_val(Value::values_equal(a, b)));
                    Ok(())
                }
                OP_POP => {
                    self.pop();
                    Ok(())
                }
                OP_DEFINE_GLOBAL(index) => {
                    let name = self.chunk.get_constant_name(&index).unwrap();
                    let value = self.pop();
                    self.table.insert(name, value);
                    Ok(())
                }
                OP_GET_GLOBAL(index) => {
                    let key = self.chunk.get_constant_name(&index).unwrap();
                    let table_value = self.table.get(key.as_str());
                    match table_value {
                        Some(value) => {
                            let stack_value = value.clone();
                            self.push(stack_value);
                            Ok(())
                        }
                        None => {
                            eprint!("Undefined variable: {}", key);
                            Err(INTERPRET_RUNTIME_ERROR)
                        }
                    }
                }
                // TODO: debug, do I ever set the value
                OP_SET_GLOBAL(index) => {
                    let key = self.chunk.get_constant_name(&index).unwrap();
                    let table_value = self.table.get(key.as_str());
                    match table_value {
                        None => {
                            eprintln!("Undefined variable: {}", key);
                            Err(INTERPRET_RUNTIME_ERROR)
                        }
                        _ => {
                            let updated_value = self.peek(0).unwrap().clone();
                            self.table.delete(key.as_str());
                            let _ = self.table.insert(key, updated_value);
                            Ok(())
                        }
                    }
                }
                OP_FALSE => {
                    self.push(Value::bool_val(false));
                    Ok(())
                }
                OP_GREATER => binary_operator(self, '>'),
                OP_LESS => binary_operator(self, '<'),
                OP_ADD => {
                    if Value::is_string(self.peek(0).unwrap())
                        && Value::is_string(self.peek(1).unwrap())
                    {
                        self.concatenate();
                        Ok(())
                    } else if Value::is_number(self.peek(0).unwrap())
                        && Value::is_number(self.peek(1).unwrap())
                    {
                        let b: f32 = Value::as_number(&self.stack.pop().unwrap()).unwrap();
                        let a: f32 = Value::as_number(&self.stack.pop().unwrap()).unwrap();
                        self.push(Value::number_value(a + b));
                        Ok(())
                    } else {
                        eprintln!("Operands must be two numbers or two strings.");
                        Err(INTERPRET_RUNTIME_ERROR)
                    }
                    //binary_operator(self, '+')
                }
                OP_SUBTRACT => binary_operator(self, '-'),
                OP_MULTIPLY => binary_operator(self, '*'),
                OP_DIVIDE => binary_operator(self, '/'),
                OP_DEBUG => {
                    unimplemented!()
                }
                OP_PRINT => {
                    println!("{:?}", self.pop());
                    Ok(())
                }
            };

            if let Err(e) = result {
                return Err(e);
            }
        }
    }

    fn read_byte(&mut self) -> OpCode {
        let instruction = self.chunk.code.get(self.ip).unwrap().clone();
        self.ip += 1;
        return instruction;
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
