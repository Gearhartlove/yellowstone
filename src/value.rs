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
