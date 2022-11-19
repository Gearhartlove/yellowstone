use crate::error::InterpretError;
use std::fmt::{Debug, Display, Formatter};
use std::mem::ManuallyDrop;
use std::rc::Rc;

#[repr(C)]
pub struct Value {
    pub kind: ValueKind,
    u: ValueUnion,
    /// if once is true, this Value can only be assigned once.
    pub once: bool,
}

#[repr(C)]
pub union ValueUnion {
    f: f32,
    b: bool,
    o: ManuallyDrop<Rc<dyn ObjectHandler>>,
}

type YSObject = ManuallyDrop<Rc<dyn ObjectHandler>>;

#[repr(u32)]
#[derive(Clone, Copy, Eq, PartialEq)]
pub enum ValueKind {
    ValBool,
    ValNil,
    ValNumber,
    ValObj,
}

impl Display for ValueKind {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let word = match self {
            ValueKind::ValBool => "ValBool",
            ValueKind::ValNil => "ValNil",
            ValueKind::ValNumber => "ValNumber",
            ValueKind::ValObj => "ValObj",
        };

        write!(f, "{}", word)
    }
}

impl Clone for Value {
    fn clone(&self) -> Self {
        match self.kind {
            ValueKind::ValBool => {
                let b = self.as_bool().unwrap();
                Value {
                    kind: ValueKind::ValBool,
                    u: ValueUnion { b },
                    once: false,
                }
            }
            ValueKind::ValNil => Value {
                kind: ValueKind::ValNil,
                u: ValueUnion { f: 0. },
                once: false,
            },
            ValueKind::ValNumber => {
                let f = self.as_number().unwrap();
                Value {
                    kind: ValueKind::ValNumber,
                    u: ValueUnion { f },
                    once: false,
                }
            }
            ValueKind::ValObj => {
                let o = self.as_obj().unwrap();
                Value {
                    kind: ValueKind::ValObj,
                    u: ValueUnion {
                        o: ManuallyDrop::new(o),
                    },
                    once: false,
                }
            }
        }
    }
}

impl PartialEq<str> for Value {
    fn eq(&self, other: &str) -> bool {
        let conv = self.as_string().unwrap();
        // todo: reason out the extra " " around the word

        &conv.as_str()[1..conv.len() - 1] == other
    }
}

impl PartialEq<bool> for Value {
    fn eq(&self, other: &bool) -> bool {
        let conv = self.as_bool().unwrap();
        conv == *other
    }
}

impl PartialEq<f32> for Value {
    fn eq(&self, other: &f32) -> bool {
        let conv = match self.kind {
            ValueKind::ValNil => self.as_nil().unwrap(),
            ValueKind::ValNumber => self.as_number().unwrap(),
            _ => (return false),
        };
        conv == *other
    }
}

impl PartialEq<Value> for str {
    fn eq(&self, other: &Value) -> bool {
        let other = other.as_string().unwrap();
        // todo: reason out the extra " " around the word

        self == &other.as_str()[1..other.len() - 1]
    }
}

impl PartialEq<Value> for f32 {
    fn eq(&self, other: &Value) -> bool {
        let conv = other.as_number().unwrap();
        *self == conv
    }
}

impl PartialEq<Value> for bool {
    fn eq(&self, other: &Value) -> bool {
        let conv = other.as_bool().unwrap();
        *self == conv
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
            //let a = self.as_obj().unwrap();
            //let b = self.as_obj().unwrap();
            //a == b
            todo!("no mechanism to compare raw objects yet")
        } else {
            false
        };
    }
}

impl Debug for Value {
    fn fmt(&self, fmtr: &mut Formatter<'_>) -> std::fmt::Result {
        unsafe {
            match self {
                Value {
                    kind: ValueKind::ValBool,
                    u: ValueUnion { b },
                    once,
                } => write!(fmtr, "{}", b),
                Value {
                    kind: ValueKind::ValNil,
                    u: ValueUnion { f },
                    once,
                } => write!(fmtr, "{}", f),
                Value {
                    kind: ValueKind::ValNumber,
                    u: ValueUnion { f },
                    once,
                } => write!(fmtr, "{}", f),
                Value {
                    kind: ValueKind::ValObj,
                    u: ValueUnion { o },
                    once,
                } => write!(fmtr, "{}", o.to_string()),
            }
        }
    }
}

impl Value {
    pub fn values_equal(a: Value, b: Value) -> bool {
        if a.kind != b.kind {
            return false;
        }

        match a.kind {
            ValueKind::ValBool => Value::as_bool(&a) == Value::as_bool(&b),
            ValueKind::ValNil => true,
            ValueKind::ValNumber => Value::as_number(&a) == Value::as_number(&b),
            ValueKind::ValObj => {
                let string_a = a.as_string().unwrap();
                let string_b = b.as_string().unwrap();

                string_a == string_b
            }
        }
    }

