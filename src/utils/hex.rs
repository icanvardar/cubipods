pub fn to_u8_32(data: impl Into<DataType>) -> [u8; 32] {
    let mut array = [0; 32];

    match data.into() {
        DataType::Bytes(value) => {
            let len = value.len().min(32);
            array[..len].copy_from_slice(&value[..len]);
        }
        DataType::String(value) => {
            let bytes = value.as_bytes();
            let len = bytes.len().min(32);
            array[..len].copy_from_slice(&bytes[..len])
        }
        DataType::Integer(value) => {
            let bytes = value.to_be_bytes();
            let len = bytes.len().min(32);
            array[32 - len..].copy_from_slice(&bytes)
        }
        DataType::Array(value) => {
            let len = value.len().min(32);
            array[..len].copy_from_slice(&value[..len]);
        }
    }

    array
}

pub fn from_u8_32<T: To>(array: [u8; 32]) -> T {
    T::to(array)
}

pub enum DataType {
    Bytes(Vec<u8>),
    String(String),
    Integer(u128),
    Array(Vec<u8>),
}

// NOTE:: from relevant DataType to [u8; 32]
impl From<&[u8]> for DataType {
    fn from(value: &[u8]) -> Self {
        DataType::Bytes(value.to_vec())
    }
}

impl From<&String> for DataType {
    fn from(value: &String) -> Self {
        DataType::String(value.to_string())
    }
}

impl From<u128> for DataType {
    fn from(value: u128) -> Self {
        DataType::Integer(value)
    }
}

impl From<[u8; 8]> for DataType {
    fn from(value: [u8; 8]) -> Self {
        DataType::Array(value.to_vec())
    }
}

impl From<[u8; 16]> for DataType {
    fn from(value: [u8; 16]) -> Self {
        DataType::Array(value.to_vec())
    }
}

impl From<[u8; 32]> for DataType {
    fn from(value: [u8; 32]) -> Self {
        DataType::Array(value.to_vec())
    }
}

// NOTE: from [u8; 32] to relevant DataType
pub trait To {
    fn to(array: [u8; 32]) -> Self;
}

impl To for Vec<u8> {
    fn to(array: [u8; 32]) -> Self {
        array.iter().cloned().take_while(|&x| x != 0).collect()
    }
}

impl To for String {
    fn to(array: [u8; 32]) -> Self {
        String::from_utf8(array.iter().cloned().take_while(|&x| x != 0).collect())
            .unwrap_or_default()
    }
}

impl To for u128 {
    fn to(array: [u8; 32]) -> Self {
        let mut bytes = [0u8; 16];
        bytes.copy_from_slice(&array[16..]);
        u128::from_be_bytes(bytes)
    }
}

impl To for [u8; 8] {
    fn to(array: [u8; 32]) -> Self {
        array[..8].try_into().unwrap()
    }
}

impl To for [u8; 16] {
    fn to(array: [u8; 32]) -> Self {
        array[..16].try_into().unwrap()
    }
}

impl To for [u8; 32] {
    fn to(array: [u8; 32]) -> Self {
        array
    }
}
