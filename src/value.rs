use std::fmt::{Debug, Display, Formatter};
use crate::value;

pub struct Value {
    // todo: add type abstraction
    pub value: f32,
}

impl Value {
    pub fn get(&self) -> f32 {
        self.value
    }
}

impl From<f32> for Value {
    fn from(item: f32) -> Self {
        Value { value: item }
    }
}

impl Display for Value {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.value)
    }
}
