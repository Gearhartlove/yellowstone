use std::fmt;

type CompilerResult<T> = std::result::Result<_, T>;


#[derive(Debug, Clone)]
struct InterpretCompileError;
#[derive(Debup, Clone)]
struct InterpretRuntimeError;


// impl Display
impl fmt::Display for InterpretCompileError {
}
