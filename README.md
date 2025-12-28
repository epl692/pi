# Pi Calculator

This is a multi-threaded Rust program that calculates the first n digits of Pi using the Chudnovsky algorithm with binary splitting. It uses arbitrary-precision arithmetic (rug) and parallelism (rayon).

## Improvements in this branch

* Parallelized Chudnovsky binary-splitting with rayon for better thread control and load balancing.
* Safer argument validation and error handling (avoids unwraps on runtime errors).
* Optional output-to-file support.
* Added CI workflow to run formatting, clippy, tests and build on push/PR.
* Release profile tuned for better optimized builds (LTO, opt-level=3).

## Building

Requires Rust and Cargo. Build with:

```bash
cargo build --release
```

## Usage

```bash
./target/release/pi <N> [OPTIONS]
```

Arguments

* `<N>`: Number of digits after the decimal point to calculate.

Options

* `-t`, `--threads <THREADS>`: Number of threads to use (default 4). Note: this implementation performs parallel binary splitting and the threads option is kept for compatibility but may be ignored.
* `-o`, `--output <FILE>`: Write output to FILE instead of stdout.
* `-h`, `--help`: Print help.

Example

Calculate 1000 digits using 8 threads and write to a file:

```bash
./target/release/pi 1000 -t 8 -o pi1000.txt
```

Notes

This program uses the Chudnovsky algorithm (binary splitting), which is suitable for high-precision Pi computation. For extremely large computations, further optimizations (tuning precision, memory usage, or linking to specialized big-integer libraries) may be required.
