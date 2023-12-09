pub struct Memory {
    pub memory: Vec<usize>,
}

#[derive(Debug)]
pub enum MemoryError {
    InvalidMemoryAccess(usize, usize),
    InvalidMemoryValue(usize, usize),
}

impl Memory {
    pub fn new() -> Self {
        Self { memory: Vec::new() }
    }

    pub fn store(&mut self, offset: usize, value: usize) -> Result<(), MemoryError> {
        if offset > usize::MAX {
            return Err(MemoryError::InvalidMemoryAccess(offset, value));
        }
        if value > usize::MAX {
            return Err(MemoryError::InvalidMemoryValue(offset, value));
        }
        // Memory expansion
        if self.memory.len() <= offset {
            self.memory.resize(1, 0);
        }

        self.memory[offset] = value;
        Ok(())
    }

    pub fn load(&mut self, offset: usize) -> Result<usize, MemoryError> {
        if offset > self.memory.len() {
            return Ok(0);
        }

        Ok(self.memory[offset])
    }
}

#[cfg(test)]
mod tests {
    use std::mem;

    use super::*;
    #[test]
    fn new_memory_is_empty() {
        let memory = Memory::new();
        assert_eq!(memory.memory.len(), 0);
    }

    #[test]
    fn store_in_memory() {
        let offset = 0;
        let value = 10;
        let mut memory = Memory::new();
        let _ = memory.store(offset, value);
        assert_eq!(memory.memory.len(), 1);
    }

    #[test]
    fn load_from_memory() {
        let offset = 0;
        let value = 10;
        let mut memory = Memory::new();
        let _ = memory.store(offset, value);
        let value = memory.load(offset).unwrap();
        assert_eq!(value, 10);
    }
}
