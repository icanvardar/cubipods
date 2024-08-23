use std::fmt::Debug;

use super::utils::errors::StackError;

const STACK_SIZE_LIMIT: usize = 1024;

#[derive(Debug)]
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

    pub fn pop(&mut self) -> Result<(usize, T), StackError> {
        let popped_index = self.stack.len().saturating_sub(1);
        match self.stack.is_empty() {
            true => Err(StackError::StackUnderflow),
            false => Ok((popped_index, self.stack.pop().unwrap())),
        }
    }

    pub fn push(&mut self, item: T) -> Result<usize, StackError> {
        let pushed_index = self.stack.len();
        match self.validate_stack_size() {
            true => Err(StackError::StackOverflow),
            false => {
                self.stack.push(item);
                Ok(pushed_index)
            }
        }
    }

    pub fn dup(&mut self, index: usize) -> Result<(usize, T), StackError> {
        match self.stack.len() <= index {
            true => Err(StackError::StackSizeExceeded),
            false => {
                let duplicated_index = self.stack.len() - 1 - index;

                let value = self.stack[duplicated_index].clone();
                self.stack.push(value.clone());

                Ok((duplicated_index, value))
            }
        }
    }

    pub fn swap(&mut self, index: usize) -> Result<([usize; 2], [T; 2]), StackError> {
        let stack_length = self.stack.len();

        match stack_length <= index {
            true => Err(StackError::StackSizeExceeded),
            false => {
                let first_item_index = stack_length - 1;
                let second_item_index = first_item_index - index;

                let value_1 = self.stack[first_item_index].clone();
                let value_2 = self.stack[second_item_index].clone();

                self.stack[first_item_index] = value_2.clone();
                self.stack[second_item_index] = value_1.clone();

                Ok(([first_item_index, second_item_index], [value_1, value_2]))
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
    fn it_duplicates_an_item() -> Result<(), StackError> {
        let mut stack: Stack<String> = Stack::new();

        let input_1 = "ff1".to_string();
        let input_2 = "ff2".to_string();
        stack.push(input_1.clone())?;
        stack.push(input_2.clone())?;

        // DUP1 opcode
        let (duplicated_index, duplicated_value) = stack.dup(0)?;

        assert_eq!(duplicated_index, 1);
        assert_eq!(duplicated_value, input_2);

        Ok(())
    }

    #[test]
    fn it_swaps_an_item() -> Result<(), StackError> {
        let mut stack: Stack<String> = Stack::new();

        let input_1 = "ff1".to_string();
        let input_2 = "ff2".to_string();
        let input_3 = "ff3".to_string();
        stack.push(input_1.clone())?;
        stack.push(input_2.clone())?;
        stack.push(input_3.clone())?;

        // SWAP3 opcode
        let ([index_1, index_2], [swapped_1, swapped_2]) = stack.swap(2)?;

        assert_eq!(index_1, 2);
        assert_eq!(index_2, 0);
        assert_eq!(swapped_1, input_3);
        assert_eq!(swapped_2, input_1);
        assert_eq!(stack.peek(), Some(input_1).as_ref());

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
    fn test_push_function_with_more_than_stack_size_limit_returns_stack_overflow_error(
    ) -> Result<(), StackError> {
        let mut stack: Stack<String> = Stack::new();

        let mut counter = 0;
        while counter < 1025 {
            stack.push("ff".to_string())?;

            counter += 1;
        }

        let result = stack.push("ff".to_string());
        assert!(matches!(result, Err(StackError::StackOverflow)));

        Ok(())
    }

    #[test]
    fn test_pop_function_with_empty_stack_returns_stack_underflow_error() -> Result<(), StackError>
    {
        let mut stack: Stack<String> = Stack::new();

        let result = stack.pop();
        assert!(matches!(result, Err(StackError::StackUnderflow)));

        Ok(())
    }

    #[test]
    fn test_dup_function_with_index_of_more_than_size_returns_stack_size_exceeded_error(
    ) -> Result<(), StackError> {
        let mut stack: Stack<String> = Stack::new();

        let result = stack.swap(32);
        assert!(matches!(result, Err(StackError::StackSizeExceeded)));

        Ok(())
    }

    #[test]
    fn test_swap_function_with_index_of_more_than_size_returns_stack_size_exceeded_error(
    ) -> Result<(), StackError> {
        let mut stack: Stack<String> = Stack::new();

        let result = stack.dup(32);
        assert!(matches!(result, Err(StackError::StackSizeExceeded)));

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
