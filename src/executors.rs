use std::fmt::Debug;

use crate::execution_context::ExecutionContext;

#[derive(Debug)]
pub struct PUSH1;

#[derive(Debug)]
pub struct STOP;

#[derive(Debug)]
pub struct ADD;

#[derive(Debug)]
pub struct MUL;

pub trait InstructionExecutor: Send + Sync + Debug {
    fn execute(&self, context: &mut ExecutionContext);
}

impl InstructionExecutor for STOP {
    fn execute(&self, context: &mut ExecutionContext) {
        context.stop();
    }
}

impl InstructionExecutor for PUSH1 {
    fn execute(&self, context: &mut ExecutionContext) {
        let value = context.read_code(1);
        let _ = context.stack.push(value);
    }
}

impl InstructionExecutor for ADD {
    fn execute(&self, context: &mut ExecutionContext) {
        let value1 = context.stack.pop().unwrap();
        let value2 = context.stack.pop().unwrap();
        context.stack.push(value1.wrapping_add(value2)).unwrap();
    }
}

impl InstructionExecutor for MUL {
    fn execute(&self, context: &mut ExecutionContext) {
        let value1 = context.stack.pop().unwrap();
        let value2 = context.stack.pop().unwrap();
        context.stack.push(value1.wrapping_mul(value2)).unwrap();
    }
}