use clap::Parser;
use rug::{Float, Integer, ops::Pow};
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
fn bs(a: u64, b: u64) -> (Integer, Integer, Integer) {
    if b - a == 1 {
        if a == 0 {
            // P = 1, Q = 1, T = 13591409
            return (Integer::from(1), Integer::from(1), Integer::from(13591409));
        }
        let a_i = Integer::from(a as i128);
        let p: Integer = (Integer::from(6 * a as i128 - 5)
            * Integer::from(2 * a as i128 - 1)
            * Integer::from(6 * a as i128 - 1))
            .into();
        let q: Integer = (Integer::from(a as i128).pow(3) * Integer::from(640320i128).pow(3)).into();
        let mut t: Integer = (p.clone() * Integer::from(13591409i128 + 545140134i128 * a_i)).into();
        if a % 2 == 1 {
            t = -t;
        }
        return (p, q, t);
    }
    let m = (a + b) / 2;
    let (p1, q1, t1) = bs(a, m);
    let (p2, q2, t2) = bs(m, b);
    let p = (&p1 * &p2).into();
    let q = (&q1 * &q2).into();
    let t1q2: Integer = (&t1 * &q2).into();
    let p1t2: Integer = (&p1 * &t2).into();
    let t = t1q2 + p1t2;
    (p, q, t)
}

/// Calculate Pi to `n` decimal digits using the Chudnovsky algorithm (binary splitting).
pub fn calculate_pi_chudnovsky(n: u32) -> Result<String, String> {
    if n == 0 {
        return Err("n must be > 0".into());
    }

    // Each term of Chudnovsky yields ~14.181647462725477 decimal digits
    let digits_per_term = 14.181647462725477;
    let terms = ((n as f64) / digits_per_term).ceil() as u64 + 1;

    // Bits of precision: log2(10) ~= 3.321928. Add guard bits.
    let prec = (n as f64 * 3.3219280948873626).ceil() as u32 + 20;

    let (_p, q, t) = bs(0, terms);

    // Convert big integers to high-precision floats
    let prec_u = prec as u32;
    let qf = Float::with_val(prec_u, q);
    let tf = Float::with_val(prec_u, t);

    // C = 426880 * sqrt(10005)
    let c = Float::with_val(prec_u, 426880) * Float::with_val(prec_u, 10005).sqrt();

    let pi = c * qf / tf;

    // Convert to decimal string with a few extra digits for safe truncation.
    let extra = 10usize;
    let pi_string = pi.to_string_radix(10, Some(n as usize + extra));

    // Find dot safely and truncate or pad as needed.
    let dot_pos = pi_string.find('.').unwrap_or(pi_string.len());
    let end_pos = dot_pos + 1 + n as usize;

    let out = if pi_string.len() >= end_pos {
        pi_string[..end_pos].to_string()
    } else {
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
