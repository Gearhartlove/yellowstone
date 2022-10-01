use std::fmt::{Debug, Formatter};
use crate::error::InterpretError;

#[repr(C)]
#[derive(Clone, Copy)]
pub struct Value {
    kind: ValueKind,
    u: ValueUnion,
}

impl PartialEq for Value {
    fn eq(&self, other: &Self) -> bool {
        return if self.is_number() && other.is_number() {
            let a = self.as_number().unwrap();
            let b = self.as_number().unwrap();
            a == b
        } else if self.is_nil() && other.is_nil() {
            let a = self.as_nil().unwrap();
            let b = self.as_nil().unwrap();
            a == b
        } else if self.is_bool() && other.is_number() {
            let a = self.as_bool().unwrap();
            let b = self.as_bool().unwrap();
            a == b
        } else {
            false
        }
    }

    fn ne(&self, other: &Self) -> bool {
        todo!()
    }
}

impl Debug for Value {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> std::fmt::Result {
        unsafe {
            match self {
                Value { kind: ValueKind::ValBool, u: ValueUnion { b } } => {
                    formatter.debug_struct("Value")
                        .field("bool_value", b)
                        .finish()
                }
                Value { kind: ValueKind::ValNil, u: ValueUnion { f } } => {
                    formatter.debug_struct("Value")
                        .field("nil_val", f)
                        .finish()
                }
                Value { kind: ValueKind::ValNumber, u: ValueUnion { f } } => {
                    formatter.debug_struct("Value")
                        .field("number_value", f)
                        .finish()
                }
            }
        }
    }
}

#[repr(C)]
#[derive(Clone, Copy)]
pub union ValueUnion {
    f: f32,
    b: bool,
}

#[repr(u32)]
#[derive(Clone, Copy)]
pub enum ValueKind {
    ValBool,
    ValNil,
    ValNumber,
}

impl Value {
    fn print_value(self) {
        match self.kind {
            ValueKind::ValBool => { println!("{}", self.as_bool().unwrap()) },
            ValueKind::ValNil => { println!("{}", self.as_nil().unwrap()) },
            ValueKind::ValNumber => { println!("{}", self.as_number().unwrap()) },
        }
    }
    // instantiate a Value from a Rust primitive
    // primitive -> Value
    pub fn bool_val(b: bool) -> Self {
        Self {
            kind: ValueKind::ValBool,
            u: ValueUnion { b }
        }
    }

    pub fn nil_value() -> Self {
        Self {
            kind: ValueKind::ValNil,
            u: ValueUnion { f: 0. },
        }
    }

    pub fn number_value(num: f32) -> Self {
        Self {
            kind: ValueKind::ValNumber,
            u: ValueUnion { f: num }
        }
    }

    // read the rust value from the Value struct
    // Value -> primitive
    pub fn as_bool(&self) -> Result<bool, InterpretError> {
        if self.is_bool() {
            unsafe {
                Ok(self.u.b)
            }
        } else {
            Err(InterpretError::INTERPRET_RUNTIME_ERROR)
        }
    }

    pub fn as_nil(&self) -> Result<f32, InterpretError> {
        if self.is_nil() {
            unsafe {
                Ok(self.u.f)
            }
        } else {
            Err(InterpretError::INTERPRET_RUNTIME_ERROR)
        }
    }

    pub fn as_number(&self) -> Result<f32, InterpretError> {
        if self.is_number() {
            unsafe {
                Ok(self.u.f)
            }
        } else {
            Err(InterpretError::INTERPRET_RUNTIME_ERROR)
        }
    }

    // check the kind of a value and return true or false
    pub fn is_bool(&self) -> bool {
        unsafe {
            match self {
                Value { kind: ValueKind::ValBool, u: ValueUnion { b } } => true,
                _ => false,
            }
        }
    }

    pub fn is_nil(&self) -> bool {
        unsafe {
            match self {
                Value { kind: ValueKind::ValNil, u: ValueUnion { f } } => *f == 0.,
                _ => false,
            }
        }
    }

    pub fn is_number(&self) -> bool {
        unsafe {
            match self {
                Value { kind: ValueKind::ValNumber, u: ValueUnion { f } } => true,
                _ => false,
            }
        }
    }
}