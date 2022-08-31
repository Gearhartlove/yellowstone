use std::fmt::{Debug, Formatter};

#[derive(Default)]
pub struct Stack<T> {
    pub stack: Vec<T>,
}

impl<T> Stack<T> {
    pub fn push(&mut self, pushing: T) {
        self.stack.push(pushing);
    }

    pub fn pop(&mut self) -> Option<T> {
        return self.stack.pop();
    }

    pub fn is_empty(&self) -> bool {
        return self.stack.is_empty();
    }

    pub fn peek(&self) -> Option<&T> {
        return self.peek();
    }

    pub fn reset(&mut self) {
        self.stack.clear();
    }
}