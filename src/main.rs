use clap::Parser;
use rug::Float;
use rug::ops::Pow;
use rayon::prelude::*;
use std::fs::File;
use std::io::Write;
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Number of digits of Pi to calculate (digits after the decimal point)
    n: u32,

    /// Number of threads to use
    #[arg(short, long, default_value_t = 4)]
    threads: usize,

    /// Optional output file (writes result there if provided)
    #[arg(short, long)]
    output: Option<PathBuf>,
}

fn bbp_term(k: u32, prec: u32) -> Float {
    // Compute one BBP term at precision `prec`.
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
    term /= sixteen.pow(k as i32);

    term
}

/// Calculate Pi to `n` decimal digits using a parallelized BBP summation.
/// Returns a decimal string containing Pi truncated to `n` digits after the decimal point.
pub fn calculate_pi(n: u32, num_threads: usize) -> Result<String, String> {
    if n == 0 {
        return Err("n must be > 0".into());
    }
    if num_threads == 0 {
        return Err("threads must be > 0".into());
    }

    // Bits of precision: log2(10) ~= 3.321928. Add some guard bits.
    let prec = (n as f64 * 3.3219280948873626).ceil() as u32 + 20;

    // BBP converges in base-16; use a modest overestimate for term count.
    let num_terms = (n as usize / 1) + 20; // conservative

    // Use rayon thread pool to control threads for parallel work.
    let pool = rayon::ThreadPoolBuilder::new()
        .num_threads(num_threads)
        .build()
        .map_err(|e| format!("Failed to build thread pool: {}", e))?;

    let pi = pool.install(|| {
        // Parallel iterator over term indices.
        (0..num_terms as u32)
            .into_par_iter()
            .map(|k| bbp_term(k, prec))
            .reduce(|| Float::with_val(prec, 0), |a, b| a + b)
    });

    // Convert to decimal string with a few extra digits for safe truncation.
    let extra = 10usize;
    let pi_string = pi.to_string_radix(10, Some(n as usize + extra));

    // Find dot safely and truncate or pad as needed.
    let dot_pos = pi_string.find('.').unwrap_or(pi_string.len());
    let end_pos = dot_pos + 1 + n as usize;

    let out = if pi_string.len() >= end_pos {
        pi_string[..end_pos].to_string()
    } else {
        // If not enough digits were produced, pad with zeros.
        let mut s = pi_string;
        if !s.contains('.') {
            s.push('.');
        }
        while s.len() < end_pos {
            s.push('0');
        }
        s
    };

    Ok(out)
}

fn main() {
    let args = Args::parse();

    match calculate_pi(args.n, args.threads) {
        Ok(pi_str) => {
            if let Some(path) = args.output {
                match File::create(&path) {
                    Ok(mut f) => {
                        if let Err(e) = writeln!(f, "{}", pi_str) {
                            eprintln!("Failed to write to {}: {}", path.display(), e);
                        }
                    }
                    Err(e) => eprintln!("Failed to create {}: {}", path.display(), e),
                }
            } else {
                println!("Pi: {}", pi_str);
            }
        }
        Err(e) => eprintln!("Error: {}", e),
    }
}

#[cfg(test)]
mod tests {
    use super::calculate_pi;

    #[test]
    fn pi_10_digits() {
        let pi = calculate_pi(10, 2).expect("calculation failed");
        assert_eq!(pi, "3.1415926535");
    }
}
