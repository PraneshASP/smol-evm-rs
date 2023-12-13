use crate::{memory::Memory, stack::Stack};
use bytes::Bytes;

#[derive(Debug)]
pub struct ExecutionContext {
    pub code: Bytes,
    pub stack: Stack,
    pub memory: Memory,
    pub pc: usize,
    pub stopped: bool,
    pub returndata: Bytes,
}

impl ExecutionContext {
    pub fn new(code: Bytes) -> Self {
        Self {
            code,
            stack: Stack::new(1024),
            memory: Memory::new(),
            pc: 0,
            stopped: false,
            returndata: Bytes::new(),
        }
    }

    pub fn stop(&mut self) {
        self.stopped = true;
    }

    /// Returns the next num_bytes from the code buffer as an integer and advances pc by num_bytes.
    pub fn read_code(&mut self, num_bytes: usize) -> usize {
        let bytes_slice = &self.code[self.pc..self.pc + num_bytes];
        self.pc += num_bytes;

        let mut result: usize = 0;
        for &byte in bytes_slice.iter() {
            result = (result << 8) | (byte as usize);
        }

        result
    }

    pub fn set_returndata(&mut self, offset: usize, length: usize) {
        self.stopped = true;
        self.returndata = self.memory.load_range(offset, length);
    }
}
