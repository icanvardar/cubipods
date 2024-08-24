use std::{error::Error, str::FromStr};

use super::errors::Bytes32Error;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct Bytes32([u8; 32]);

impl FromStr for Bytes32 {
    type Err = Bytes32Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut bytes = [0u8; 32];
        let s = hex::decode(s.trim_start_matches("0x")).map_err(|_| Bytes32Error::InvalidStr)?;
        bytes[32 - s.len()..].copy_from_slice(&s);
        Ok(Bytes32(bytes))
    }
}

impl ToString for Bytes32 {
    fn to_string(&self) -> String {
        hex::encode(&self.0)
    }
}

impl TryInto<String> for Bytes32 {
    type Error = Bytes32Error;

    fn try_into(self) -> Result<String, Self::Error> {
        Ok(hex::encode(&self.0))
    }
}

impl From<bool> for Bytes32 {
    fn from(value: bool) -> Self {
        let mut bytes = [0u8; 32];
        bytes[31] = if value { 1 } else { 0 };
        Bytes32(bytes)
    }
}

impl TryInto<bool> for Bytes32 {
    type Error = Bytes32Error;

    fn try_into(self) -> Result<bool, Self::Error> {
        Ok(self.0[31] != 0)
    }
}

impl From<u128> for Bytes32 {
    fn from(value: u128) -> Self {
        let mut bytes = [0u8; 32];
        bytes[16..32].copy_from_slice(&value.to_be_bytes());
        Bytes32(bytes)
    }
}

impl TryInto<u128> for Bytes32 {
    type Error = Bytes32Error;

    fn try_into(self) -> Result<u128, Self::Error> {
        Ok(u128::from_be_bytes(
            self.0[16..32]
                .try_into()
                .map_err(|_| Bytes32Error::U128ConversionFailed)?,
        ))
    }
}

impl Bytes32 {
    pub fn cast_with_size(self, size: usize) -> Result<String, Box<dyn Error>> {
        let result: String = self.try_into()?;
        let lower_limit = result.len() - (size * 2);

        Ok(result[lower_limit..result.len()].to_string())
    }
}
