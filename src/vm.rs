use std::{any::Any, error::Error, fmt::Write, str::FromStr};

use tiny_keccak::{Hasher, Keccak};

use crate::{
    instruction::InstructionType,
    memory::Memory,
    stack::Stack,
    storage::Storage,
    utils::{
        bytes32::{Bytes32, Pow},
        errors::VmError,
        history::{Component, History},
    },
    Lexer,
};

#[derive(Default)]
pub struct Vm<'a> {
    pub stack: Stack<String>,
    pub lexer: Lexer<'a>,
    pub memory: Memory,
    pub storage: Storage,
    pub history: History,
    pub verbose: bool,
}

impl<'a> Vm<'a> {
    pub fn new(bytecode: &'a str, verbose: bool) -> Result<Self, Box<dyn Error>> {
        Ok(Self {
            lexer: Lexer::new(bytecode)?,
            verbose,
            ..Default::default()
        })
    }

    pub fn run(&mut self) -> Result<(), Box<dyn Error>> {
        self.lexer.read_char();

        'main: while self.lexer.ch != '\0' {
            let instruction = self.lexer.next_byte()?;
            let instruction = InstructionType::from_str(&instruction)?;

            let mut build_initials = || -> Result<Box<dyn Any>, Box<dyn Error>> {
                match instruction {
                    InstructionType::ISZERO
                    | InstructionType::NOT
                    | InstructionType::KECCAK256
                    | InstructionType::POP
                    | InstructionType::MLOAD
                    | InstructionType::SLOAD => {
                        let (index_1, item_1) = self.pop_first_item(instruction.clone())?;

                        if self.verbose {
                            self.history
                                .save_on_event(Component::build_stack_with_one_item(
                                    instruction.clone(),
                                    item_1,
                                    index_1 as u16,
                                ))?;
                        }

                        Ok(Box::new(item_1))
                    }
                    InstructionType::PUSH(_size)
                    | InstructionType::DUP(_size)
                    | InstructionType::SWAP(_size) => Ok(Box::new((0, 0))),
                    _ => {
                        let ([index_1, index_2], [item_1, item_2]) =
                            self.pop_first_two_items(instruction.clone())?;

                        if self.verbose {
                            self.history.save_on_event(Component::build_stack(
                                instruction.clone(),
                                item_1,
                                index_1 as u16,
                                item_2,
                                index_2 as u16,
                            ))?;

                            match instruction {
                                InstructionType::MSTORE => self
                                    .history
                                    .save_on_event(Component::build_memory(item_1, item_2))?,
                                InstructionType::SSTORE => self
                                    .history
                                    .save_on_event(Component::build_storage(item_1, item_2))?,
                                _ => {}
                            }
                        }

                        Ok(Box::new((item_1, item_2)))
                    }
                }
            };

            match instruction {
                InstructionType::STOP => break 'main,
                InstructionType::ADD => {
                    let (item_1, item_2) =
                        *build_initials()?.downcast::<(Bytes32, Bytes32)>().unwrap();
                    let result = item_1 + item_2;

                    self.stack.push(result.parse_and_trim()?)?;
                }
                InstructionType::MUL => {
                    let (item_1, item_2) =
                        *build_initials()?.downcast::<(Bytes32, Bytes32)>().unwrap();
                    let result = item_1 * item_2;

                    self.stack.push(result.parse_and_trim()?)?;
                }
                InstructionType::SUB => {
                    let (item_1, item_2) =
                        *build_initials()?.downcast::<(Bytes32, Bytes32)>().unwrap();
                    let result = item_1 - item_2;

                    self.stack.push(result.parse_and_trim()?)?;
                }
                InstructionType::DIV => {
                    let (item_1, item_2) =
                        *build_initials()?.downcast::<(Bytes32, Bytes32)>().unwrap();
                    let result = item_1 / item_2;

                    self.stack.push(result.parse_and_trim()?)?;
                }
                InstructionType::MOD => {
                    let (item_1, item_2) =
                        *build_initials()?.downcast::<(Bytes32, Bytes32)>().unwrap();
                    let result = item_1 % item_2;

                    self.stack.push(result.parse_and_trim()?)?;
                }
                InstructionType::EXP => {
                    let (item_1, item_2) =
                        *build_initials()?.downcast::<(Bytes32, Bytes32)>().unwrap();
                    let result = item_1.pow(item_2);

                    self.stack.push(result.parse_and_trim()?)?;
                }
                InstructionType::LT => {
                    let (item_1, item_2) =
                        *build_initials()?.downcast::<(Bytes32, Bytes32)>().unwrap();
                    let result = item_1 < item_2;

                    self.stack.push(format!("{:x}", result as u128))?;
                }
                InstructionType::GT => {
                    let (item_1, item_2) =
                        *build_initials()?.downcast::<(Bytes32, Bytes32)>().unwrap();
                    let result = item_1 > item_2;

                    self.stack.push(format!("{:x}", result as u128))?;
                }
                InstructionType::EQ => {
                    let (item_1, item_2) =
                        *build_initials()?.downcast::<(Bytes32, Bytes32)>().unwrap();
                    let result = item_1 == item_2;

                    self.stack.push(format!("{:x}", result as u128))?;
                }
                InstructionType::ISZERO => {
                    let item_1 = *build_initials()?.downcast::<Bytes32>().unwrap();
                    let result = item_1 == Bytes32::from(0);

                    self.stack.push(format!("{:x}", result as u128))?;
                }
                InstructionType::AND => {
                    let (item_1, item_2) =
                        *build_initials()?.downcast::<(Bytes32, Bytes32)>().unwrap();
                    let result = item_1 & item_2;

                    self.stack.push(result.parse_and_trim()?)?;
                }
                InstructionType::OR => {
                    let (item_1, item_2) =
                        *build_initials()?.downcast::<(Bytes32, Bytes32)>().unwrap();
                    let result = item_1 | item_2;

                    self.stack.push(result.parse_and_trim()?)?;
                }
                InstructionType::XOR => {
                    let (item_1, item_2) =
                        *build_initials()?.downcast::<(Bytes32, Bytes32)>().unwrap();
                    let result = item_1 ^ item_2;

                    self.stack.push(result.parse_and_trim()?)?;
                }
                InstructionType::NOT => {
                    let item_1 = *build_initials()?.downcast::<Bytes32>().unwrap();
                    let result = !item_1;

                    self.stack.push(result.parse_and_trim()?)?;
                }
                InstructionType::BYTE => {
                    let (item_1, item_2) =
                        *build_initials()?.downcast::<(Bytes32, Bytes32)>().unwrap();

                    let result = if item_1 < Bytes32::from(32) {
                        (item_2 >> (Bytes32::from(8) * (Bytes32::from(31) - item_1)))
                            & Bytes32::from(0xFF)
                    } else {
                        Bytes32::from(0)
                    };

                    self.stack.push(result.parse_and_trim()?)?;
                }
                InstructionType::KECCAK256 => {
                    let item_1 = *build_initials()?.downcast::<Bytes32>().unwrap();

                    // NOTE: need to trim leading zeroes for keccak operation
                    let mut data: Vec<u8> = vec![];
                    for &byte in item_1.0.iter() {
                        if byte != 0 || !data.is_empty() {
                            data.push(byte);
                        }
                    }

                    let mut result = [0u8; 32];
                    let mut keccak = Keccak::v256();
                    keccak.update(&data);
                    keccak.finalize(&mut result);

                    let mut hex_result = String::with_capacity(result.len() * 2);
                    for byte in &result {
                        write!(&mut hex_result, "{:02x}", byte)?;
                    }

                    self.stack.push(hex_result)?;
                }
                InstructionType::POP => {
                    build_initials()?.downcast::<Bytes32>().unwrap();
                }
                InstructionType::MLOAD => {
                    let item_1 = *build_initials()?.downcast::<Bytes32>().unwrap();

                    let result;
                    unsafe {
                        result = self.memory.mload(item_1);
                    }
                    let result: String = result.try_into()?;

                    self.stack.push(result)?;
                }
                InstructionType::MSTORE => unsafe {
                    let (item_1, item_2) =
                        *build_initials()?.downcast::<(Bytes32, Bytes32)>().unwrap();

                    self.memory.mstore(item_1, item_2);
                },
                InstructionType::SLOAD => {
                    let item_1 = *build_initials()?.downcast::<Bytes32>().unwrap();

                    let result = self.storage.sload(item_1).unwrap();
                    let result: String = result.clone().to_string();

                    self.stack.push(result)?;
                }
                InstructionType::SSTORE => {
                    let (item_1, item_2) =
                        *build_initials()?.downcast::<(Bytes32, Bytes32)>().unwrap();

                    self.storage.sstore(item_1, item_2);
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

        if self.verbose {
            self.history.summarize();
        }

        Ok(())
    }

    fn pop_first_item(
        &mut self,
        instruction: InstructionType,
    ) -> Result<(usize, Bytes32), Box<dyn Error>> {
        if self.stack.is_empty() {
            return Err(Box::new(VmError::ShallowStack(Box::leak(Box::new(
                instruction,
            )))));
        }

        let (index, item) = &self.stack.pop()?;

        Ok((*index, Bytes32::from_str(item.as_str())?))
    }

    fn pop_first_two_items(
        &mut self,
        instruction: InstructionType,
    ) -> Result<([usize; 2], [Bytes32; 2]), Box<dyn Error>> {
        if self.stack.length() < 2 {
            return Err(Box::new(VmError::ShallowStack(Box::leak(Box::new(
                instruction,
            )))));
        }

        let (index_1, item_1) = &self.stack.pop()?;
        let (index_2, item_2) = &self.stack.pop()?;

        Ok((
            [*index_1, *index_2],
            [Bytes32::from_str(item_1)?, Bytes32::from_str(item_2)?],
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_creates_vm() -> Result<(), Box<dyn Error>> {
        let bytecode = "0x8060";

        let vm = create_vm(&bytecode)?;

        assert_eq!(vm.stack.is_empty(), true);
        assert_eq!(vm.lexer.bytecode, bytecode.strip_prefix("0x").unwrap());
        assert_eq!(vm.memory.msize(), 0);
        assert_eq!(vm.storage.size(), 0);
        assert_eq!(vm.history.size(), 0);

        Ok(())
    }

    #[test]
    fn it_runs_add_opcode() -> Result<(), Box<dyn Error>> {
        // NOTE: 10 + 20 = 30 which is 1e in hex
        let bytecode = "6014600a01";

        let mut vm = create_vm(&bytecode)?;
        vm.run()?;

        assert_eq!(vm.stack.peek().unwrap(), "1e");

        // NOTE: (10 + 20) + 32 = 62 which is 3e in hex
        let bytecode = "6020600a60140101";

        let mut vm = create_vm(&bytecode)?;
        vm.run()?;

        assert_eq!(vm.stack.peek().unwrap(), "3e");

        Ok(())
    }

    #[test]
    fn it_runs_mul_opcode() -> Result<(), Box<dyn Error>> {
        // NOTE: 10 * 20 = 200 which is c8 in hex
        let bytecode = "6014600a02";

        let mut vm = create_vm(&bytecode)?;
        vm.run()?;

        assert_eq!(vm.stack.peek().unwrap(), "c8");

        // NOTE: (10 * 20) * 2 = 400 which is 190 in hex
        let bytecode = "60026014600a0202";

        let mut vm = create_vm(&bytecode)?;
        vm.run()?;

        assert_eq!(vm.stack.peek().unwrap(), "190");

        Ok(())
    }

    #[test]
    fn it_runs_sub_opcode() -> Result<(), Box<dyn Error>> {
        // NOTE: 20 - 10 = 10 which is a in hex
        let bytecode = "600a601403";

        let mut vm = create_vm(&bytecode)?;
        vm.run()?;

        assert_eq!(vm.stack.peek().unwrap(), "a");

        Ok(())
    }

    #[test]
    fn it_runs_div_opcode() -> Result<(), Box<dyn Error>> {
        // NOTE: 5 / 2 = rounded as 2 which is 2 in hex
        let bytecode = "6002600504";

        let mut vm = create_vm(&bytecode)?;
        vm.run()?;

        assert_eq!(vm.stack.peek().unwrap(), "2");

        Ok(())
    }

    #[test]
    fn it_runs_mod_opcode() -> Result<(), Box<dyn Error>> {
        // NOTE: 5 % 2 = 1 which is 1 in hex
        let bytecode = "6002600506";

        let mut vm = create_vm(&bytecode)?;
        vm.run()?;

        assert_eq!(vm.stack.peek().unwrap(), "1");

        Ok(())
    }

    #[test]
    fn it_runs_exp_opcode() -> Result<(), Box<dyn Error>> {
        // NOTE: 5**2 = 25 which is 19 in hex
        let bytecode = "600260050a";

        let mut vm = create_vm(&bytecode)?;
        vm.run()?;

        assert_eq!(vm.stack.peek().unwrap(), "19");

        Ok(())
    }

    #[test]
    fn it_runs_lt_opcode() -> Result<(), Box<dyn Error>> {
        // NOTE: 5 < 10 = true which is 1 in hex
        let bytecode = "600a600510";

        let mut vm = create_vm(&bytecode)?;
        vm.run()?;

        assert_eq!(vm.stack.peek().unwrap(), "1");

        Ok(())
    }

    #[test]
    fn it_runs_gt_opcode() -> Result<(), Box<dyn Error>> {
        // NOTE: 20 > 10 = true which is 1 in hex
        let bytecode = "600a601411";

        let mut vm = create_vm(&bytecode)?;
        vm.run()?;

        assert_eq!(vm.stack.peek().unwrap(), "1");

        Ok(())
    }

    #[test]
    fn it_runs_eq_opcode() -> Result<(), Box<dyn Error>> {
        // NOTE: 10 == 10 = true which is 1 in hex
        let bytecode = "600a600a14";

        let mut vm = create_vm(&bytecode)?;
        vm.run()?;

        assert_eq!(vm.stack.peek().unwrap(), "1");

        Ok(())
    }

    #[test]
    fn it_runs_iszero_opcode() -> Result<(), Box<dyn Error>> {
        // NOTE: 10 == 0 = false which is 0 in hex
        let bytecode = "600a15";

        let mut vm = create_vm(&bytecode)?;
        vm.run()?;

        assert_eq!(vm.stack.peek().unwrap(), "0");

        Ok(())
    }

    #[test]
    fn it_runs_and_opcode() -> Result<(), Box<dyn Error>> {
        // NOTE: 1 & 1 = 1 which is 1 in hex
        let bytecode = "6001600116";

        let mut vm = create_vm(&bytecode)?;
        vm.run()?;

        assert_eq!(vm.stack.peek().unwrap(), "1");

        Ok(())
    }

    #[test]
    fn it_runs_or_opcode() -> Result<(), Box<dyn Error>> {
        // NOTE: 1 | 0 = 1 which is 1 in hex
        let bytecode = "6000600117";

        let mut vm = create_vm(&bytecode)?;
        vm.run()?;

        assert_eq!(vm.stack.peek().unwrap(), "1");

        Ok(())
    }

    #[test]
    fn it_runs_xor_opcode() -> Result<(), Box<dyn Error>> {
        // NOTE: 1 ^ 1 = 0 which is 0 in hex
        let bytecode = "6001600118";

        let mut vm = create_vm(&bytecode)?;
        vm.run()?;

        assert_eq!(vm.stack.peek().unwrap(), "0");

        Ok(())
    }

    #[test]
    fn it_runs_not_opcode() -> Result<(), Box<dyn Error>> {
        // NOTE: !0 = [f; 32] which is ff..ff in hex
        let bytecode = "600019";

        let mut vm = create_vm(&bytecode)?;
        vm.run()?;

        assert_eq!(vm.stack.peek().unwrap(), "1");

        Ok(())
    }

    #[test]
    fn it_runs_byte_opcode() -> Result<(), Box<dyn Error>> {
        // NOTE: pushes 0xff to the stack and extracts its 31st byte which is ff = 255
        let bytecode = "60ff601f1a";

        let mut vm = create_vm(&bytecode)?;
        vm.run()?;

        assert_eq!(vm.stack.peek().unwrap(), "ff");
        assert_eq!(vm.stack.length(), 1);

        Ok(())
    }

    #[test]
    fn it_runs_keccak256_opcode() -> Result<(), Box<dyn Error>> {
        // NOTE: keccaks word "hello"
        // result must be 1c8aff950685c2ed4bc3174f3472287b56d9517b9c948127319a09a7a36deac8
        // and found by using the command `cast keccak hello`

        // hello in hex string
        let data = "68656c6c6f";

        let bytecode = format!("64{data}20");
        let mut vm = create_vm(&bytecode)?;

        vm.run()?;

        assert_eq!(
            vm.stack.peek().unwrap(),
            "1c8aff950685c2ed4bc3174f3472287b56d9517b9c948127319a09a7a36deac8"
        );
        assert_eq!(vm.stack.length(), 1);

        Ok(())
    }

    #[test]
    fn it_runs_pop_opcode() -> Result<(), Box<dyn Error>> {
        // NOTE: it pushes number 1 and number 2 to stack in order,
        // then it pops an item from top
        let bytecode = "6001600250";

        let mut vm = create_vm(&bytecode)?;
        vm.run()?;

        assert_eq!(vm.stack.peek().unwrap(), "01");

        Ok(())
    }

    #[test]
    fn it_runs_mstore_and_mload_opcodes() -> Result<(), Box<dyn Error>> {
        // NOTE: pushes 0x20(32) and 0x80(memory location), and saves it on memory
        let bytecode = "6020608052";

        let mut vm = create_vm(&bytecode)?;
        vm.run()?;

        let data;
        unsafe {
            data = vm.memory.mload("80".parse::<Bytes32>()?);
        }

        let data: u128 = data.try_into()?;

        assert_eq!(vm.stack.is_empty(), true);
        assert_eq!(data, 32);

        Ok(())
    }

    #[test]
    fn it_runs_sstore_and_sload_opcodes() -> Result<(), Box<dyn Error>> {
        // NOTE: saves word "hello" in the slot of 1

        // hello in hex string
        let data = "68656c6c6f";
        let bytecode = format!("64{data}600155");

        let mut vm = create_vm(&bytecode)?;
        vm.run()?;

        let result = vm.storage.sload("01".parse::<Bytes32>()?);

        assert_eq!(data, result.unwrap().parse_and_trim()?);

        Ok(())
    }

    #[test]
    fn it_runs_push_opcode() -> Result<(), Box<dyn Error>> {
        // NOTE: pushes 12 1 in the stack
        let bytecode = "6b010101010101010101010101";

        let mut vm = create_vm(bytecode)?;
        vm.run()?;

        assert_eq!(vm.stack.peek().unwrap(), "010101010101010101010101");

        Ok(())
    }

    #[test]
    fn it_runs_dup_opcode() -> Result<(), Box<dyn Error>> {
        // NOTE: duplicates 3rd stack item
        let bytecode = "6b010101010101010101010101";

        let mut vm = create_vm(bytecode)?;
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

        let mut vm = create_vm(bytecode)?;
        vm.run()?;

        assert_eq!(vm.stack.peek().unwrap(), "01");

        Ok(())
    }

    // NOTE: helper function
    fn create_vm(bytecode: &str) -> Result<Vm, Box<dyn Error>> {
        Ok(Vm::new(bytecode, false)?)
    }
}
