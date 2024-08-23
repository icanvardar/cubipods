use super::utils::errors::InstructionError;
use std::str::FromStr;

pub struct Instruction<'a> {
    pub r#type: InstructionType,
    pub literal: &'a str,
}

#[derive(Clone, Debug)]
#[repr(u8)]
pub enum InstructionType {
    STOP = 0x00,
    ADD = 0x01,
    MUL = 0x02,
    SUB = 0x03,
    DIV = 0x04,
    MOD = 0x06,
    EXP = 0x0a,
    LT = 0x10,
    GT = 0x11,
    EQ = 0x14,
    ISZERO = 0x15,
    AND = 0x16,
    OR = 0x17,
    XOR = 0x18,
    NOT = 0x19,
    BYTE = 0x1a,
    KECCAK256 = 0x20,
    POP = 0x50,
    MLOAD = 0x51,
    MSTORE = 0x52,
    SLOAD = 0x54,
    SSTORE = 0x55,
    PUSH(u8),
    DUP(u8),
    SWAP(u8),
}

impl FromStr for InstructionType {
    type Err = InstructionError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let tmp = u128::from_str_radix(s, 16).unwrap();
        match tmp {
            0x00 => Ok(InstructionType::STOP),
            0x01 => Ok(InstructionType::ADD),
            0x02 => Ok(InstructionType::MUL),
            0x03 => Ok(InstructionType::SUB),
            0x04 => Ok(InstructionType::DIV),
            0x06 => Ok(InstructionType::MOD),
            0x0a => Ok(InstructionType::EXP),
            0x10 => Ok(InstructionType::LT),
            0x11 => Ok(InstructionType::GT),
            0x14 => Ok(InstructionType::EQ),
            0x15 => Ok(InstructionType::ISZERO),
            0x16 => Ok(InstructionType::AND),
            0x17 => Ok(InstructionType::OR),
            0x18 => Ok(InstructionType::XOR),
            0x19 => Ok(InstructionType::NOT),
            0x1a => Ok(InstructionType::BYTE),
            0x20 => Ok(InstructionType::KECCAK256),
            0x50 => Ok(InstructionType::POP),
            0x51 => Ok(InstructionType::MLOAD),
            0x52 => Ok(InstructionType::MSTORE),
            0x54 => Ok(InstructionType::SLOAD),
            0x55 => Ok(InstructionType::SSTORE),
            0x5f..=0x7f => Ok(InstructionType::PUSH((tmp % 0x5f) as u8)),
            0x80..=0x8f => Ok(InstructionType::DUP(((tmp % 0x80) + 1) as u8)),
            0x90..=0x9f => Ok(InstructionType::SWAP(((tmp % 0x90) + 1) as u8)),
            _ => {
                let mut array = [0; 2];
                let bytes = s.as_bytes();
                let len = bytes.len().min(2);
                array[..len].copy_from_slice(&bytes[..len]);

                Err(InstructionError::InvalidInstruction(array))
            }
        }
    }
}
