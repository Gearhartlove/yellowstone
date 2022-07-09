// trying to make any variable be able to be a constant.

use std::fmt::Formatter;
use crate::value;

pub struct Values {
    pub values: Vec<Box<dyn Evaluate>>
}

impl Values {
    // not sure why I need this static lifetime here?
    pub(crate) fn push<T: 'static + Evaluate>(&mut self, val: T) {
        let boxed_val = Box::from(val);
        self.values.push(boxed_val)
    }
}

impl Default for Values {
    fn default() -> Self {
        Values { values: vec![] }
    }
}

pub trait Evaluate {
    fn eval(&self) -> Box<dyn Evaluate>;
}

impl Evaluate for u8 {
    fn eval(&self) -> Box<dyn Evaluate> {
        Box::from(*self)
    }
}

impl Evaluate for f32 {
    fn eval(&self) -> Box<dyn Evaluate> {
        Box::from(*self)
    }
}