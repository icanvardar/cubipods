use std::{error::Error, fmt::Display};

#[derive(Default, Debug)]
pub struct Lexer<'a> {
    pub bytecode: &'a str,
    pub position: u64,
    pub read_position: u64,
    pub ch: char,
}

#[derive(Debug)]
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

impl<'a> Lexer<'a> {
    pub fn new(bytecode: &'a str) -> Result<Lexer<'a>, LexerError> {
        let bytecode = bytecode.trim();

        if bytecode.starts_with("0x") {
            match bytecode.strip_prefix("0x") {
                Some(result) => Ok(Self {
                    bytecode: result,
                    ..Default::default()
                }),
                None => Err(LexerError::UnableToCreateLexer),
            }
        } else {
            Ok(Self {
                bytecode,
                ..Default::default()
            })
        }
    }

    pub fn read_char(&mut self) {
        if self.read_position >= self.bytecode.len() as u64 {
            self.ch = '\0';
        } else {
            let idx = self.read_position as usize;
            self.ch = self.bytecode[idx..idx + 1].chars().nth(0).unwrap();
        }
        self.position = self.read_position;
        self.read_position += 1;
    }

    pub fn next_byte(&mut self) -> Result<String, Box<dyn Error>> {
        let first_nibble = self.ch;
        self.read_char();
        let second_nibble = self.ch;

        if first_nibble.is_whitespace() || second_nibble.is_whitespace() {
            return Err(Box::new(LexerError::HasWhitespace));
        }

        if first_nibble == '\0' || second_nibble == '\0' {
            return Err(Box::new(LexerError::EmptyChar));
        }

        if !first_nibble.is_ascii_hexdigit() || !second_nibble.is_ascii_hexdigit() {
            return Err(Box::new(LexerError::InvalidNibble));
        }

        self.read_char();
        Ok(format!("{first_nibble}{second_nibble}"))
    }
}
