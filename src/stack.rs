use std::fmt::Debug;

use super::utils::errors::StackError;

const STACK_SIZE_LIMIT: u16 = 1024;

#[derive(Debug)]
pub struct Stack<T> {
    pub head: Option<Box<StackNode<T>>>,
    pub length: u16,
}

#[derive(Clone, Debug)]
pub struct StackNode<T> {
    pub item: T,
    pub prev: Option<Box<StackNode<T>>>,
}

impl<T> Default for Stack<T> {
    fn default() -> Self {
        Self {
            head: None,
            length: 0,
        }
    }
}

impl<T: Clone> Stack<T> {
    pub fn new() -> Self {
        Stack {
            ..Default::default()
        }
    }

    pub fn pop(&mut self) -> Result<(usize, T), StackError> {
        if let Some(head) = self.head.take() {
            self.length -= 1;
            self.head = head.prev;

            Ok((1, head.item))
        } else {
            Err(StackError::StackUnderflow)
        }
    }

    pub fn push(&mut self, item: T) -> Result<usize, StackError>
    where
        T: Clone,
    {
        if self.length == STACK_SIZE_LIMIT {
            return Err(StackError::StackOverflow);
        }

        self.length += 1;
        let index = (self.length - 1) as usize;

        let stack_node = StackNode::new(item, self.head.take());

        self.head = Some(Box::new(stack_node));

        Ok(index)
    }

    pub fn dup(&mut self, index: usize) -> Result<(usize, T), StackError> {
        if usize::from(self.length) <= index {
            return Err(StackError::StackSizeExceeded);
        }

        if let Some(head) = self.head.take() {
            let mut curr = head;
            let mut counter = 0;

            while counter < index {
                if let Some(prev) = curr.prev {
                    curr = prev;
                }
                counter += 1;
            }

            let dup_index = (self.length - 1) as usize - index;

            self.push(curr.item.clone())?;

            Ok((dup_index, curr.item))
        } else {
            Err(StackError::StackIsEmpty)
        }
    }

    /// Documentation
    ///
    /// # Safety
    ///
    /// As Stack::swap, it swaps a specific item with the head of stack.
    pub unsafe fn swap(&mut self, index: usize) -> Result<([usize; 2], [T; 2]), StackError> {
        if index == 0 {
            return Err(StackError::WrongIndex);
        }

        if usize::from(self.length) <= index {
            return Err(StackError::StackSizeExceeded);
        }

        if self.is_empty() {
            return Err(StackError::StackIsEmpty);
        }

        let mut curr = self.head.as_mut().unwrap();
        let mut counter = 0;

        unsafe {
            let head_pointer = &mut curr.item as *mut T;

            while counter < index {
                if let Some(ref mut prev) = curr.prev {
                    curr = prev;
                }
                counter += 1;
            }

            let curr_pointer = &mut curr.item as *mut T;

            let head_item = std::ptr::read(head_pointer);
            let curr_item = std::ptr::read(curr_pointer);

            std::ptr::write(head_pointer, curr_item.clone());
            std::ptr::write(curr_pointer, head_item.clone());

            let head_index = (self.length - 1) as usize;
            let swapped_index = head_index - index;

            Ok(([head_index, swapped_index], [head_item, curr_item]))
        }
    }

    pub fn is_empty(&self) -> bool {
        self.head.is_none()
    }

    pub fn peek(&self) -> Option<&T> {
        if let Some(head) = &self.head {
            Some(&head.item)
        } else {
            None
        }
    }
}

impl<T> StackNode<T> {
    fn new(item: T, prev: Option<Box<StackNode<T>>>) -> Self {
        Self { item, prev }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_creates_stack() {
        let stack: Stack<String> = Stack::new();

        assert_eq!(stack.length, 0);
    }

    #[test]
    fn it_pops_an_item() -> Result<(), StackError> {
        let mut stack: Stack<String> = Stack::new();

        let input = "ff".to_string();
        check_input_validity(&input);

        stack.push(input)?;

        stack.pop()?;

        assert_eq!(stack.length, 0);

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

        unsafe {
            // SWAP3 opcode
            let ([index_1, index_2], [swapped_1, swapped_2]) = stack.swap(2)?;

            assert_eq!(index_1, 2);
            assert_eq!(index_2, 0);
            assert_eq!(swapped_1, input_3);
            assert_eq!(swapped_2, input_1);
            assert_eq!(stack.peek(), Some(input_1).as_ref());
        }

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

        assert_eq!(stack.length, 100);

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
        while counter < 1024 {
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

        let result = stack.dup(32);
        assert!(matches!(result, Err(StackError::StackSizeExceeded)));

        Ok(())
    }

    #[test]
    fn test_swap_function_with_index_of_more_than_size_returns_stack_size_exceeded_error(
    ) -> Result<(), StackError> {
        let mut stack: Stack<String> = Stack::new();

        unsafe {
            let result = stack.swap(32);
            assert!(matches!(result, Err(StackError::StackSizeExceeded)));
        }

        Ok(())
    }

    #[test]
    fn test_swap_function_with_index_zero_returns_wrong_index_error() -> Result<(), StackError> {
        let mut stack: Stack<String> = Stack::new();

        unsafe {
            let result = stack.swap(0);
            assert!(matches!(result, Err(StackError::WrongIndex)));
        }

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
