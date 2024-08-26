use crate::utils::bytes32::Bytes32;

#[derive(Debug)]
pub struct Memory {
    pub heap: Vec<u8>,
}

impl Default for Memory {
    fn default() -> Self {
        Self::new()
    }
}

impl Memory {
    pub fn new() -> Self {
        Self { heap: vec![0; 0] }
    }

    fn extend(&mut self, size: usize) {
        self.heap.extend(vec![0; size]);
    }

    /// Documentation
    ///
    /// # Safety
    ///
    /// As Memory::mload, it loads data from given location pointer.
    pub unsafe fn mload(&mut self, location: Bytes32) -> Bytes32 {
        let location: usize = location.try_into().unwrap();
        let extended_location = location + 32;

        if extended_location > self.heap.len() {
            if location % 32 == 0 {
                self.extend(extended_location - self.heap.len());
            } else {
                self.extend(extended_location + (location % 32) - self.heap.len());
            };
        }

        let ptr = self.heap.as_ptr().add(location) as *const Bytes32;

        unsafe { *ptr }
    }

    /// Documentation
    ///
    /// # Safety
    ///
    /// As Memory::mstore, it stores given data to given location pointer.
    pub unsafe fn mstore(&mut self, location: Bytes32, data: Bytes32) {
        let location: usize = location.try_into().unwrap();
        let extended_location = location + 32;

        if extended_location > self.heap.len() {
            self.extend(extended_location - self.heap.len());
        }

        let ptr = self.heap.as_mut_ptr().add(location) as *mut [u8; 32];

        *ptr = data.0;
    }

    /// Documentation
    ///
    /// # Safety
    ///
    /// As Memory::load_only, it loads data from given location pointer.
    pub unsafe fn load_only(&self, location: Bytes32) -> Bytes32 {
        let location: usize = location.try_into().unwrap();

        let ptr = self.heap.as_ptr().add(location) as *const Bytes32;

        unsafe { *ptr }
    }

    pub fn msize(&self) -> usize {
        self.heap.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_creates_memory_heap() {
        let memory = Memory::new();

        assert_eq!(memory.heap.is_empty(), true);
    }

    #[test]
    fn it_extends_memory() {
        let mut memory = Memory::new();

        memory.extend(32);

        assert_eq!(memory.heap.is_empty(), false);
        assert_eq!(memory.msize(), 32);
    }

    #[test]
    fn it_stores_and_loads_data_which_is_multiplication_of_32_in_memory() {
        let mut memory = Memory::new();

        let data = "ff1122".parse::<Bytes32>().unwrap();
        let mem_location = Bytes32::from(0);

        let result: Bytes32;
        unsafe {
            memory.mstore(mem_location, data);

            result = memory.mload(mem_location);
        }

        assert_eq!(result, data);
    }

    #[test]
    fn it_stores_and_loads_data_which_is_not_multiplication_of_32_in_memory() {
        let mut memory = Memory::new();

        let data = "ff1122".parse::<Bytes32>().unwrap();
        let mem_location = Bytes32::from(37);
        let mem_upper_limit = 37 + 32;

        let result: Bytes32;
        unsafe {
            memory.mstore(mem_location, data);

            result = memory.mload(mem_location);
        }

        assert_eq!(result, data);
        assert_eq!(memory.msize(), mem_upper_limit);
    }
}
