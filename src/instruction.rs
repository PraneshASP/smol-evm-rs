use bytes::Bytes;
use lazy_static::lazy_static;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

use crate::execution_context::ExecutionContext;
use crate::opcodes::OpcodeExecutor;

#[derive(Debug)]
pub struct Instruction {
    pub opcode: usize,
    pub name: String,
    pub executor: Box<dyn OpcodeExecutor>,
}

#[derive(Debug)]
pub enum InstructionError {
    InvalidCodeOffset { code: Bytes, pc: usize },
}

// Note to self: Rust requires the use of a mutex or similar mechanism for mutable statics due to thread safety concerns.
// Mutex and lazy_static approach is one way to handle this
lazy_static! {
    static ref INSTRUCTIONS: Mutex<Vec<Arc<Instruction>>> = Mutex::new(vec![]);
    static ref INSTRUCTIONS_BY_OPCODE: Mutex<HashMap<usize, Arc<Instruction>>> =
        Mutex::new(HashMap::new());
}
impl Instruction {
    pub fn register_instruction(opcode: usize, name: String, executor: Box<dyn OpcodeExecutor>) {
        let instruction = Arc::new(Instruction {
            opcode,
            name: name.clone(),
            executor,
        });

        {
            let mut instructions = INSTRUCTIONS.lock().unwrap();
            instructions.push(instruction.clone());

            let mut instructions_by_opcode = INSTRUCTIONS_BY_OPCODE.lock().unwrap();
            instructions_by_opcode.insert(opcode, instruction);
        }
    }

    pub fn decode_opcode(
        context: &mut ExecutionContext,
    ) -> Result<Arc<Instruction>, InstructionError> {
        let opcode = context.read_code(1);

        let instructions_by_opcode = INSTRUCTIONS_BY_OPCODE.lock().unwrap();

        if context.pc > context.code.len() {
            Ok(instructions_by_opcode.get(&0x00).cloned().unwrap()) // STOP if pc goes over code length
        } else {
            Ok(instructions_by_opcode.get(&opcode).cloned().unwrap())
        }
    }
}
