use clap::Parser;
use rug::{Float, ops::Pow};
use std::thread;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Number of digits of Pi to calculate
    n: u32,

    /// Number of threads to use
    #[arg(short, long, default_value_t = 4)]
    threads: usize,
}

fn bbp_term(k: u32, prec: u32) -> Float {
    let mut term = Float::with_val(prec, 4);
    term /= Float::with_val(prec, 8 * k + 1);

    let mut term2 = Float::with_val(prec, 2);
    term2 /= Float::with_val(prec, 8 * k + 4);
    term -= term2;

    let mut term3 = Float::with_val(prec, 1);
    term3 /= Float::with_val(prec, 8 * k + 5);
    term -= term3;

    let mut term4 = Float::with_val(prec, 1);
    term4 /= Float::with_val(prec, 8 * k + 6);
    term -= term4;

    let sixteen = Float::with_val(prec, 16);
    term /= sixteen.pow(k);

    term
}

fn main() {
    let args = Args::parse();
    let n = args.n;
    let num_threads = args.threads;

    // Precision for rug::Float. We need a bit more than n decimal digits.
    // log2(10) is approx 3.32. So, we need n * 3.32 bits.
    let prec = (n as f64 * 3.33).ceil() as u32;

    let terms_per_thread = (n + num_threads as u32 - 1) / num_threads as u32;

    let mut handles = vec![];

    for i in 0..num_threads {
        let start = i as u32 * terms_per_thread;
        let end = ((i + 1) as u32 * terms_per_thread).min(n);
        let handle = thread::spawn(move || {
            let mut partial_sum = Float::with_val(prec, 0);
            for k in start..end {
                partial_sum += bbp_term(k, prec);
            }
            partial_sum
        });
        handles.push(handle);
    }

    let mut pi = Float::with_val(prec, 0);
    for handle in handles {
        pi += handle.join().unwrap();
    }

    println!("Pi: {}", pi.to_string_radix(10, Some(n as usize)));
}