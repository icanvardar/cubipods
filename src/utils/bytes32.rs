use std::{
    error::Error,
    fmt::Display,
    ops::{Add, BitAnd, BitOr, BitXor, Div, Mul, Not, Rem, Shr, Sub},
    str::FromStr,
};

use super::errors::Bytes32Error;

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub struct Bytes32(pub [u8; 32]);

impl Bytes32 {
    pub fn cast_with_size(self, size: usize) -> Result<String, Box<dyn Error>> {
        let result: String = self.try_into()?;
        let lower_limit = result.len() - (size * 2);

        Ok(result[lower_limit..result.len()].to_string())
    }

    pub fn parse_and_trim(self) -> Result<String, Box<dyn Error>> {
        let result: String = self.try_into()?;
        let result = result.trim_start_matches('0').to_string();
        let result = if result.is_empty() {
            "0".to_string()
        } else {
            result
        };

        Ok(result)
    }
}

impl FromStr for Bytes32 {
    type Err = Bytes32Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut bytes = [0u8; 32];
        let s = hex::decode(s.trim_start_matches("0x")).map_err(|_| Bytes32Error::InvalidStr)?;
        bytes[32 - s.len()..].copy_from_slice(&s);
        Ok(Bytes32(bytes))
    }
}

impl Display for Bytes32 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", hex::encode(self.0))
    }
}

impl TryInto<String> for Bytes32 {
    type Error = Bytes32Error;

