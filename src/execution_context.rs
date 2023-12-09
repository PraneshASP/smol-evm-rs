use crate::{memory::Memory, stack::Stack};
use bytes::Bytes;

pub struct ExecutionContext {
    pub code: Bytes,
    pub stack: Stack,
    pub memory: Memory,
    pub pc: usize,
    pub stopped: bool,
}

impl ExecutionContext {
    pub fn new() -> Self {
        Self {
            code: Bytes::new(),
            stack: Stack::new(1024),
            memory: Memory::new(),
            pc: 0,
            stopped: false,
        }
    }

    pub fn stop(&mut self) {
        self.stopped = true;
    }

    /// Returns the next num_bytes from the code buffer as an integer and advances pc by num_bytes.
    pub fn read_code(&mut self, num_bytes: usize) -> usize {
        let bytes_slice = &self.code[self.pc..self.pc + num_bytes];
        self.pc += num_bytes;
        usize::from_be_bytes(bytes_slice.try_into().unwrap())
    }
}
