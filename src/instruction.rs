pub enum Instruction {
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

impl Instruction {
    pub fn to_u8(self) -> u8 {
        self as u8
    }
}
