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
        write!(f, "{:?}", self)
    }
}

impl Error for LexerError {}

#[derive(Debug)]
pub enum InstructionError {
    InvalidInstruction,
}

impl Display for InstructionError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
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
        write!(f, "{:?}", self)
    }
}

impl Error for StackError {}
