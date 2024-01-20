use crate::{execution_context::ExecutionContext, instruction::Instruction};
use std::fmt::Debug;

#[derive(Debug)]
pub enum Opcodes {
    STOP,
    ADD,
    MUL,
    MSTORE8,
    RETURN,
    PC,
    MSIZE,
    PUSH1,
    PUSH2,
    PUSH3,
    PUSH4,
    PUSH5,
    PUSH6,
    PUSH7,
    PUSH8,
    PUSH9,
    PUSH10,
    PUSH11,
    PUSH12,
    PUSH13,
    PUSH14,
    PUSH15,
    PUSH16,
    PUSH17,
    PUSH18,
    PUSH19,
    PUSH20,
    PUSH21,
    PUSH22,
    PUSH23,
    PUSH24,
    PUSH25,
    PUSH26,
    PUSH27,
    PUSH28,
    PUSH29,
    PUSH30,
    PUSH31,
    PUSH32,
}

impl Opcodes {
    pub fn register_instructions() {
        Instruction::register_instruction(0x00, "STOP".to_string(), Box::new(Opcodes::STOP));
        Instruction::register_instruction(0x01, "ADD".to_string(), Box::new(Opcodes::ADD));
        Instruction::register_instruction(0x02, "MUL".to_string(), Box::new(Opcodes::MUL));
        Instruction::register_instruction(0x53, "MSTORE8".to_string(), Box::new(Opcodes::MSTORE8));
        Instruction::register_instruction(0xf3, "RETURN".to_string(), Box::new(Opcodes::RETURN));
        Instruction::register_instruction(0x58, "PC".to_string(), Box::new(Opcodes::PC));
        Instruction::register_instruction(0x59, "MSIZE".to_string(), Box::new(Opcodes::MSIZE));
        Instruction::register_instruction(0x60, "PUSH1".to_string(), Box::new(Opcodes::PUSH1));
        Instruction::register_instruction(0x61, "PUSH2".to_string(), Box::new(Opcodes::PUSH2));
        Instruction::register_instruction(0x62, "PUSH3".to_string(), Box::new(Opcodes::PUSH3));
        Instruction::register_instruction(0x63, "PUSH4".to_string(), Box::new(Opcodes::PUSH4));
        Instruction::register_instruction(0x64, "PUSH5".to_string(), Box::new(Opcodes::PUSH5));
        Instruction::register_instruction(0x65, "PUSH6".to_string(), Box::new(Opcodes::PUSH6));
        Instruction::register_instruction(0x66, "PUSH7".to_string(), Box::new(Opcodes::PUSH7));
        Instruction::register_instruction(0x67, "PUSH8".to_string(), Box::new(Opcodes::PUSH8));
        Instruction::register_instruction(0x68, "PUSH9".to_string(), Box::new(Opcodes::PUSH9));
        Instruction::register_instruction(0x69, "PUSH10".to_string(), Box::new(Opcodes::PUSH10));
        Instruction::register_instruction(0x6A, "PUSH11".to_string(), Box::new(Opcodes::PUSH11));
        Instruction::register_instruction(0x6B, "PUSH12".to_string(), Box::new(Opcodes::PUSH12));
        Instruction::register_instruction(0x6C, "PUSH13".to_string(), Box::new(Opcodes::PUSH13));
        Instruction::register_instruction(0x6D, "PUSH14".to_string(), Box::new(Opcodes::PUSH14));
        Instruction::register_instruction(0x6E, "PUSH15".to_string(), Box::new(Opcodes::PUSH15));
        Instruction::register_instruction(0x6F, "PUSH16".to_string(), Box::new(Opcodes::PUSH16));
        Instruction::register_instruction(0x70, "PUSH17".to_string(), Box::new(Opcodes::PUSH17));
        Instruction::register_instruction(0x71, "PUSH18".to_string(), Box::new(Opcodes::PUSH18));
        Instruction::register_instruction(0x72, "PUSH19".to_string(), Box::new(Opcodes::PUSH19));
        Instruction::register_instruction(0x73, "PUSH20".to_string(), Box::new(Opcodes::PUSH20));
        Instruction::register_instruction(0x74, "PUSH21".to_string(), Box::new(Opcodes::PUSH21));
        Instruction::register_instruction(0x75, "PUSH22".to_string(), Box::new(Opcodes::PUSH22));
        Instruction::register_instruction(0x76, "PUSH23".to_string(), Box::new(Opcodes::PUSH23));
        Instruction::register_instruction(0x77, "PUSH24".to_string(), Box::new(Opcodes::PUSH24));
        Instruction::register_instruction(0x78, "PUSH25".to_string(), Box::new(Opcodes::PUSH25));
        Instruction::register_instruction(0x79, "PUSH26".to_string(), Box::new(Opcodes::PUSH26));
        Instruction::register_instruction(0x7A, "PUSH27".to_string(), Box::new(Opcodes::PUSH27));
        Instruction::register_instruction(0x7B, "PUSH28".to_string(), Box::new(Opcodes::PUSH28));
        Instruction::register_instruction(0x7C, "PUSH29".to_string(), Box::new(Opcodes::PUSH29));
        Instruction::register_instruction(0x7D, "PUSH30".to_string(), Box::new(Opcodes::PUSH30));
        Instruction::register_instruction(0x7E, "PUSH31".to_string(), Box::new(Opcodes::PUSH31));
        Instruction::register_instruction(0x7F, "PUSH32".to_string(), Box::new(Opcodes::PUSH32));
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
            Opcodes::PC => context.stack.push(context.pc).unwrap(),
            Opcodes::MSIZE => context
                .stack
                .push(32 * (context.memory.active_words()))
                .unwrap(),
            Opcodes::PUSH1 => {
                let value = context.read_code(1);
                context.stack.push(value).unwrap();
            }
            Opcodes::PUSH2 => {
                let value = context.read_code(2);
                context.stack.push(value).unwrap();
            }
            Opcodes::PUSH3 => {
                let value = context.read_code(3);
                context.stack.push(value).unwrap();
            }
            Opcodes::PUSH4 => {
                let value = context.read_code(4);
                context.stack.push(value).unwrap();
            }
            Opcodes::PUSH5 => {
                let value = context.read_code(5);
                context.stack.push(value).unwrap();
            }
            Opcodes::PUSH6 => {
                let value = context.read_code(6);
                context.stack.push(value).unwrap();
            }
            Opcodes::PUSH7 => {
                let value = context.read_code(7);
                context.stack.push(value).unwrap();
            }
            Opcodes::PUSH8 => {
                let value = context.read_code(8);
                context.stack.push(value).unwrap();
            }
            Opcodes::PUSH9 => {
                let value = context.read_code(9);
                context.stack.push(value).unwrap();
            }
            Opcodes::PUSH10 => {
                let value = context.read_code(10);
                context.stack.push(value).unwrap();
            }
            Opcodes::PUSH11 => {
                let value = context.read_code(11);
                context.stack.push(value).unwrap();
            }
            Opcodes::PUSH12 => {
                let value = context.read_code(12);
                context.stack.push(value).unwrap();
            }
            Opcodes::PUSH13 => {
                let value = context.read_code(13);
                context.stack.push(value).unwrap();
            }
            Opcodes::PUSH14 => {
                let value = context.read_code(14);
                context.stack.push(value).unwrap();
            }
            Opcodes::PUSH15 => {
                let value = context.read_code(15);
                context.stack.push(value).unwrap();
            }
            Opcodes::PUSH16 => {
                let value = context.read_code(16);
                context.stack.push(value).unwrap();
            }
            Opcodes::PUSH17 => {
                let value = context.read_code(17);
                context.stack.push(value).unwrap();
            }
            Opcodes::PUSH18 => {
                let value = context.read_code(18);
                context.stack.push(value).unwrap();
            }
            Opcodes::PUSH19 => {
                let value = context.read_code(19);
                context.stack.push(value).unwrap();
            }
            Opcodes::PUSH20 => {
                let value = context.read_code(20);
                context.stack.push(value).unwrap();
            }
            Opcodes::PUSH21 => {
                let value = context.read_code(21);
                context.stack.push(value).unwrap();
            }
            Opcodes::PUSH22 => {
                let value = context.read_code(22);
                context.stack.push(value).unwrap();
            }
            Opcodes::PUSH23 => {
                let value = context.read_code(23);
                context.stack.push(value).unwrap();
            }
            Opcodes::PUSH24 => {
                let value = context.read_code(24);
                context.stack.push(value).unwrap();
            }
            Opcodes::PUSH25 => {
                let value = context.read_code(25);
                context.stack.push(value).unwrap();
            }
            Opcodes::PUSH26 => {
                let value = context.read_code(26);
                context.stack.push(value).unwrap();
            }
            Opcodes::PUSH27 => {
                let value = context.read_code(27);
                context.stack.push(value).unwrap();
            }
            Opcodes::PUSH28 => {
                let value = context.read_code(28);
                context.stack.push(value).unwrap();
            }
            Opcodes::PUSH29 => {
                let value = context.read_code(29);
                context.stack.push(value).unwrap();
            }
            Opcodes::PUSH30 => {
                let value = context.read_code(30);
                context.stack.push(value).unwrap();
            }
            Opcodes::PUSH31 => {
                let value = context.read_code(31);
                context.stack.push(value).unwrap();
            }
            Opcodes::PUSH32 => {
                let value = context.read_code(32);
                context.stack.push(value).unwrap();
            }
        }
    }
}
