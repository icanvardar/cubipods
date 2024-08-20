use std::{error::Error, fmt::Display};

#[derive(Debug, PartialEq)]
pub enum LexerError {
    UnableToCreateLexer,
    HasWhitespace,
    EmptyChar,
    InvalidNibble,
}

impl Display for LexerError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LexerError::UnableToCreateLexer => {
                write!(f, "An error occured while creating lexer.")
            }
            LexerError::HasWhitespace => {
                write!(f, "A nibble cannot contain whitespace.")
            }
            LexerError::EmptyChar => {
                write!(f, "A nibble cannot be empty character.")
            }
            LexerError::InvalidNibble => {
                write!(f, "A nibble must be hex digit.")
            }
        }
    }
}

impl Error for LexerError {}

#[derive(Debug)]
pub enum InstructionError {
    InvalidInstruction([u8; 2]),
}

impl Display for InstructionError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            InstructionError::InvalidInstruction(opcode) => {
                write!(f, "The opcode {:?} is unknown.", opcode)
            }
        }
    }
}

impl Error for InstructionError {}

#[derive(Debug)]
pub enum StackError {
    StackOverflow,
    StackUnderflow,
    StackSizeExceeded,
}

impl Display for StackError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            StackError::StackOverflow => {
                write!(f, "The stack size exceeded.")
            }
            StackError::StackUnderflow => {
                write!(f, "The stack is currently empty.")
            }
            StackError::StackSizeExceeded => {
                write!(f, "The given index is out of stack size.")
            }
        }
    }
}

impl Error for StackError {}
