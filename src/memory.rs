#[derive(Default)]
pub struct Memory {
    pub heap: Vec<u8>,
}

enum MemoryError {}

impl Memory {
    pub fn new() -> Self {
        Self { heap: vec![0; 0] }
    }

    pub fn extend(&mut self, size: usize) {
        self.heap.extend(vec![0; size]);
    }

    pub unsafe fn mload(&mut self, location: usize) -> Result<[u8; 32], ()> {
        let extended_location = location + 32;

        if extended_location > self.heap.len() {
            if location % 32 == 0 {
                self.extend(extended_location - self.heap.len());
            } else {
                self.extend(extended_location + (location % 32) - self.heap.len());
            };
        }

        let ptr = self.heap.as_ptr().add(location) as *const [u8; 32];

        unsafe { Ok(*ptr) }
    }

    pub unsafe fn mstore(&mut self, location: usize, data: [u8; 32]) -> Result<(), ()> {
        let extended_location = location + 32;

        if extended_location > self.heap.len() {
            self.extend(extended_location - self.heap.len());
        }

        let ptr = self.heap.as_mut_ptr().add(location) as *mut [u8; 32];

        *ptr = data;

        Ok(())
    }

    pub fn msize(&self) -> usize {
        self.heap.len()
    }
}
