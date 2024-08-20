use std::{error::Error, str::FromStr};

use keccak_hash::keccak256;

use crate::{
    instruction::InstructionType,
    memory::Memory,
    stack::Stack,
    storage::Storage,
    utils::{
        bytes::{from_u8_32, to_u8_32},
        errors::VmError,
    },
    Lexer,
};

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

        'main: while self.lexer.ch != '\0' {
            let instruction = self.lexer.next_byte()?;
            let instruction = InstructionType::from_str(&instruction)?;

            match instruction {
                InstructionType::STOP => break 'main,
                InstructionType::ADD => {
                    let [item_1, item_2] = self.pop_first_two_items(InstructionType::ADD)?;

                    let result = item_1 + item_2;

                    self.stack.push(format!("{:x}", result))?;
                }
                InstructionType::MUL => {
                    let [item_1, item_2] = self.pop_first_two_items(InstructionType::MUL)?;

                    let result = item_1 * item_2;

                    self.stack.push(format!("{:x}", result))?;
                }
                InstructionType::SUB => {
                    let [item_1, item_2] = self.pop_first_two_items(InstructionType::SUB)?;

                    let result = item_1 - item_2;

                    self.stack.push(format!("{:x}", result))?;
                }
                InstructionType::DIV => {
                    let [item_1, item_2] = self.pop_first_two_items(InstructionType::DIV)?;

                    let result = item_1 / item_2;

                    self.stack.push(format!("{:x}", result))?;
                }
                InstructionType::MOD => {
                    let [item_1, item_2] = self.pop_first_two_items(InstructionType::MOD)?;

                    let result = item_1 % item_2;

                    self.stack.push(format!("{:x}", result))?;
                }
                InstructionType::EXP => {
                    let [item_1, item_2] = self.pop_first_two_items(InstructionType::EXP)?;

                    let result = u128::pow(item_1, item_2 as u32);

                    self.stack.push(format!("{:x}", result))?;
                }
                InstructionType::LT => {
                    let [item_1, item_2] = self.pop_first_two_items(InstructionType::LT)?;

                    let result = item_1 < item_2;

                    self.stack.push(format!("{:x}", result as u128))?;
                }
                InstructionType::GT => {
                    let [item_1, item_2] = self.pop_first_two_items(InstructionType::GT)?;

                    let result = item_1 > item_2;

                    self.stack.push(format!("{:x}", result as u128))?;
                }
                InstructionType::EQ => {
                    let [item_1, item_2] = self.pop_first_two_items(InstructionType::EQ)?;

                    let result = item_1 == item_2;

                    self.stack.push(format!("{:x}", result as u128))?;
                }
                InstructionType::ISZERO => {
                    let item = u128::from_str_radix(&self.stack.pop()?.unwrap(), 16)?;

                    let result = item == 0;

                    self.stack.push(format!("{:x}", result as u128))?;
                }
                InstructionType::AND => {
                    let [item_1, item_2] = self.pop_first_two_items(InstructionType::AND)?;

                    let result = item_1 & item_2;

                    self.stack.push(format!("{:x}", result))?;
                }
                InstructionType::OR => {
                    let [item_1, item_2] = self.pop_first_two_items(InstructionType::OR)?;

                    let result = item_1 | item_2;

                    self.stack.push(format!("{:x}", result))?;
                }
                InstructionType::XOR => {
                    let [item_1, item_2] = self.pop_first_two_items(InstructionType::XOR)?;

                    let result = item_1 ^ item_2;

                    self.stack.push(format!("{:x}", result))?;
                }
                InstructionType::NOT => {
                    let item = u128::from_str_radix(&self.stack.pop()?.unwrap(), 16)?;

                    let result = !item;

                    self.stack.push(format!("{:x}", result))?;
                }
                InstructionType::BYTE => {
                    let [item_1, item_2] = self.pop_first_two_items(InstructionType::BYTE)?;

                    let result = if item_1 < 32 {
                        (item_2 >> (8 * (31 - item_1))) & 0xFF
                    } else {
                        0
                    };

                    self.stack.push(format!("{:x}", result))?;
                }
                InstructionType::KECCAK256 => {
                    let item = u128::from_str_radix(&self.stack.pop()?.unwrap(), 16)?;

                    let mut result = to_u8_32(item);
                    keccak256(&mut result);
                    let result: String = from_u8_32(result);

                    self.stack.push(result)?;
                }
                InstructionType::POP => {
                    self.stack.pop()?;
                }
                InstructionType::MLOAD => {
                    let item = u128::from_str_radix(&self.stack.pop()?.unwrap(), 16)?;

                    let result;
                    unsafe {
                        result = self.memory.mload(item as usize);
                    }
                    let result: String = from_u8_32(result);

                    self.stack.push(result)?;
                }
                InstructionType::MSTORE => {}
                InstructionType::SLOAD => {}
                InstructionType::SSTORE => {}
                InstructionType::PUSH(size) => {}
                InstructionType::DUP(size) => {}
                InstructionType::SWAP(size) => {}
            }
        }

        Ok(())
    }

    fn pop_first_two_items(
        &mut self,
        instruction: InstructionType,
    ) -> Result<[u128; 2], Box<dyn Error>> {
        if self.stack.length() < 2 {
            return Err(Box::new(VmError::ArithmeticOperationError(instruction)));
        }

        // NOTE: custom errors might be added for data extraction
        let value_1 = u128::from_str_radix(&self.stack.pop()?.unwrap(), 16)?;
        let value_2 = u128::from_str_radix(&self.stack.pop()?.unwrap(), 16)?;

        Ok([value_1, value_2])
    }
}
