
# RISC-V Harness

A test harness for RISC-V developers who use RARS.

## Getting Started

More info to come. Currently best suited for developers.

## Developing

### Setup + Building
- Rust installation
- RARS jar file

To get started you should just need
```
cargo install
```
to install all the dependencies, and then 
```
cargo build
```
to build the program in debug mode (for releases, `cargo build -r`).

### Running the program

**Windows**
```
target\[TARGET]\riscv-harness.exe path\to\asm\script.asm path\to\test\vectors
```