use std::{error::Error, fmt::Display};


#[allow(non_camel_case_types)]
#[derive(PartialEq, Debug, Clone)]
pub enum InterpretError {
    COMPILE_ERROR,
    RUNTIME_ERROR,
    RUNTIME_UNRECOGNIZED_VARIABLE_ERROR,
    RUNTIME_ASSERT_ERROR,
}

impl Display for InterpretError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "")
    }
}

impl Error for InterpretError {}