    fn try_into(self) -> Result<String, Self::Error> {
        Ok(hex::encode(self.0))
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

impl From<i32> for Bytes32 {
    fn from(value: i32) -> Self {
        let mut bytes = [0u8; 32];
        bytes[28..32].copy_from_slice(&value.to_be_bytes());
        Bytes32(bytes)
    }
}

impl TryInto<i32> for Bytes32 {
    type Error = Bytes32Error;

    fn try_into(self) -> Result<i32, Self::Error> {
        Ok(i32::from_be_bytes(
            self.0[28..32]
                .try_into()
                .map_err(|_| Bytes32Error::U128ConversionFailed)?,
        ))
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

impl From<usize> for Bytes32 {
    fn from(value: usize) -> Self {
        let from = 32 - (usize::BITS / 8) as usize;
        let mut bytes = [0u8; 32];
        bytes[from..32].copy_from_slice(&value.to_be_bytes());
        Bytes32(bytes)
    }
}

impl TryInto<usize> for Bytes32 {
    type Error = Bytes32Error;

    fn try_into(self) -> Result<usize, Self::Error> {
        let from = 32 - (usize::BITS / 8) as usize;
        Ok(usize::from_be_bytes(
            self.0[from..32]
                .try_into()
                .map_err(|_| Bytes32Error::U128ConversionFailed)?,
        ))
    }
}

impl Add for Bytes32 {
    type Output = Bytes32;

    fn add(self, rhs: Self) -> Self::Output {
        let a: u128 = self.try_into().unwrap();
        let b: u128 = rhs.try_into().unwrap();

        Bytes32::from(a + b)
    }
}

impl Mul for Bytes32 {
    type Output = Bytes32;

    fn mul(self, rhs: Self) -> Self::Output {
        let a: u128 = self.try_into().unwrap();
        let b: u128 = rhs.try_into().unwrap();

        Bytes32::from(a * b)
    }
}

impl Sub for Bytes32 {
    type Output = Bytes32;

    fn sub(self, rhs: Self) -> Self::Output {
        let a: u128 = self.try_into().unwrap();
        let b: u128 = rhs.try_into().unwrap();

        Bytes32::from(a - b)
    }
}

impl Div for Bytes32 {
    type Output = Bytes32;

    fn div(self, rhs: Self) -> Self::Output {
        let a: u128 = self.try_into().unwrap();
        let b: u128 = rhs.try_into().unwrap();

        Bytes32::from(a / b)
    }
}

impl Rem for Bytes32 {
    type Output = Bytes32;

    fn rem(self, rhs: Self) -> Self::Output {
        let a: u128 = self.try_into().unwrap();
        let b: u128 = rhs.try_into().unwrap();

        Bytes32::from(a % b)
    }
}

impl PartialOrd for Bytes32 {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        let left: u128 = (*self).try_into().unwrap();
        let right: u128 = (*other).try_into().unwrap();

        left.partial_cmp(&right)
    }
}

impl BitAnd for Bytes32 {
    type Output = Bytes32;

    fn bitand(self, rhs: Self) -> Self::Output {
        let a: u128 = self.try_into().unwrap();
        let b: u128 = rhs.try_into().unwrap();

        Bytes32::from(a & b)
    }
}

impl BitOr for Bytes32 {
    type Output = Bytes32;

    fn bitor(self, rhs: Self) -> Self::Output {
        let a: u128 = self.try_into().unwrap();
        let b: u128 = rhs.try_into().unwrap();

        Bytes32::from(a | b)
    }
}

impl BitXor for Bytes32 {
    type Output = Bytes32;

    fn bitxor(self, rhs: Self) -> Self::Output {
        let a: u128 = self.try_into().unwrap();
        let b: u128 = rhs.try_into().unwrap();

        Bytes32::from(a ^ b)
    }
}

impl Not for Bytes32 {
    type Output = Bytes32;

    fn not(self) -> Self::Output {
        let a: bool = self.try_into().unwrap();

        Bytes32::from(!a)
    }
}

pub trait Pow {
    type Output;

    fn pow(self, exponent: Self) -> Self::Output;
}

impl Pow for Bytes32 {
    type Output = Bytes32;

    fn pow(self, exponent: Self) -> Self::Output {
        let a: u128 = self.try_into().unwrap();
        let b: u128 = exponent.try_into().unwrap();

        Bytes32::from(u128::pow(a, b as u32))
    }
}

impl Shr for Bytes32 {
    type Output = Bytes32;

    fn shr(self, rhs: Self) -> Self::Output {
        let a: u128 = self.try_into().unwrap();
        let b: u128 = rhs.try_into().unwrap();

        Bytes32::from(a >> b)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_str_to_bytes32() {
        let result = Bytes32::from_str("8060202020").unwrap();

        let mut expected = [0u8; 32];
        expected[31] = 32;
        expected[30] = 32;
        expected[29] = 32;
        expected[28] = 96;
        expected[27] = 128;

        assert_eq!(result.0[31], expected[31]);
        assert_eq!(result.0[30], expected[30]);
        assert_eq!(result.0[29], expected[29]);
        assert_eq!(result.0[28], expected[28]);
        assert_eq!(result.0[27], expected[27]);
        assert_eq!(result.0.len(), expected.len());
    }

    #[test]
    fn test_to_string_from_bytes32() {
        let data = "8060202020";
        let result = Bytes32::from_str(data).unwrap();
        let result = result.to_string();

        assert_eq!(
            result,
            "0000000000000000000000000000000000000000000000000000008060202020"
        );
    }

    #[test]
    fn test_try_into_string_from_bytes32() -> Result<(), Box<dyn Error>> {
        let data = "8060202020";
        let result = Bytes32::from_str(data).unwrap();
        let result: String = result.try_into()?;

        assert_eq!(
            result,
            "0000000000000000000000000000000000000000000000000000008060202020"
        );

        Ok(())
    }

    #[test]
    fn test_from_bool_to_bytes32() {
        let result = Bytes32::from(true);

        assert_eq!(result.0[31], 1);
    }

    #[test]
    fn test_try_into_bool_from_bytes32() -> Result<(), Box<dyn Error>> {
        let data = true;
        let result = Bytes32::from(data);
        let result: bool = result.try_into()?;

        assert_eq!(result, data);

        Ok(())
    }

    #[test]
    fn test_from_i32_to_bytes32() {
        let data: i32 = 1024;
        let result = Bytes32::from(data);

        assert_eq!(result.0[30], 4);
    }

    #[test]
    fn test_try_into_i32_from_bytes32() -> Result<(), Box<dyn Error>> {
        let data: i32 = 1024;
        let result = Bytes32::from(data);
        let result: i32 = result.try_into()?;

        assert_eq!(result, data);

        Ok(())
    }

    #[test]
    fn test_from_u128_to_bytes32() {
        let data: u128 = 1024;
        let result = Bytes32::from(data);

        assert_eq!(result.0[30], 4);
    }

    #[test]
    fn test_try_into_u128_from_bytes32() -> Result<(), Box<dyn Error>> {
        let data: u128 = 1024;
        let result = Bytes32::from(data);
        let result: u128 = result.try_into()?;

        assert_eq!(result, data);

        Ok(())
    }

    #[test]
    fn test_from_usize_to_bytes32() {
        let data: usize = 1024;
        let result = Bytes32::from(data);

        assert_eq!(result.0[30], 4);
    }

    #[test]
    fn test_try_into_usize_from_bytes32() -> Result<(), Box<dyn Error>> {
        let data: usize = 1024;
        let result = Bytes32::from(data);
        let result: usize = result.try_into()?;

        assert_eq!(result, data);

        Ok(())
    }

    #[test]
    fn test_cast_with_size() -> Result<(), Box<dyn Error>> {
        let data = "8060202020";
        let result = Bytes32::from_str(data)?;
        let result = result.cast_with_size(5)?;

        assert_eq!(result, data);

        Ok(())
    }
}
