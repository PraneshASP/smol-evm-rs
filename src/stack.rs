#[derive(Debug)]
pub struct Stack {
    pub stack: Vec<usize>,
    pub max_depth: usize,
}

#[derive(Debug)]
pub enum StackError {
    InvalidItem(usize),
    StackOverflow,
    StackUnderflow,
}

impl Stack {
    pub fn new(max_depth: usize) -> Self {
        Self {
            stack: Vec::new(),
            max_depth,
        }
    }

    pub fn push(&mut self, item: usize) -> Result<(), StackError> {
        if item > usize::MAX {
            return Err(StackError::InvalidItem(item));
        }

        if self.stack.len() >= self.max_depth {
            return Err(StackError::StackOverflow);
        }

        self.stack.push(item);
        Ok(())
    }

    pub fn pop(&mut self) -> Result<usize, StackError> {
        match self.stack.pop() {
            Some(item) => Ok(item),
            None => Err(StackError::StackUnderflow),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn new_stack_is_empty() {
        let stack = Stack::new(1024);
        assert_eq!(stack.stack.len(), 0);
    }

    #[test]
    fn push_and_pop_on_stack() {
        let mut stack = Stack::new(1024);
        stack.push(42).unwrap();
        assert_eq!(stack.stack.len(), 1);
        assert_eq!(stack.pop().unwrap(), 42);
        assert_eq!(stack.stack.len(), 0);
    }
}
