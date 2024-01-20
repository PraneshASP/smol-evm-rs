use crate::{execution_context::ExecutionContext, instruction::Instruction};
use std::fmt::Debug;

#[derive(Debug)]
pub enum Opcodes {
    STOP,
    ADD,
    MUL,
    SUB,
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

    // DUP Opcodes
    DUP1,
    DUP2,
    DUP3,
    DUP4,
    DUP5,
    DUP6,
    DUP7,
    DUP8,
    DUP9,
    DUP10,
    DUP11,
    DUP12,
    DUP13,
    DUP14,
    DUP15,
    DUP16,

    // SWAP Opcodes
    SWAP1,
    SWAP2,
    SWAP3,
    SWAP4,
    SWAP5,
    SWAP6,
    SWAP7,
    SWAP8,
    SWAP9,
    SWAP10,
    SWAP11,
    SWAP12,
    SWAP13,
    SWAP14,
    SWAP15,
    SWAP16,

    // JUMP Opcodes
    JUMP,
    JUMPI,
    JUMPDEST,
}

#[derive(Debug)]
pub enum Errors {
    InvalidJumpDestination(usize),
}

impl Opcodes {
    pub fn register_instructions() {
        Instruction::register_instruction(0x00, "STOP".to_string(), Box::new(Opcodes::STOP));
        Instruction::register_instruction(0x01, "ADD".to_string(), Box::new(Opcodes::ADD));
        Instruction::register_instruction(0x02, "MUL".to_string(), Box::new(Opcodes::MUL));
        Instruction::register_instruction(0x03, "SUB".to_string(), Box::new(Opcodes::SUB));

        Instruction::register_instruction(0x53, "MSTORE8".to_string(), Box::new(Opcodes::MSTORE8));
        Instruction::register_instruction(0xf3, "RETURN".to_string(), Box::new(Opcodes::RETURN));
        Instruction::register_instruction(0x58, "PC".to_string(), Box::new(Opcodes::PC));
        Instruction::register_instruction(0x59, "MSIZE".to_string(), Box::new(Opcodes::MSIZE));

        // PUSH Instructions

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

        // DUP Instructions

        Instruction::register_instruction(0x80, "DUP1".to_string(), Box::new(Opcodes::DUP1));
        Instruction::register_instruction(0x81, "DUP2".to_string(), Box::new(Opcodes::DUP2));
        Instruction::register_instruction(0x82, "DUP3".to_string(), Box::new(Opcodes::DUP3));
        Instruction::register_instruction(0x83, "DUP4".to_string(), Box::new(Opcodes::DUP4));
        Instruction::register_instruction(0x84, "DUP5".to_string(), Box::new(Opcodes::DUP5));
        Instruction::register_instruction(0x85, "DUP6".to_string(), Box::new(Opcodes::DUP6));
        Instruction::register_instruction(0x86, "DUP7".to_string(), Box::new(Opcodes::DUP7));
        Instruction::register_instruction(0x87, "DUP8".to_string(), Box::new(Opcodes::DUP8));
        Instruction::register_instruction(0x88, "DUP9".to_string(), Box::new(Opcodes::DUP9));
        Instruction::register_instruction(0x89, "DUP10".to_string(), Box::new(Opcodes::DUP10));
        Instruction::register_instruction(0x8A, "DUP11".to_string(), Box::new(Opcodes::DUP11));
        Instruction::register_instruction(0x8B, "DUP12".to_string(), Box::new(Opcodes::DUP12));
        Instruction::register_instruction(0x8C, "DUP13".to_string(), Box::new(Opcodes::DUP13));
        Instruction::register_instruction(0x8D, "DUP14".to_string(), Box::new(Opcodes::DUP14));
        Instruction::register_instruction(0x8E, "DUP15".to_string(), Box::new(Opcodes::DUP15));
        Instruction::register_instruction(0x8F, "DUP16".to_string(), Box::new(Opcodes::DUP16));

        // SWAP Instructions
        Instruction::register_instruction(0x90, "SWAP1".to_string(), Box::new(Opcodes::SWAP1));
        Instruction::register_instruction(0x91, "SWAP2".to_string(), Box::new(Opcodes::SWAP2));
        Instruction::register_instruction(0x92, "SWAP3".to_string(), Box::new(Opcodes::SWAP3));
        Instruction::register_instruction(0x93, "SWAP4".to_string(), Box::new(Opcodes::SWAP4));
        Instruction::register_instruction(0x94, "SWAP5".to_string(), Box::new(Opcodes::SWAP5));
        Instruction::register_instruction(0x95, "SWAP6".to_string(), Box::new(Opcodes::SWAP6));
        Instruction::register_instruction(0x96, "SWAP7".to_string(), Box::new(Opcodes::SWAP7));
        Instruction::register_instruction(0x97, "SWAP8".to_string(), Box::new(Opcodes::SWAP8));
        Instruction::register_instruction(0x98, "SWAP9".to_string(), Box::new(Opcodes::SWAP9));
        Instruction::register_instruction(0x99, "SWAP10".to_string(), Box::new(Opcodes::SWAP10));
        Instruction::register_instruction(0x9A, "SWAP11".to_string(), Box::new(Opcodes::SWAP11));
        Instruction::register_instruction(0x9B, "SWAP12".to_string(), Box::new(Opcodes::SWAP12));
        Instruction::register_instruction(0x9C, "SWAP13".to_string(), Box::new(Opcodes::SWAP13));
        Instruction::register_instruction(0x9D, "SWAP14".to_string(), Box::new(Opcodes::SWAP14));
        Instruction::register_instruction(0x9E, "SWAP15".to_string(), Box::new(Opcodes::SWAP15));
        Instruction::register_instruction(0x9F, "SWAP16".to_string(), Box::new(Opcodes::SWAP16));

        // JUMP Instructions
        Instruction::register_instruction(0x56, "JUMP".to_string(), Box::new(Opcodes::JUMP));
        Instruction::register_instruction(0x57, "JUMPI".to_string(), Box::new(Opcodes::JUMPI));
        Instruction::register_instruction(
            0x5B,
            "JUMPDEST".to_string(),
            Box::new(Opcodes::JUMPDEST),
        );
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
            Opcodes::SUB => {
                let a = context.stack.pop().unwrap();
                let b = context.stack.pop().unwrap();
                context.stack.push(a.checked_sub(b).unwrap()).unwrap();
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

            // DUP Opcodes
            Opcodes::DUP1 => {
                let value = context.stack.peek(0).unwrap();
                context.stack.push(value).unwrap();
            }
            Opcodes::DUP2 => {
                let value = context.stack.peek(1).unwrap();
                context.stack.push(value).unwrap();
            }
            Opcodes::DUP3 => {
                let value = context.stack.peek(2).unwrap();
                context.stack.push(value).unwrap();
            }
            Opcodes::DUP4 => {
                let value = context.stack.peek(3).unwrap();
                context.stack.push(value).unwrap();
            }
            Opcodes::DUP5 => {
                let value = context.stack.peek(4).unwrap();
                context.stack.push(value).unwrap();
            }
            Opcodes::DUP6 => {
                let value = context.stack.peek(5).unwrap();
                context.stack.push(value).unwrap();
            }
            Opcodes::DUP7 => {
                let value = context.stack.peek(6).unwrap();
                context.stack.push(value).unwrap();
            }
            Opcodes::DUP8 => {
                let value = context.stack.peek(7).unwrap();
                context.stack.push(value).unwrap();
            }
            Opcodes::DUP9 => {
                let value = context.stack.peek(8).unwrap();
                context.stack.push(value).unwrap();
            }
            Opcodes::DUP10 => {
                let value = context.stack.peek(9).unwrap();
                context.stack.push(value).unwrap();
            }
            Opcodes::DUP11 => {
                let value = context.stack.peek(10).unwrap();
                context.stack.push(value).unwrap();
            }
            Opcodes::DUP12 => {
                let value = context.stack.peek(11).unwrap();
                context.stack.push(value).unwrap();
            }
            Opcodes::DUP13 => {
                let value = context.stack.peek(12).unwrap();
                context.stack.push(value).unwrap();
            }
            Opcodes::DUP14 => {
                let value = context.stack.peek(13).unwrap();
                context.stack.push(value).unwrap();
            }
            Opcodes::DUP15 => {
                let value = context.stack.peek(14).unwrap();
                context.stack.push(value).unwrap();
            }
            Opcodes::DUP16 => {
                let value = context.stack.peek(15).unwrap();
                context.stack.push(value).unwrap();
            }

            // SWAP Opcodes
            Opcodes::SWAP1 => {
                context.stack.swap(1).unwrap();
            }
            Opcodes::SWAP2 => {
                context.stack.swap(2).unwrap();
            }
            Opcodes::SWAP3 => {
                context.stack.swap(3).unwrap();
            }
            Opcodes::SWAP4 => {
                context.stack.swap(4).unwrap();
            }
            Opcodes::SWAP5 => {
                context.stack.swap(5).unwrap();
            }
            Opcodes::SWAP6 => {
                context.stack.swap(6).unwrap();
            }
            Opcodes::SWAP7 => {
                context.stack.swap(7).unwrap();
            }
            Opcodes::SWAP8 => {
                context.stack.swap(8).unwrap();
            }
            Opcodes::SWAP9 => {
                context.stack.swap(9).unwrap();
            }
            Opcodes::SWAP10 => {
                context.stack.swap(10).unwrap();
            }
            Opcodes::SWAP11 => {
                context.stack.swap(11).unwrap();
            }
            Opcodes::SWAP12 => {
                context.stack.swap(12).unwrap();
            }
            Opcodes::SWAP13 => {
                context.stack.swap(13).unwrap();
            }
            Opcodes::SWAP14 => {
                context.stack.swap(14).unwrap();
            }
            Opcodes::SWAP15 => {
                context.stack.swap(15).unwrap();
            }
            Opcodes::SWAP16 => {
                context.stack.swap(16).unwrap();
            }

            // JUMP Instructions
            Opcodes::JUMP => {
                let target_pc = context.stack.pop().unwrap();
                if context.jumpdests.contains(&target_pc) {
                    context.set_pc(target_pc);
                } else {
                    panic!("Invalid JumpDestination"); //TODO: Handle gracefully
                }
            }
            Opcodes::JUMPI => {
                let target_pc = context.stack.pop().unwrap();
                let condition = context.stack.pop().unwrap();
                if condition != 0 {
                    if context.jumpdests.contains(&target_pc) {
                        context.set_pc(target_pc);
                    } else {
                        panic!("Invalid JumpDestination"); //TODO: Handle gracefully
                    }
                }
            }
            Opcodes::JUMPDEST => {}
        }
    }
}
