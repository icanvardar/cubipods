use super::utils::errors::StackError;

const STACK_SIZE_LIMIT: usize = 1024;

pub struct Stack<T> {
    stack: Vec<T>,
}

impl<T> Default for Stack<T> {
    fn default() -> Self {
        Self { stack: Vec::new() }
    }
}

impl<T: Clone> Stack<T> {
    pub fn new() -> Self {
        Stack {
            ..Default::default()
        }
    }

    pub fn pop(&mut self) -> Result<Option<T>, StackError> {
        match self.stack.is_empty() {
            true => Err(StackError::StackUnderflow),
            false => Ok(self.stack.pop()),
        }
    }

    pub fn push(&mut self, item: T) -> Result<(), StackError> {
        match self.validate_stack_size() {
            true => Err(StackError::StackOverflow),
            false => {
                self.stack.push(item);
                Ok(())
            }
        }
    }

    pub fn dup(&mut self, index: usize) -> Result<T, StackError> {
        match self.stack.len() <= index {
            true => Err(StackError::StackSizeExceeded),
            false => {
                let value = self.stack[self.stack.len() - 1 - index].clone();
                self.stack.push(value.clone());

                Ok(value)
            }
        }
    }

    pub fn swap(&mut self, index: usize) -> Result<[T; 2], StackError> {
        let stack_length = self.stack.len();

        match stack_length <= index {
            true => Err(StackError::StackSizeExceeded),
            false => {
                let value_1 = self.stack[stack_length - 1].clone();
                let value_2 = self.stack[stack_length - 1 - index].clone();

                self.stack[stack_length - 1] = value_2.clone();
                self.stack[stack_length - 1 - index] = value_1.clone();

                Ok([value_1, value_2])
            }
        }
    }

    pub fn is_empty(&self) -> bool {
        self.stack.is_empty()
    }

    pub fn length(&self) -> usize {
        self.stack.len()
    }

    pub fn peek(&self) -> Option<&T> {
        self.stack.last()
    }

    fn validate_stack_size(&self) -> bool {
        self.length().gt(&STACK_SIZE_LIMIT)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_creates_stack() {
        let stack: Stack<String> = Stack::new();

        assert_eq!(stack.stack.len(), 0);
    }

    #[test]
    fn it_pops_an_item() -> Result<(), StackError> {
        let mut stack: Stack<String> = Stack::new();

        let input = "ff".to_string();
        check_input_validity(&input);

        stack.push(input)?;

        stack.pop()?;

        assert_eq!(stack.stack.len(), 0);

        Ok(())
    }

    #[test]
    fn it_pushes_an_item() -> Result<(), StackError> {
        let mut stack: Stack<String> = Stack::new();

        let input = "ff".to_string();
        check_input_validity(&input);

        stack.push(input.clone())?;

        assert_eq!(stack.peek(), Some(&input));

        Ok(())
    }

    #[test]
    fn test_is_empty_function() -> Result<(), StackError> {
        let mut stack: Stack<String> = Stack::new();

        assert!(stack.is_empty());

        let input = "ff".to_string();
        check_input_validity(&input);

        stack.push(input)?;

        assert_eq!(stack.is_empty(), false);

        Ok(())
    }

    #[test]
    fn test_length_function() -> Result<(), StackError> {
        let mut stack: Stack<String> = Stack::new();

        let mut counter = 0;
        while counter < 100 {
            stack.push("ff".to_string())?;

            counter += 1;
        }

        assert_eq!(stack.length(), 100);

        Ok(())
    }

    #[test]
    fn test_peek_function() -> Result<(), StackError> {
        let mut stack: Stack<String> = Stack::new();

        let input = "ff".to_string();
        check_input_validity(&input);

        stack.push(input.clone())?;

        let result = stack.peek();

        assert_eq!(result, Some(&input));

        Ok(())
    }

    #[test]
    fn test_push_function_with_more_than_stack_size_limit_returns_stack_error(
    ) -> Result<(), StackError> {
        let mut stack: Stack<String> = Stack::new();

        let mut counter = 0;
        while counter < 1025 {
            stack.push("ff".to_string())?;

            counter += 1;
        }

        let result = stack.push("ff".to_string());
        assert!(matches!(result, Err(StackError::LimitExceeded)));

        Ok(())
    }

    // NOTE: Helper function
    fn check_input_validity(input: &String) {
        for c in input.chars() {
            if !c.is_ascii_hexdigit() {
                panic!();
            }
        }
    }
}
