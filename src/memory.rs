use bytes::{BufMut, Bytes, BytesMut};

#[derive(Debug)]
pub struct Memory {
    pub memory: Vec<usize>,
}

#[derive(Debug)]
pub enum MemoryError {
    InvalidMemoryAccess(usize, usize),
    InvalidMemoryValue(usize, usize),
    InvalidOffset(usize),
}

impl Memory {
    const ZERO_WORD: [usize; 16] = [0; 16];
    pub fn new() -> Self {
        Self { memory: Vec::new() }
    }

    pub fn store(&mut self, offset: usize, value: usize) -> Result<(), MemoryError> {
        if value >= u8::MAX.into() {
            return Err(MemoryError::InvalidMemoryValue(offset, value));
        }

        self.expand_if_needed(offset);
        self.memory[offset] = value;
        Ok(())
    }

    pub fn load(&mut self, offset: usize) -> usize {
        if offset > self.memory.len() {
            return 0;
        }
        self.expand_if_needed(offset);
        self.memory[offset]
    }

    pub fn load_range(&mut self, offset: usize, length: usize) -> Bytes {
        self.expand_if_needed(offset);
        let mut bytes = BytesMut::new();
        for i in offset..offset + length {
            let data = self.load(i).to_ne_bytes(); // Convert usize to [u8; 8]
            for byte in &data {
                bytes.put_u8(*byte);
            }
        }
        bytes.freeze()
    }

    // TODO:
    // Currently handles only upto 16 bytes
    // pub fn load_word(&mut self, offset: usize) -> u128 {
    //     let range = self.load_range(offset, 16); // Ensure loading 16 bytes for a u128
    //     let mut bytes = [0u8; 16];
    //     // Copy bytes from 'range' into 'bytes' array
    //     for (i, &byte) in range.iter().enumerate() {
    //         bytes[i] = byte;
    //     }

    //     // Convert the bytes array into a u128
    //     u128::from_be_bytes(bytes)
    // }

    pub fn store_word(&mut self, offset: usize, value: usize) {
        self.expand_if_needed(offset + 16);

        for i in 0..16 {
            self.memory[offset + 16 - i] = value & (255_u128.wrapping_shl((i * 8) as u32)) as usize;
        }
    }

    pub fn active_words(&self) -> usize {
        match self.memory.len().checked_div(16) {
            Some(v) => v,
            None => todo!(),
        }
    }

    fn expand_if_needed(&mut self, offset: usize) {
        if offset < self.memory.len() {
            return;
        }
        let active_words_after = std::cmp::max(self.active_words(), (offset + 1).div_ceil(16));
        let additional_words = active_words_after.saturating_sub(self.active_words());
        for _ in 0..additional_words {
            self.memory.extend_from_slice(&Self::ZERO_WORD);
        }
        return;
    }
}

#[cfg(test)]
mod tests {

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
        assert_eq!(memory.memory.len(), 16);

        memory.store(2, value).unwrap();
        assert_eq!(memory.memory.len(), 16);

        memory.store(15, value).unwrap();
        assert_eq!(memory.memory.len(), 16);

        // Activate 2nd word
        memory.store(17, value).unwrap();
        assert_eq!(memory.memory.len(), 32);
    }

    #[test]
    fn test_active_words() {
        let offset = 0;
        let value = 10;
        let mut memory = Memory::new();
        let _ = memory.store(offset, value);
        assert_eq!(memory.active_words(), 1);

        memory.store(2, value).unwrap();
        assert_eq!(memory.active_words(), 1);

        memory.store(15, value).unwrap();
        assert_eq!(memory.active_words(), 1);

        // Activate 3rd word
        memory.store(35, value).unwrap();
        assert_eq!(memory.active_words(), 3);

        // Activate 5th word
        memory.store(65, value).unwrap();
        assert_eq!(memory.active_words(), 5);
    }

    #[test]
    fn load_from_memory() {
        let offset = 0;
        let value = 10;
        let mut memory = Memory::new();
        let _ = memory.store(offset, value);
        let value = memory.load(offset);
        assert_eq!(value, 10);
    }
}
