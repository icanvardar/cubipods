#[derive(Default, Debug)]
pub struct Lexer<'a> {
    pub bytecode: &'a [u8],
    pub position: u64,
    pub read_position: usize,
    pub ch: u8,
}

impl<'a> Lexer<'a> {
    pub fn from_hex_string(hex_string: &'a str) -> Result<Self, String> {
        let hex_string = hex_string.strip_prefix("0x").ok_or("Missing '0x' prefix")?;

        if hex_string.len() % 2 != 0 {
            return Err("Hex string has an odd length".into());
        }

        let bytecode: Result<Vec<u8>, String> = (0..hex_string.len())
            .step_by(2)
            .map(|i| u8::from_str_radix(&hex_string[i..i + 2], 16).map_err(|e| e.to_string()))
            .collect();

        Ok(Self {
            bytecode: bytecode?,
            ..Default::default()
        })
    }

    pub fn from_bytes(bytecode: &'a [u8]) -> Self {
        Self {
            bytecode,
            ..Default::default()
        }
    }

    pub fn read_byte(&mut self) {
        if self.read_position >= self.bytecode.len() {
            self.ch = 0;
        } else {
            self.ch = self.bytecode[self.read_position]
        }
    }
}
