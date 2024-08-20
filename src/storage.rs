use std::collections::HashMap;

#[derive(Default)]
pub struct Storage {
    storage: HashMap<[u8; 32], [u8; 32]>,
}

impl Storage {
    pub fn new() -> Self {
        Self {
            ..Default::default()
        }
    }

    pub fn sstore(&mut self, key: [u8; 32], value: [u8; 32]) {
        self.storage.insert(key, value);
    }

    pub fn sload(&self, key: [u8; 32]) -> Option<&[u8; 32]> {
        self.storage.get(&key)
    }
}

#[cfg(test)]
mod tests {
    use crate::utils;

    use super::*;

    #[test]
    fn it_creates_storage() {
        let storage = Storage::new();

        assert_eq!(storage.storage.len(), 0);
        assert_eq!(storage.storage.is_empty(), true);
    }

    #[test]
    fn it_stores_and_loads_data() {
        let mut storage = Storage::new();

        let key = utils::bytes::to_u8_32(1);
        let value = utils::bytes::to_u8_32("hello".as_bytes());

        storage.sstore(key, value);

        assert_eq!(storage.sload(key), Some(value).as_ref());
    }
}
