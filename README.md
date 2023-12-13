# smol-evm-rs • ![License](https://img.shields.io/badge/license-MIT-brown.svg) [![CI](https://github.com/PraneshASP/smol-evm-rs/actions/workflows/tests.yml/badge.svg)](https://github.com/PraneshASP/smol-evm-rs/actions/workflows/tests.yml) ![Built Using Rust](https://img.shields.io/badge/Built%20Using-Rust-orange.svg) 
---

`smol-evm-rs` is a Rust port of the [smol-evm](https://github.com/karmacoma-eth/smol-evm) project, originally implemented in Python by karmacoma. This project aims to implement the Ethereum Virtual Machine (EVM) from scratch using Rust. The primary goal of the project is to learn Rust. 

This project is currently a work in progress under the `part-1` branch. 

> [!WARNING]  
> It is important to note that the code is not optimized and is primarily intended for educational purposes.

## Getting started:

### Prerequisites

- [Rust](https://doc.rust-lang.org/book/ch01-01-installation.html)  
 
### Build Instructions

1. Clone the repository:
   ```bash
   git clone https://github.com/PraneshASP/smol-evm-rs.git
   cd smol-evm-rs
   ```
2. Switch to the `part-1` branch:
    ```bash
    git checkout part-1
    ```
3. Build the project:
   ```bash
   cargo build
   ```
4. Run the project:
   ```bash
   cargo run <BYTECODE>
   ```
 
 ### Example:
 Run: `cargo run 600660070260005360016000f3`
 
 It should return`0x2a` as output along with the memory, stack and program counter values.

```bash

"PUSH1" @ pc=0
Stack: [6]
Memory: []
---------
"PUSH1" @ pc=2
Stack: [6, 7]
Memory: []
---------
"MUL" @ pc=4
Stack: [42]
Memory: []
---------
"PUSH1" @ pc=5
Stack: [42, 0]
Memory: []
---------
"MSTORE8" @ pc=7
Stack: []
Memory: [42]
---------
"PUSH1" @ pc=8
Stack: [1]
Memory: [42]
---------
"PUSH1" @ pc=10
Stack: [1, 0]
Memory: [42]
---------
"RETURN" @ pc=12
Stack: []
Memory: [42]
---------
Output : 0x2a00000000000000

```
 
> [!NOTE]  
> Supported Opcodes: `ADD`,`MUL`,`PUSH1`, `MSTORE8`, `RETURN` and `STOP`

## TODO:
- [X] [Implement Part 1 (execution context)](https://github.com/PraneshASP/smol-evm-rs/tree/part-1)
- [ ] Add more tests
- [ ] Implement Part 2 (branching instructions)
- [ ] Implement Part 3 (calldata and function dispatcher)

## Acknowledgments

- [karmacoma-eth](https://github.com/karmacoma-eth) for the original Python implementation of smol-evm.