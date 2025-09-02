# Pi Calculator

This is a multi-threaded Rust program that calculates the first n digits of Pi using the Bailey–Borwein–Plouffe (BBP) formula. It uses arbitrary-precision arithmetic to ensure the accuracy of the calculated digits.

## Features

*   Calculates the first n digits of Pi.
*   Multi-threaded to speed up the calculation.
*   Configurable number of threads.
*   Uses the BBP algorithm.
*   High-precision calculation using the `rug` crate.

## Building

To build the program, you need to have Rust and Cargo installed. You can install them from [https://rustup.rs/](https://rustup.rs/).

Once you have Rust and Cargo installed, you can build the program with the following command:

```bash
cargo build --release
```

## Usage

To run the program, you can use the following command:

```bash
./target/release/pi <N> [OPTIONS]
```

### Arguments

*   `<N>`: The number of digits of Pi to calculate.

### Options

*   `-t`, `--threads <THREADS>`: The number of threads to use. Defaults to 4.
*   `-h`, `--help`: Print help information.
*   `-V`, `--version`: Print version information.

### Example

To calculate the first 1000 digits of Pi using 8 threads, you can run the following command:

```bash
./target/release/pi 1000 -t 8
```