    pub fn print(&self) {
        match self.kind {
            ValueKind::ValBool => {
                println!("{}", self.as_bool().unwrap())
            }
            ValueKind::ValNil => {
                println!("{}", self.as_nil().unwrap())
            }
            ValueKind::ValNumber => {
                println!("{}", self.as_number().unwrap())
            }
            ValueKind::ValObj => {
                println!("{:?}", self.as_obj())
            }
        }
    }

    // instantiate a Value from a Rust primitive
    // primitive -> Value
    pub fn bool_val(b: bool) -> Self {
        Self {
            kind: ValueKind::ValBool,
            u: ValueUnion { b },
            once: false,
        }
    }

    pub fn nil_value() -> Self {
        Self {
            kind: ValueKind::ValNil,
            u: ValueUnion { f: 0. },
            once: false,
        }
    }

    pub fn number_value(num: f32) -> Self {
        Self {
            kind: ValueKind::ValNumber,
            u: ValueUnion { f: num },
            once: false,
        }
    }

    fn obj_value(o: Rc<dyn ObjectHandler>) -> Self {
        Self {
            kind: ValueKind::ValObj,
            u: ValueUnion {
                o: ManuallyDrop::new(Rc::clone(&o)),
            },
            once: false,
        }
    }

    // read the rust value from the Value struct
    // Value -> primitive
    pub fn as_bool(&self) -> Result<bool, InterpretError> {
        if self.is_bool() {
            unsafe { Ok(self.u.b) }
        } else {
            Err(InterpretError::RUNTIME_ERROR)
        }
    }

    pub fn as_nil(&self) -> Result<f32, InterpretError> {
        if self.is_nil() {
            unsafe { Ok(self.u.f) }
        } else {
            Err(InterpretError::RUNTIME_ERROR)
        }
    }

    pub fn as_number(&self) -> Result<f32, InterpretError> {
        if self.is_number() {
            unsafe { Ok(self.u.f) }
        } else {
            Err(InterpretError::RUNTIME_ERROR)
        }
    }

    pub fn as_obj(&self) -> Result<Rc<dyn ObjectHandler>, InterpretError> {
        if self.is_obj() {
            unsafe { Ok(Rc::clone(&self.u.o)) }
        } else {
            Err(InterpretError::RUNTIME_ERROR)
        }
    }

    pub fn as_string(&self) -> Result<String, InterpretError> {
        if self.is_obj() {
            let obj = self.as_obj().unwrap();
            Ok(obj.to_string().as_str().trim_matches('\"').to_string())
        } else {
            eprintln!("Error: cannot convert type to a string. It is not an object.");
            Err(InterpretError::RUNTIME_ERROR)
        }
    }

    // check the kind of a value and return true or false
    pub fn is_bool(&self) -> bool {
        unsafe {
            matches!(
                self,
                Value {
                    kind: ValueKind::ValBool,
                    u: ValueUnion { b: _b },
                    once
                }
            )
        }
    }

    pub fn is_nil(&self) -> bool {
        matches!(
            self,
            Value {
                kind: ValueKind::ValNil,
                u: ValueUnion { f: _ },
                once
            }
        )
    }

    pub fn is_number(&self) -> bool {
        unsafe {
            matches!(
                self,
                Value {
                    kind: ValueKind::ValNumber,
                    u: ValueUnion { f: _f },
                    once
                }
            )
        }
    }

    pub fn is_obj(&self) -> bool {
        unsafe {
            matches!(
                self,
                Value {
                    kind: ValueKind::ValObj,
                    u: ValueUnion { o: _o },
                    once
                }
            )
        }
    }

    pub fn is_string(value: &Value) -> bool {
        Value::is_obj_kind(value, ObjKind::OBJ_STRING)
    }

    fn is_obj_kind(value: &Value, obj_kind: ObjKind) -> bool {
        value.is_obj() && value.as_obj().unwrap().kind() == obj_kind
    }
}

// ##############################################################
// Object Type
// ##############################################################

// > uses trait inheritance : a constraint on implementors of MyTrait: "If you implement MyTrait, you have to implement Debug too"
// todo: research if a static lifetime is right here
pub fn allocate_object<T>(data: T) -> Value
where
    T: ObjectHandler + 'static,
{
    let rc = Rc::new(data);
    let obj = Value::obj_value(rc);

    return obj;
}

pub trait ObjectHandler: std::fmt::Debug {
    fn kind(self: Rc<Self>) -> ObjKind;

    fn to_string(&self) -> String {
        format!("{:?}", self)
    }
}

// ##############################################################
// Object Implementations
// ##############################################################

#[allow(non_camel_case_types)]
#[derive(PartialOrd, PartialEq, Debug)]
pub enum ObjKind {
    OBJ_STRING,
}

impl ObjectHandler for String {
    fn kind(self: Rc<Self>) -> ObjKind {
        ObjKind::OBJ_STRING
    }
}

impl ObjectHandler for &str {
    fn kind(self: Rc<Self>) -> ObjKind {
        ObjKind::OBJ_STRING
    }
}
