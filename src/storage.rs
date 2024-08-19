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
    use super::*;

    #[test]
    fn it_creates_storage() {
        let storage = Storage::new();

        assert_eq!(storage.storage.len(), 0);
        assert_eq!(storage.storage.is_empty(), true);
    }

    #[test]
    fn it_stores_data() {}

    #[test]
    fn it_loads_data() {}
}
