use crate::{
    calldata::Calldata,
    instruction::{Instruction, INSTRUCTIONS_BY_OPCODE},
    memory::Memory,
    stack::Stack,
};
use bytes::Bytes;

#[derive(Debug)]
pub struct ExecutionContext {
    pub code: Bytes,
    pub stack: Stack,
    pub memory: Memory,
    pub pc: usize,
    pub stopped: bool,
    pub returndata: Bytes,
    pub jumpdests: Vec<usize>,
    pub calldata: Calldata,
}

impl ExecutionContext {
    pub fn new(code: Bytes) -> Self {
        Self {
            code: code.clone(),
            stack: Stack::new(1024),
            memory: Memory::new(),
            pc: 0,
            stopped: false,
            returndata: Bytes::new(),
            jumpdests: Self::valid_jump_destinations(code.clone()),
            calldata: Calldata::new(code),
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

    pub fn set_pc(&mut self, pc: usize) {
        self.pc = pc;
    }

    pub fn valid_jump_destinations(code: Bytes) -> Vec<usize> {
        let mut jumpdests: Vec<usize> = Vec::new();

        let mut i = 0;
        while i < code.len() {
            let current_op = code[i] as usize;
            if current_op == 0x5B {
                jumpdests.push(i);
            } else if 0x60 <= current_op && current_op <= 0x7F {
                i += current_op - 0x60 + 1
            }
            i += 1;
        }
        jumpdests
    }
}
