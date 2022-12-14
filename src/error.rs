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
        let message = match self {
            InterpretError::COMPILE_ERROR => "COMPILE_ERROR",
            InterpretError::RUNTIME_ERROR => "RUNTIME_ERROR",
            InterpretError::RUNTIME_UNRECOGNIZED_VARIABLE_ERROR => {
                "RUNTIME_UNRECOGNIZED_VARIABLE_ERROR"
            }
            InterpretError::RUNTIME_ASSERT_ERROR => "RUNTIME_ASSERT_ERROR",
        };

        write!(f, "{message}")
    }
}

impl Error for InterpretError {}
