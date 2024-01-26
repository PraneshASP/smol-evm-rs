# smol-evm-rs • ![License](https://img.shields.io/badge/license-MIT-brown.svg) [![CI](https://github.com/PraneshASP/smol-evm-rs/actions/workflows/tests.yml/badge.svg)](https://github.com/PraneshASP/smol-evm-rs/actions/workflows/tests.yml) ![Built Using Rust](https://img.shields.io/badge/Built%20Using-Rust-orange.svg) 
---

> [!WARNING]  
> It is important to note that the code is not optimized and may contain bugs. It is primarily intended for educational purposes. So don't use any code from this repo for production.

`smol-evm-rs` is a toy implementation of the Ethereum Virtual Machine, inspired by the [smol-evm](https://github.com/karmacoma-eth/smol-evm) project, originally implemented in Python by karmacoma. The primary goal of the project is to increase my Rust proficiency. 
- [X] [**Part1**: The execution context ](https://github.com/PraneshASP/smol-evm-rs/tree/part-1)
- [X] [**Part2**: Branching instructions](https://github.com/PraneshASP/smol-evm-rs/tree/part-2) 
- [X] [**Part 3**: Calldata & comparison instructions](https://github.com/PraneshASP/smol-evm-rs/tree/part-3) 


## Getting started:

### Prerequisites

- [Rust](https://doc.rust-lang.org/book/ch01-01-installation.html)  
 
### Build Instructions

1. Clone the repository:
   ```bash
   git clone https://github.com/PraneshASP/smol-evm-rs.git
   cd smol-evm-rs
   ```
2. Build the project:
   ```bash
   cargo build
   ```
3. Run the project:
   ```bash
   cargo run <BYTECODE>
   ```
 
 ### Example:
 Run: `cargo run 60048060005b8160125760005360016000f35b8201906001900390600556`

 > The above bytecode calculates 4.pow(2) (four-squared)
 
 It should return`0x10` as output along with the memory, stack and program counter values.

```bash

Opcode: 96
"PUSH1" @ pc=0
Stack: [4]
Memory: []
---------
Opcode: 128
"DUP1" @ pc=2
Stack: [4, 4]
Memory: []
---------
Opcode: 96
"PUSH1" @ pc=3
Stack: [4, 4, 0]
Memory: []
---------
Opcode: 3
"SUB" @ pc=25
Stack: [4, 8, 2]
Memory: []
---------
Opcode: 144
"SWAP1" @ pc=26
Stack: [4, 2, 8]
Memory: []
---------

...

...

...

---------
Opcode: 87
"JUMPI" @ pc=9
Stack: [4, 0, 16]
Memory: []
---------
Opcode: 96
"PUSH1" @ pc=10
Stack: [4, 0, 16, 0]
Memory: []
---------
"MSTORE8" @ pc=12
Stack: [4, 0]
Memory: [16, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]
---------
"PUSH1" @ pc=13
Stack: [4, 0, 1]
Memory: [16, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]
---------
"PUSH1" @ pc=15
Stack: [4, 0, 1, 0]
Memory: [16, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]
---------
"RETURN" @ pc=17
Stack: [4, 0]
Memory: [16, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]
---------
Output : 0x1000000000000000

```
 
> [!NOTE]  
> Supported Opcodes: `ADD`,`SUB`,`MUL`,`PUSH1`, `MSTORE8`, `RETURN`, `STOP`,`JUMP`, `JUMPI`,`JUMPDEST`,`GT` ,`LT`, `ISZERO`,`SHR`,SHL`,`CALLDATALOAD`, `CALLDATASIZE`, `CALLVALUE`,`SWAP[1-16]`, `PUSH[0-32]`and `DUP[1-16]`

### Improvement ideas:
- Make word size 32 instead of 16.
- Implement remaining opcodes like MSTORE, MLOAD, CALLDATACOPY
- Implement `gas` calculation. 
- Add more tests.
  
## Acknowledgments

- [karmacoma-eth](https://github.com/karmacoma-eth) for the original Python implementation of `smol-evm`.