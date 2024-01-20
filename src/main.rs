use bytes::Bytes;
use hex::{self};
use smol_evm_rs::{
    execution_context::ExecutionContext, instruction::Instruction, opcodes::Opcodes,
};
use std::env::args;

fn main() {
    let max_steps = 1000; // Todo: Should replace with gas
    Opcodes::register_instructions();

    let hex_str = args().nth(1).expect("No bytecode passed");
    let byte_array = hex::decode(&hex_str).expect("Invalid hex string");
    let code = Bytes::from(byte_array);
    let mut context = ExecutionContext::new(code);
    let mut steps = 1;
    while !context.stopped {
        let pc_before = context.pc.clone();
        let instruction = Instruction::decode_opcode(&mut context).unwrap();
        instruction.executor.execute(&mut context);
        steps += 1;
        if steps > max_steps {
            panic!("Error: Out of gas"); // Todo: Handle gracefully
        }
        println!("{:?} @ pc={}", instruction.name, pc_before);
        println!("Stack: {:?}", context.stack.stack);
        println!("Memory: {:?}", context.memory.memory);
        println!("---------");
    }

    println!("{}", format!("Output : 0x{:x}", context.returndata));
}
