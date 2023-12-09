use bytes::Bytes;
use hex;
use smol_evm_rs::{
    execution_context::ExecutionContext,
    executors::{ADD, MUL, PUSH1, STOP},
    instruction::Instruction,
};
use std::env::args;

fn main() {
    Instruction::register_instruction(0x00, "STOP".to_string(), Box::new(STOP));
    Instruction::register_instruction(0x60, "PUSH1".to_string(), Box::new(PUSH1));
    Instruction::register_instruction(0x01, "ADD".to_string(), Box::new(ADD));
    Instruction::register_instruction(0x02, "MUL".to_string(), Box::new(MUL));

    let hex_str = args().nth(1).expect("no bytecode passed");
    let byte_array = hex::decode(&hex_str).expect("Invalid hex string");
    let code = Bytes::from(byte_array);
    let mut context = ExecutionContext::new(code);
    while !context.stopped {
        let pc_before = context.pc.clone();
        let instruction = Instruction::decode_opcode(&mut context).unwrap();
        instruction.executor.execute(&mut context);

        println!("{:?} @ pc={}", instruction, pc_before);
        println!("{:?}", context);
    }
}
