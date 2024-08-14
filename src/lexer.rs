use std::{error::Error, fmt::Display};

#[derive(Default, Debug)]
pub struct Lexer<'a> {
    pub bytecode: &'a str,
    pub position: u64,
    pub read_position: u64,
    pub ch: char,
}

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

impl<'a> Lexer<'a> {
    pub fn new(bytecode: &'a str) -> Result<Lexer<'a>, LexerError> {
        let bytecode = if bytecode.starts_with("0x") {
            match bytecode.strip_prefix("0x") {
                Some(result) => result.trim(),
                None => return Err(LexerError::UnableToCreateLexer),
            }
        } else {
            bytecode
        };

        Ok(Self {
            bytecode,
            ..Default::default()
        })
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_trims_bytecode() -> Result<(), Box<dyn Error>> {
        let bytecode = "0x12ffcb";
        let expected = "12ffcb";

        let lexer = Lexer::new(bytecode)?;

        assert_eq!(lexer.bytecode, expected);

        Ok(())
    }

    #[test]
    fn it_reads_char() -> Result<(), Box<dyn Error>> {
        let bytecode = "0x11ffcdaa";
        let expected_chars: Vec<char> = vec!['1', '1', 'f', 'f', 'c', 'd', 'a', 'a'];

        let mut lexer = Lexer::new(bytecode)?;
        lexer.read_char();

        for (index, _) in lexer.bytecode.chars().enumerate() {
            if lexer.ch == '\0' {
                break;
            }

            assert_eq!(lexer.ch, expected_chars[index]);
            lexer.read_char();
        }

        Ok(())
    }

    #[test]
    fn it_gets_next_byte() -> Result<(), Box<dyn Error>> {
        let bytecode = "0x608011facddb";
        let expected_bytes: Vec<&str> = vec!["60", "80", "11", "fa", "cd", "db"];

        let mut lexer = Lexer::new(bytecode)?;
        lexer.read_char();

        for (index, _) in lexer.bytecode.chars().enumerate() {
            if lexer.ch == '\0' {
                break;
            }

            assert_eq!(lexer.next_byte()?, expected_bytes[index]);
        }

        Ok(())
    }

    #[test]
    fn test_bytecode_contains_whitespace_returns_lexer_error() -> Result<(), Box<dyn Error>> {
        let bytecode = "0x60 80";

        let mut lexer = Lexer::new(bytecode)?;
        lexer.read_char();
        lexer.next_byte()?;
        let next_byte = lexer.next_byte();

        assert!(matches!(
            next_byte,
            Err(e) if e.downcast_ref::<LexerError>() == Some(&LexerError::HasWhitespace)
        ));

        Ok(())
    }

    #[test]
    fn test_bytecode_contains_empty_char_returns_lexer_error() -> Result<(), Box<dyn Error>> {
        let bytecode = "0x1";

        let mut lexer = Lexer::new(bytecode)?;
        let next_byte = lexer.next_byte();

        assert!(matches!(
            next_byte,
            Err(e) if e.downcast_ref::<LexerError>() == Some(&LexerError::EmptyChar)
        ));

        Ok(())
    }

    #[test]
    fn test_bytecode_contains_non_hex_char_returns_lexer_error() -> Result<(), Box<dyn Error>> {
        let bytecode = "0xzz";

        let mut lexer = Lexer::new(bytecode)?;
        lexer.read_char();
        let next_byte = lexer.next_byte();

        assert!(matches!(
            next_byte,
            Err(e) if e.downcast_ref::<LexerError>() == Some(&LexerError::InvalidNibble)
        ));

        Ok(())
    }
}
