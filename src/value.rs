use std::borrow::Borrow;
use std::fmt::{Debug, Formatter};
use std::mem::ManuallyDrop;
use std::rc::Rc;
use crate::error::InterpretError;

#[repr(C)]
pub struct Value {
    kind: ValueKind,
    u: ValueUnion,
}

impl Clone for Value {
    fn clone(&self) -> Self {
        match self.kind {
            ValueKind::ValBool => { let b = self.as_bool().unwrap(); Value { kind: ValueKind::ValBool, u: ValueUnion { b } } }
            ValueKind::ValNil => { Value { kind: ValueKind::ValNil, u: ValueUnion { f: 0. } } }
            ValueKind::ValNumber => { let f = self.as_number().unwrap(); Value { kind: ValueKind::ValNumber, u: ValueUnion { f } } }
            ValueKind::ValObj => { let o = self.as_obj().unwrap(); Value { kind: ValueKind::ValObj, u: ValueUnion { o: ManuallyDrop::new(o) } } }
        }
    }
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
        } else if self.is_obj() && other.is_obj() {
            let a = self.as_bool().unwrap();
            let b = self.as_bool().unwrap();
            a == b
        }
        else {
            false
        }
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
                },
                Value { kind: ValueKind::ValNil, u: ValueUnion { f } } => {
                    formatter.debug_struct("Value")
                        .field("nil_val", f)
                        .finish()
                },
                Value { kind: ValueKind::ValNumber, u: ValueUnion { f } } => {
                    formatter.debug_struct("Value")
                        .field("number_value", f)
                        .finish()
                },
                Value { kind: ValueKind::ValObj, u: ValueUnion { o } } => {
                    formatter.debug_struct("Object")
                        .field("object_value", o)
                        .finish()
                }
            }
        }
    }
}

pub type YSObject = Rc<dyn ObjectHandler>;

// > uses trait inheritance : a constraint on implementors of MyTrait: "If you implement MyTrait, you have to implement Debug too"
trait ObjectHandler: std::fmt::Debug {
    fn value(self: Rc<Self>) -> YSObject;
}

#[repr(C)]
pub union ValueUnion {
    f: f32,
    b: bool,
    o: ManuallyDrop<YSObject>,
}

#[repr(u32)]
#[derive(Clone, Copy, Eq, PartialEq)]
pub enum ValueKind {
    ValBool,
    ValNil,
    ValNumber,
    ValObj,
}

impl Value {
    pub fn values_equal(a: Value, b: Value) -> bool {
        if a.kind != b.kind {
            return false
        }
        return match a.kind {
            ValueKind::ValBool => { Value::as_bool(&a) == Value::as_bool(&b) },
            ValueKind::ValNil => { true },
            ValueKind::ValNumber => { Value::as_number(&a) == Value::as_number(&b) },
            ValueKind::ValObj => { Rc::ptr_eq(&Value::as_obj(&a).unwrap(), &Value::as_obj(&b).unwrap()) }
        }
    }

    fn print_value(self) {
        match self.kind {
            ValueKind::ValBool => { println!("{}", self.as_bool().unwrap()) },
            ValueKind::ValNil => { println!("{}", self.as_nil().unwrap()) },
            ValueKind::ValNumber => { println!("{}", self.as_number().unwrap()) },
            ValueKind::ValObj => { println!("{:?}", self.as_obj()) }
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

    pub fn obj_value(o: YSObject) -> Self {
        Self {
            kind: ValueKind::ValObj,
            u: ValueUnion { o: ManuallyDrop::new(Rc::clone(&o)) }
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

    pub fn as_obj(&self) -> Result<YSObject, InterpretError> {
        if self.is_obj() {
            unsafe {
                Ok(Rc::clone(&self.u.o))
            }
        } else {
            Err(InterpretError::INTERPRET_RUNTIME_ERROR)
        }
    }

    // check the kind of a value and return true or false
    pub fn is_bool(&self) -> bool {
        unsafe {
            match self {
                Value { kind: ValueKind::ValBool, u: ValueUnion { b: _b } } => true,
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
                Value { kind: ValueKind::ValNumber, u: ValueUnion { f: _f } } => true,
                _ => false,
            }
        }
    }

    pub fn is_obj(&self) -> bool {
        unsafe {
            match self {
                Value { kind: ValueKind::ValObj, u: ValueUnion { o: _o } } => true,
                _ => false,
            }
        }
    }
}