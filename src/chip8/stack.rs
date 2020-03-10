/// Struct representing the stack of a chip-8 machine
pub struct Stack {
    stack: [u16; 16],
    stack_pointer: usize,
}

impl Stack {
    /// Creates a new Stack struct with all values initialized to zero
    pub fn new() -> Stack {
        Stack {
            stack: [0; 16],
            stack_pointer: 0,
        }
    }

    /// Pushes a value onto the top of the stack.
    /// If the stack is full, StackError::Full is returned
    pub fn push(&mut self, val: u16) -> Result<(), StackError> {
        if self.stack_pointer >= self.stack.len() {
            return Err(StackError::Full);
        }
        self.stack[self.stack_pointer] = val;
        self.stack_pointer += 1;
        Ok(())
    }

    /// Pops the value on the top of the stack.
    /// If the stack is empty, StackError::Empty is returned
    pub fn pop(&mut self) -> Result<u16, StackError> {
        if self.stack_pointer <= 0 {
            return Err(StackError::Empty);
        }
        self.stack_pointer -= 1;
        Ok(self.stack[self.stack_pointer])
    }

    /// Returns the value on the top of the stack if is not empty
    /// Otherwise, it returns none
    pub fn top(&self) -> Option<u16> {
        if self.stack_pointer <= 0 {
            return None;
        }
        Some(self.stack[self.stack_pointer - 1])
    }
}

#[derive(Debug, PartialEq)]
pub enum StackError {
    Full,
    Empty,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_push_top_pop() {
        let mut stack = Stack::new();
        assert_eq!(None, stack.top());
        stack.push(3);
        assert_eq!(Some(3), stack.top());
        stack.push(5);
        assert_eq!(Some(5), stack.top());
        assert_eq!(Ok(5), stack.pop());
        assert_eq!(Some(3), stack.top());
    }

    #[test]
    #[should_panic]
    fn test_pop_panic() {
        let mut stack = Stack::new();
        stack.pop().unwrap();
    }

    #[test]
    #[should_panic]
    fn test_push_panic() {
        let mut stack = Stack::new();
        for i in 0..=stack.stack.len() {
            stack.push(3).unwrap();
        }
    }
}
