use crate::{execution_context::ExecutionContext, instruction::Instruction};
use std::fmt::Debug;

#[derive(Debug)]
pub enum Opcodes {
    PUSH1,
    STOP,
    ADD,
    MUL,
    MSTORE8,
    RETURN,
}

impl Opcodes {
    pub fn register_instructions() {
        Instruction::register_instruction(0x00, "STOP".to_string(), Box::new(Opcodes::STOP));
        Instruction::register_instruction(0x60, "PUSH1".to_string(), Box::new(Opcodes::PUSH1));
        Instruction::register_instruction(0x01, "ADD".to_string(), Box::new(Opcodes::ADD));
        Instruction::register_instruction(0x02, "MUL".to_string(), Box::new(Opcodes::MUL));
        Instruction::register_instruction(0x53, "MSTORE8".to_string(), Box::new(Opcodes::MSTORE8));
        Instruction::register_instruction(0xf3, "RETURN".to_string(), Box::new(Opcodes::RETURN));
    }
}
pub trait OpcodeExecutor: Send + Sync + Debug {
    fn execute(&self, context: &mut ExecutionContext);
}

impl OpcodeExecutor for Opcodes {
    fn execute(&self, context: &mut ExecutionContext) {
        match self {
            Opcodes::STOP => {
                context.stop();
            }
            Opcodes::PUSH1 => {
                let value = context.read_code(1);
                let _ = context.stack.push(value);
            }
            Opcodes::ADD => {
                let value1 = context.stack.pop().unwrap();
                let value2 = context.stack.pop().unwrap();
                context.stack.push(value1.wrapping_add(value2)).unwrap();
            }
            Opcodes::MUL => {
                let value1 = context.stack.pop().unwrap();
                let value2 = context.stack.pop().unwrap();
                context.stack.push(value1.wrapping_mul(value2)).unwrap();
            }
            Opcodes::MSTORE8 => {
                let offset = context.stack.pop().unwrap();
                let value = context.stack.pop().unwrap() % 256;
                context.memory.store(offset, value).unwrap();
            }
            Opcodes::RETURN => {
                let offset = context.stack.pop().unwrap();
                let length = context.stack.pop().unwrap();
                context.set_returndata(offset, length);
            }
        }
    }
}
