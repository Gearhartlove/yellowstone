#[repr(C)]
pub struct Value {
    kind: ValueKind,
    u: ValueUnion,
}

#[repr(C)]
pub union ValueUnion {
    f: f32,
    b: bool,
}

#[repr(u32)]
pub enum ValueKind {
    ValBool,
    ValNil,
    ValNumber,
}

impl Value {
    // instantiate a Value from a Rust primitive
    // primitive -> Value
    fn bool_val(b: bool) -> Self {
        Self {
            kind: ValueKind::ValBool,
            u: ValueUnion { b }
        }
    }

    fn nil_value() -> Self {
        Self {
            kind: ValueKind::ValNil,
            u: ValueUnion { f: 0. },
        }
    }

    fn number_value(num: f32) -> Self {
        Self {
            kind: ValueKind::ValBool,
            u: ValueUnion { f: num }
        }
    }

    // read the rust value from the Value struct
    // Value -> primitive
    fn as_bool(&self) -> bool {
        unsafe {
            match self {
                Value { kind: ValueKind::ValBool, u: ValueUnion { b } } => *b,
                _ => { panic!("Error getting bool primitive from Value struct.") },
            }
        }
    }

    fn as_nil(&self) -> f32 {
        unsafe {
            match self {
                Value { kind: ValueKind::ValNil, u: ValueUnion { f } } => {
                    if f == 0. {
                        return *f;
                    } else {
                        panic!("Union not equal to 0. in nil conversion.")
                    }
                },
                _ => { panic!("Error getting nil primitive from Value struct.") },
            }
        }
    }

    fn as_number(&self) -> f32 {
        unsafe {
            match self {
                Value { kind: ValueKind::ValNumber, u: ValueUnion { f } } => *f,
                _ => { panic!("Error getting number primitive from Value struct.") },
            }
        }
    }

}