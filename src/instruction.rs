use std::{error::Error, fmt::Display, str::FromStr};

pub struct Instruction<'a> {
    pub r#type: InstructionType,
    pub literal: &'a str,
}

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
    SHL = 0x1b,
    SHR = 0x1c,
    SAR = 0x1d,
    KECCAK256 = 0x20,
    ADDRESS = 0x30,
}

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

impl FromStr for InstructionType {
    type Err = InstructionError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.as_bytes() {
            b"00" => Ok(InstructionType::STOP),
            b"01" => Ok(InstructionType::ADD),
            b"02" => Ok(InstructionType::MUL),
            b"03" => Ok(InstructionType::SUB),
            b"04" => Ok(InstructionType::DIV),
            b"06" => Ok(InstructionType::MOD),
            b"0a" => Ok(InstructionType::EXP),
            b"10" => Ok(InstructionType::LT),
            b"11" => Ok(InstructionType::GT),
            b"14" => Ok(InstructionType::EQ),
            b"15" => Ok(InstructionType::ISZERO),
            b"16" => Ok(InstructionType::AND),
            b"17" => Ok(InstructionType::OR),
            b"18" => Ok(InstructionType::XOR),
            b"19" => Ok(InstructionType::NOT),
            b"1a" => Ok(InstructionType::BYTE),
            b"1b" => Ok(InstructionType::SHL),
            b"1c" => Ok(InstructionType::SHR),
            b"1d" => Ok(InstructionType::SAR),
            b"20" => Ok(InstructionType::KECCAK256),
            b"30" => Ok(InstructionType::ADDRESS),
            _ => Err(InstructionError::InvalidInstruction),
        }
    }
}
