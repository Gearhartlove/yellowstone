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

    pub fn peek(&mut self) -> Option<&T> {
        // let pop = self.pop();
        // if let Some(value) = pop {
        //     let value_reference = &value;
        //     self.push(value);
        //     return Some(value_reference);
        // } return None;
    }

    pub fn reset(&mut self) {
        self.stack.clear();
    }
}

impl<T> Default for Stack<T> {
    fn default() -> Self {
        Stack {
            stack: Vec::<T>::new(),
        }
    }
}