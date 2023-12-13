use bytes::Bytes;
use hex::{self};
use smol_evm_rs::{
    execution_context::ExecutionContext,
    executors::{ADD, MSTORE8, MUL, PUSH1, RETURN, STOP},
    instruction::Instruction,
};
use std::env::args;

fn main() {
    Instruction::register_instruction(0x00, "STOP".to_string(), Box::new(STOP));
    Instruction::register_instruction(0x60, "PUSH1".to_string(), Box::new(PUSH1));
    Instruction::register_instruction(0x01, "ADD".to_string(), Box::new(ADD));
    Instruction::register_instruction(0x02, "MUL".to_string(), Box::new(MUL));
    Instruction::register_instruction(0x53, "MSTORE8".to_string(), Box::new(MSTORE8));
    Instruction::register_instruction(0xf3, "RETURN".to_string(), Box::new(RETURN));

    let hex_str = args().nth(1).expect("no bytecode passed");
    let byte_array = hex::decode(&hex_str).expect("Invalid hex string");
    let code = Bytes::from(byte_array);
    let mut context = ExecutionContext::new(code);
    while !context.stopped {
        let pc_before = context.pc.clone();
        let instruction = Instruction::decode_opcode(&mut context).unwrap();
        instruction.executor.execute(&mut context);

        println!("{:?} @ pc={}", instruction.name, pc_before);
        println!("Stack: {:?}", context.stack.stack);
        println!("Memory: {:?}", context.memory.memory);
        println!("---------");
    }
    println!("{}", format!("Output : 0x{:x}", context.returndata));
}
