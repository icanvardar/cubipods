use std::collections::HashMap;

use crate::utils::bytes32::Bytes32;

#[derive(Default)]
pub struct Storage {
    storage: HashMap<Bytes32, Bytes32>,
}

impl Storage {
    pub fn new() -> Self {
        Self {
            ..Default::default()
        }
    }

    pub fn sstore(&mut self, slot: Bytes32, value: Bytes32) {
        self.storage.insert(slot, value);
    }

    pub fn sload(&self, slot: Bytes32) -> Option<&Bytes32> {
        self.storage.get(&slot)
    }

    pub fn size(&self) -> usize {
        self.storage.len()
    }

    pub fn is_empty(&self) -> bool {
        self.storage.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use std::error::Error;

    use super::*;

    #[test]
    fn it_creates_storage() {
        let storage = Storage::new();

        assert_eq!(storage.storage.len(), 0);
        assert_eq!(storage.storage.is_empty(), true);
    }

    #[test]
    fn it_stores_and_loads_data() -> Result<(), Box<dyn Error>> {
        let mut storage = Storage::new();

        let slot = Bytes32::from(1);
        let value = "68656c6c6f".parse::<Bytes32>()?;

        storage.sstore(slot, value);

        assert_eq!(storage.sload(slot), Some(value).as_ref());

        Ok(())
    }

    #[test]
    fn it_returns_storage_size() {
        let storage = Storage::new();

        let size = storage.size();

        assert_eq!(size, 0);
    }

    #[test]
    fn it_checks_storage_emptiness() {
        let storage = Storage::new();

        let is_empty = storage.is_empty();

        assert_eq!(is_empty, true);
    }
}
