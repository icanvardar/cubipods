use std::{error::Error, fmt::Display};

const STACK_SIZE_LIMIT: usize = 1024;

pub struct Stack<T> {
    stack: Vec<T>,
}

#[derive(Debug)]
pub enum StackError {
    LimitExceeded,
}

impl Display for StackError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl Error for StackError {}

impl<T> Default for Stack<T> {
    fn default() -> Self {
        Self { stack: Vec::new() }
    }
}

impl<T> Stack<T> {
    pub fn new() -> Self {
        Stack {
            ..Default::default()
        }
    }

    pub fn pop(&mut self) -> Option<T> {
        self.stack.pop()
    }

    pub fn push(&mut self, item: T) -> Result<(), StackError> {
        match self.validate_stack_size() {
            true => Err(StackError::LimitExceeded),
            false => {
                self.stack.push(item);
                Ok(())
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
