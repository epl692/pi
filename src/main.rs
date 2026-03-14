use clap::Parser;
use num_bigint::BigInt;
use num_traits::{One, Zero};
use rayon::join;
use std::fs::File;
use std::io::Write;
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Number of digits of Pi to calculate (digits after the decimal point)
    n: u32,

    /// Number of threads to use (kept for compatibility; Chudnovsky is CPU bound)
    #[arg(short, long, default_value_t = 4)]
    threads: usize,

    /// Optional output file (writes result there if provided)
    #[arg(short, long)]
    output: Option<PathBuf>,
}

// Binary splitting for the Chudnovsky algorithm.
// Returns (P, Q, T) as big integers for the interval [a, b)
fn bs(a: u64, b: u64) -> (BigInt, BigInt, BigInt) {
    if b - a == 1 {
        if a == 0 {
            // P = 1, Q = 1, T = 13591409
            return (BigInt::one(), BigInt::one(), BigInt::from(13591409u64));
        }
        let ai = BigInt::from(a);
        let p = BigInt::from(6 * a - 5) * BigInt::from(2 * a - 1) * BigInt::from(6 * a - 1);
        let c = BigInt::from(640320u64);
        let q = (&ai * &ai * &ai) * (&c * &c * &c);
        let mut t = &p * (BigInt::from(13591409u64) + BigInt::from(545140134u64) * &ai);
        if a % 2 == 1 {
            t = -t;
        }
        return (p, q, t);
    }
    let m = (a + b) / 2;
    let (left, right) = join(|| bs(a, m), || bs(m, b));
    let (p1, q1, t1) = left;
    let (p2, q2, t2) = right;
    let p = &p1 * &p2;
    let q = &q1 * &q2;
    let t1q2: BigInt = &t1 * &q2;
    let p1t2: BigInt = &p1 * &t2;
    let t = t1q2 + p1t2;
    (p, q, t)
}

/// Integer square root: returns floor(sqrt(n))
fn isqrt(n: &BigInt) -> BigInt {
    if n <= &BigInt::zero() {
        return BigInt::zero();
    }
    let bits = n.bits() as usize;
    let mut x: BigInt = BigInt::one() << ((bits + 1) / 2);
    loop {
        let y = (&x + n / &x) >> 1usize;
        if y >= x {
            return x;
        }
        x = y;
    }
}

/// Compute 10^exp using repeated squaring
fn pow10(exp: usize) -> BigInt {
    if exp == 0 {
        return BigInt::one();
    }
    let half = pow10(exp / 2);
    let sq = &half * &half;
    if exp % 2 == 0 {
        sq
    } else {
        sq * BigInt::from(10u32)
    }
}

/// Calculate Pi to `n` decimal digits using the Chudnovsky algorithm (binary splitting).
pub fn calculate_pi_chudnovsky(n: u32) -> Result<String, String> {
    if n == 0 {
        return Err("n must be > 0".into());
    }

    // Each term of Chudnovsky yields ~14.181647462725477 decimal digits
    let digits_per_term = 14.181647462725477;
    let terms = ((n as f64) / digits_per_term).ceil() as u64 + 1;

    let (_p, q, t) = bs(0, terms);

    // Compute pi using integer arithmetic:
    //   pi = 426880 * sqrt(10005) * Q / T
    // We compute pi * 10^work_prec as a big integer, then format.
    let extra = 20usize;
    let work_prec = n as usize + extra;

    // sqrt(10005) * 10^work_prec = isqrt(10005 * 10^(2*work_prec))
    let scale_sq = pow10(2 * work_prec);
    let sqrt_10005_scaled = isqrt(&(BigInt::from(10005u32) * scale_sq));

    // pi * 10^work_prec = 426880 * sqrt_10005_scaled * Q / T
    let pi_scaled = BigInt::from(426880u32) * sqrt_10005_scaled * q / t;

    // pi_scaled ≈ 3.14159... * 10^work_prec; insert decimal point after first digit.
    let s = pi_scaled.to_str_radix(10);
    if s.len() < 2 + n as usize {
        return Err("insufficient precision".into());
    }
    let out = format!("{}.{}", &s[..1], &s[1..1 + n as usize]);

    Ok(out)
}

fn main() {
    let args = Args::parse();

    match calculate_pi_chudnovsky(args.n) {
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
    use super::calculate_pi_chudnovsky;

    #[test]
    fn pi_10_digits() {
        let pi = calculate_pi_chudnovsky(10).expect("calculation failed");
        assert_eq!(pi, "3.1415926535");
    }
}
