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
    InvalidIndex,
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

    pub fn peek(&mut self, index: usize) -> Result<usize, StackError> {
        if self.stack.len() <= index {
            return Err(StackError::StackUnderflow);
        }
        Ok(self.stack[self.stack.len() - (index + 1)])
    }

    pub fn swap(&mut self, index: usize) -> Result<(), StackError> {
        if index == 0 {
            return Err(StackError::InvalidIndex);
        }

        let stack_len = self.stack.len();

        if stack_len <= index {
            return Err(StackError::StackUnderflow);
        }

        self.stack.swap(stack_len - 1, index - 1);

        Ok(())
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

    #[test]
    fn test_peek() {
        let mut stack = Stack::new(1024);
        stack.push(41).unwrap();
        stack.push(42).unwrap();
        stack.push(43).unwrap();

        assert_eq!(stack.peek(0).unwrap(), 43);
        assert_eq!(stack.peek(1).unwrap(), 42);
        assert_eq!(stack.peek(2).unwrap(), 41);

        assert!(stack.peek(3).is_err());
    }

    #[test]
    fn test_swap() {
        let mut stack = Stack::new(1024);
        stack.push(41).unwrap();
        stack.push(42).unwrap();
        stack.push(43).unwrap();
        // Before swap
        assert_eq!(stack.peek(0).unwrap(), 43);
        assert_eq!(stack.peek(1).unwrap(), 42);
        assert_eq!(stack.peek(2).unwrap(), 41);

        stack.swap(1).unwrap();

        // After swap 1
        assert_eq!(stack.peek(0).unwrap(), 41);
        assert_eq!(stack.peek(1).unwrap(), 42);
        assert_eq!(stack.peek(2).unwrap(), 43);

        stack.swap(2).unwrap();

        // After swap 2
        assert_eq!(stack.peek(0).unwrap(), 42);
        assert_eq!(stack.peek(1).unwrap(), 41);
        assert_eq!(stack.peek(2).unwrap(), 43);

        assert!(stack.swap(3).is_err());
        assert!(stack.swap(0).is_err());
    }
}
