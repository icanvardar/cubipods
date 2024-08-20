use crate::{memory::Memory, stack::Stack, storage::Storage, Lexer};

pub struct Vm<'a> {
    pub stack: Stack<String>,
    pub lexer: Lexer<'a>,
    pub memory: Memory,
    pub storage: Storage,
}

impl<'a> Default for Vm<'a> {
    fn default() -> Self {
        Self {
            stack: Stack::default(),
            lexer: Lexer::default(),
            memory: Memory::default(),
            storage: Storage::default(),
        }
    }
}

impl<'a> Vm<'a> {
    pub fn new() -> Self {
        Self {
            ..Default::default()
        }
    }
}
