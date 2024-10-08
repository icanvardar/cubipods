use std::{error::Error, fmt::Display};

use crate::instruction::InstructionType;

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
    StackIsEmpty,
    WrongIndex,
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
            StackError::StackIsEmpty => {
                write!(f, "Stack is empty.")
            }
            StackError::WrongIndex => {
                write!(f, "Given index is wrong.")
            }
        }
    }
}

impl Error for StackError {}

#[derive(Debug)]
pub enum VmError<'a> {
    ShallowStack(&'a InstructionType),
    IncompatibleSize(InstructionType),
}

impl<'a> Display for VmError<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            VmError::ShallowStack(instruction_type) => {
                write!(f, "Cannot call {:?} opcode.", instruction_type)
            }
            VmError::IncompatibleSize(instruction_type) => {
                write!(f, "Size exceeds {:?} opcode limit.", instruction_type)
            }
        }
    }
}

impl<'a> Error for VmError<'a> {}

#[derive(Debug)]
pub enum HistoryError {
    EmptyDescription,
}

impl Display for HistoryError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            HistoryError::EmptyDescription => {
                write!(f, "Provided description is empty.")
            }
        }
    }
}

impl Error for HistoryError {}

#[derive(Debug)]
pub enum Bytes32Error {
    InvalidStr,
    U128ConversionFailed,
}

impl Error for Bytes32Error {}

impl Display for Bytes32Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Bytes32Error::InvalidStr => {
                write!(f, "Invalid string is provided.")
            }
            Bytes32Error::U128ConversionFailed => {
                write!(f, "The conversion from Bytes32 to u128 is failed.")
            }
        }
    }
}
