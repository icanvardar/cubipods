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
                InstructionType::MSTORE => {
                    let [item_1, item_2] = self.pop_first_two_items(InstructionType::MSTORE)?;

                    unsafe {
                        self.memory.mstore(item_1 as usize, to_u8_32(item_2));
                    }
                }
                InstructionType::SLOAD => {
                    let item = u128::from_str_radix(&self.stack.pop()?.unwrap(), 16)?;

                    let result = self.storage.sload(to_u8_32(item)).unwrap();
                    let result: String = from_u8_32(*result);

                    self.stack.push(result)?;
                }
                InstructionType::SSTORE => {
                    let [item_1, item_2] = self.pop_first_two_items(InstructionType::SSTORE)?;

                    self.storage.sstore(to_u8_32(item_1), to_u8_32(item_2));
                }
                InstructionType::PUSH(size) => {
                    if size > 32 {
                        return Err(Box::new(VmError::IncompatibleSize(InstructionType::PUSH(
                            size,
                        ))));
                    }

                    if size == 0 {
                        self.stack.push("0".to_string())?;
                        continue 'main;
                    }

                    let mut counter = 0;
                    let mut data = "".to_string();
                    while counter < size {
                        data += &self.lexer.next_byte()?;

                        counter += 1;
                    }

                    self.stack.push(data)?;
                }
                InstructionType::DUP(size) => {
                    if size == 0 || size > 16 {
                        return Err(Box::new(VmError::IncompatibleSize(InstructionType::DUP(
                            size,
                        ))));
                    }

                    self.stack.dup(size as usize)?;
                }
                InstructionType::SWAP(size) => {
                    if size == 0 || size > 16 {
                        return Err(Box::new(VmError::IncompatibleSize(InstructionType::SWAP(
                            size,
                        ))));
                    }

                    self.stack.swap(size as usize)?;
                }
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_creates_vm() -> Result<(), Box<dyn Error>> {
        let bytecode = "0x8060";

        let vm = Vm::new(bytecode)?;

        assert_eq!(vm.stack.is_empty(), true);
        assert_eq!(vm.lexer.bytecode, bytecode.strip_prefix("0x").unwrap());
        assert_eq!(vm.memory.msize(), 0);
        assert_eq!(vm.storage.size(), 0);

        Ok(())
    }

    #[test]
    fn it_runs_add_opcode() -> Result<(), Box<dyn Error>> {
        // NOTE: 10 + 20 = 30 which is 1e in hex
        let bytecode = "6014600a01";

        let mut vm = Vm::new(bytecode)?;
        vm.run()?;

        assert_eq!(vm.stack.peek().unwrap(), "1e");

        // NOTE: (10 + 20) + 32 = 62 which is 3e in hex
        let bytecode = "6020600a60140101";

        let mut vm = Vm::new(bytecode)?;
        vm.run()?;

        assert_eq!(vm.stack.peek().unwrap(), "3e");

        Ok(())
    }

    #[test]
    fn it_runs_mul_opcode() -> Result<(), Box<dyn Error>> {
        // NOTE: 10 * 20 = 200 which is c8 in hex
        let bytecode = "6014600a02";

        let mut vm = Vm::new(bytecode)?;
        vm.run()?;

        assert_eq!(vm.stack.peek().unwrap(), "c8");

        // NOTE: (10 * 20) * 2 = 400 which is 190 in hex
        let bytecode = "60026014600a0202";

        let mut vm = Vm::new(bytecode)?;
        vm.run()?;

        assert_eq!(vm.stack.peek().unwrap(), "190");

        Ok(())
    }

    #[test]
    fn it_runs_sub_opcode() -> Result<(), Box<dyn Error>> {
        // NOTE: 20 - 10 = 10 which is a in hex
        let bytecode = "600a601403";

        let mut vm = Vm::new(bytecode)?;
        vm.run()?;

        assert_eq!(vm.stack.peek().unwrap(), "a");

        Ok(())
    }

    #[test]
    fn it_runs_div_opcode() -> Result<(), Box<dyn Error>> {
        // NOTE: 5 / 2 = rounded as 2 which is 2 in hex
        let bytecode = "6002600504";

        let mut vm = Vm::new(bytecode)?;
        vm.run()?;

        assert_eq!(vm.stack.peek().unwrap(), "2");

        Ok(())
    }

    #[test]
    fn it_runs_mod_opcode() -> Result<(), Box<dyn Error>> {
        // NOTE: 5 % 2 = 1 which is 1 in hex
        let bytecode = "6002600506";

        let mut vm = Vm::new(bytecode)?;
        vm.run()?;

        assert_eq!(vm.stack.peek().unwrap(), "1");

        Ok(())
    }

    #[test]
    fn it_runs_exp_opcode() -> Result<(), Box<dyn Error>> {
        // NOTE: 5**2 = 25 which is 19 in hex
        let bytecode = "600260050a";

        let mut vm = Vm::new(bytecode)?;
        vm.run()?;

        assert_eq!(vm.stack.peek().unwrap(), "19");

        Ok(())
    }

    #[test]
    fn it_runs_lt_opcode() -> Result<(), Box<dyn Error>> {
        // NOTE: 5 < 10 = true which is 1 in hex
        let bytecode = "600a600510";

        let mut vm = Vm::new(bytecode)?;
        vm.run()?;

        assert_eq!(vm.stack.peek().unwrap(), "1");

        Ok(())
    }

    #[test]
    fn it_runs_gt_opcode() -> Result<(), Box<dyn Error>> {
        // NOTE: 20 > 10 = true which is 1 in hex
        let bytecode = "600a601411";

        let mut vm = Vm::new(bytecode)?;
        vm.run()?;

        assert_eq!(vm.stack.peek().unwrap(), "1");

        Ok(())
    }

    #[test]
    fn it_runs_eq_opcode() -> Result<(), Box<dyn Error>> {
        // NOTE: 10 == 10 = true which is 1 in hex
        let bytecode = "600a600a14";

        let mut vm = Vm::new(bytecode)?;
        vm.run()?;

        assert_eq!(vm.stack.peek().unwrap(), "1");

        Ok(())
    }

    #[test]
    fn it_runs_iszero_opcode() -> Result<(), Box<dyn Error>> {
        // NOTE: 10 == 0 = false which is 0 in hex
        let bytecode = "600a15";

        let mut vm = Vm::new(bytecode)?;
        vm.run()?;

        assert_eq!(vm.stack.peek().unwrap(), "0");

        Ok(())
    }

    #[test]
    fn it_runs_and_opcode() -> Result<(), Box<dyn Error>> {
        // NOTE: 1 & 1 = 1 which is 1 in hex
        let bytecode = "6001600116";

        let mut vm = Vm::new(bytecode)?;
        vm.run()?;

        assert_eq!(vm.stack.peek().unwrap(), "1");

        Ok(())
    }

    #[test]
    fn it_runs_or_opcode() -> Result<(), Box<dyn Error>> {
        // NOTE: 1 | 0 = 1 which is 1 in hex
        let bytecode = "6000600117";

        let mut vm = Vm::new(bytecode)?;
        vm.run()?;

        assert_eq!(vm.stack.peek().unwrap(), "1");

        Ok(())
    }

    #[test]
    fn it_runs_xor_opcode() -> Result<(), Box<dyn Error>> {
        // NOTE: 1 ^ 1 = 0 which is 0 in hex
        let bytecode = "6001600118";

        let mut vm = Vm::new(bytecode)?;
        vm.run()?;

        assert_eq!(vm.stack.peek().unwrap(), "0");

        Ok(())
    }

    #[test]
    fn it_runs_not_opcode() -> Result<(), Box<dyn Error>> {
        // NOTE: !0 = [f; 32] which is ff..ff in hex
        let bytecode = "600019";

        let mut vm = Vm::new(bytecode)?;
        vm.run()?;

        assert_eq!(vm.stack.peek().unwrap(), "ffffffffffffffffffffffffffffffff");

        Ok(())
    }

    #[test]
    fn it_runs_byte_opcode() -> Result<(), Box<dyn Error>> {
        Ok(())
    }

    #[test]
    fn it_runs_keccak256_opcode() -> Result<(), Box<dyn Error>> {
        Ok(())
    }

    #[test]
    fn it_runs_pop_opcode() -> Result<(), Box<dyn Error>> {
        Ok(())
    }

    #[test]
    fn it_runs_mload_opcode() -> Result<(), Box<dyn Error>> {
        Ok(())
    }

    #[test]
    fn it_runs_mstore_opcode() -> Result<(), Box<dyn Error>> {
        Ok(())
    }

    #[test]
    fn it_runs_sload_opcode() -> Result<(), Box<dyn Error>> {
        Ok(())
    }

    #[test]
    fn it_runs_sstore_opcode() -> Result<(), Box<dyn Error>> {
        // NOTE: saves word "hello" in the slot of 1
        let data = "hello"
            .as_bytes()
            .iter()
            .map(|x| format!("{:02x}", x))
            .collect::<String>();

        let bytecode = "64".to_string() + &data + "600155";
        let mut vm = Vm::new(&bytecode)?;

        vm.run()?;

        let data = vm.storage.sload(to_u8_32(1)).unwrap();
        let data: String = from_u8_32(data.clone());

        assert_eq!(data.as_str(), "hello");

        Ok(())
    }

    #[test]
    fn it_runs_push_opcode() -> Result<(), Box<dyn Error>> {
        // NOTE: pushes 12 1 in the stack
        let bytecode = "6b010101010101010101010101";

        let mut vm = Vm::new(bytecode)?;
        vm.run()?;

        assert_eq!(vm.stack.peek().unwrap(), "010101010101010101010101");

        Ok(())
    }

    #[test]
    fn it_runs_dup_opcode() -> Result<(), Box<dyn Error>> {
        // NOTE: duplicates 3rd stack item
        let bytecode = "6b010101010101010101010101";

        let mut vm = Vm::new(bytecode)?;
        vm.run()?;

        assert_eq!(vm.stack.peek().unwrap(), "010101010101010101010101");

        Ok(())
    }

    #[test]
    fn it_runs_swap_opcode() -> Result<(), Box<dyn Error>> {
        // NOTE: swaps 3rd item with 1st item in the stack
        // stack before swap [1, 2, 3]
        // stack after swap [3, 2, 1]
        let bytecode = "60016002600391";

        let mut vm = Vm::new(bytecode)?;
        vm.run()?;

        assert_eq!(vm.stack.peek().unwrap(), "01");

        Ok(())
    }
}
