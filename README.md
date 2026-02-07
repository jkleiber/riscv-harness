
# RISC-V Harness

A test harness for RISC-V developers who use RARS.

## Getting Started

### Prerequisites
- RARS 1.6 jar file, named "rars1_6.jar" (a future release will make this configurable)

### Running your first test
1. Download the release binary for your platform. Currently the project supports:
- Windows
- MacOS
- Linux

2. Place the binary alongside your RARS jar file wherever you want to run the test harness

3. From the command line / terminal, run:

**MacOS and Linux**
```
riscv-harness path/to/asm/script.asm path/to/test/vectors
```

**Windows**
```
riscv-harness.exe path\to\asm\script.asm path\to\test\vectors
```

### Test Vectors
Test vectors should take the form:
```
inputs
inputs
---
outputs
outputs
```

The `---` is very important, as the program checks for this exact match in order to determine when to start ingesting the output side of the test vector.

The test harness works by looking for all the test vectors in a given directory, so placing all the tests for a script in the same folder is recommended. The test harness does not currently explore subfolders for additional tests.

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


**MacOS and Linux**
```
target/debug/riscv-harness path/to/asm/script.asm path/to/test/vectors
```

**Windows**
```
target\debug\riscv-harness.exe path\to\asm\script.asm path\to\test\vectors
```
