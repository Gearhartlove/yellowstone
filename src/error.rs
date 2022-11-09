// #[derive(Clone, Copy)]
// pub enum YSErrorType {
//     InterpretError,
//     CompileError,
//     RuntimeError,
// }

#[allow(non_camel_case_types)]
#[derive(PartialEq, Debug)]
pub enum InterpretError {
    INTERPRET_COMPILE_ERROR,
    INTERPRET_RUNTIME_ERROR,
    INTERPRET_RUNTIME_UNRECOGNIZED_VARIABLE_ERROR,
}
