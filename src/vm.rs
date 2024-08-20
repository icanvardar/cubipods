use std::error::Error;

use crate::{memory::Memory, stack::Stack, storage::Storage, Lexer};

#[derive(Default)]
pub struct Vm<'a> {
    pub stack: Stack<String>,
    pub lexer: Lexer<'a>,
    pub memory: Memory,
    pub storage: Storage,
}

impl<'a> Vm<'a> {
    pub fn new(bytecode: &'a str) -> Result<Self, Box<dyn Error>> {
        Ok(Self {
            lexer: Lexer::new(bytecode)?,
            ..Default::default()
        })
    }

    pub fn run(&mut self) -> Result<(), Box<dyn Error>> {
        self.lexer.read_char();

        while self.lexer.ch != '\0' {
            let tmp = self.lexer.next_byte()?;

            println!("{:?}", tmp);
        }

        Ok(())
    }
}
