extern crate alloc;
extern crate clap;

use alloc::vec::Vec;
use clap::Parser;
use fixed::types::I15F17 as Fixed;
use std::io::{self, BufRead};
use std::time::Instant;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// The input CSV file (use '-' for stdin)
    #[arg(short, long)]
    input: String,
}

pub fn main() {
    let args = Args::parse();
    let ticks: Vec<[u8; 4]> = if args.input == "-" {
        // Read from stdin
        let stdin = io::stdin();
        let mut handle = stdin.lock();
        read_ticks_from_reader(&mut handle)
    } else {
        // Read from file
        let file = std::fs::File::open(args.input).expect("Could not open file");
        let mut reader = std::io::BufReader::new(file);
        read_ticks_from_reader(&mut reader)
    };

    // Calculate  1/(n-1) and the square root of 1/n.
    // These values are used in the volatility proof.
    let n = Fixed::from_num(ticks.len());
    let n_inv_sqrt = Fixed::ONE / n.sqrt();
    let n_inv_sqrt_bytes = Fixed::to_be_bytes(n_inv_sqrt);
    let n1_inv = Fixed::ONE / (n - Fixed::ONE);
    let n1_inv_bytes = Fixed::to_be_bytes(n1_inv);

    // Calculate the volatility squared, s2, using ticks
    let mut ticks_prev = Fixed::from_be_bytes(ticks[0]);
    let (sum_u, sum_u2) = ticks
        .iter()
        .fold((Fixed::ZERO, Fixed::ZERO), |(su, su2), tick| {
            let ticks_curr = Fixed::from_be_bytes(*tick);
            let delta = ticks_curr - ticks_prev;
            ticks_prev = ticks_curr;
            (su + delta * n_inv_sqrt, su2 + delta * delta * n1_inv)
        });

    let s2 = sum_u2 - (sum_u * sum_u) * n1_inv;
    println!("Volatility squared: {}", s2);

    // Build the volatility circuit
    println!("Building circuit...");
    let (prove_tick_volatility2, verify_tick_volatility2) = guest::build_tick_volatility2();
    println!("Done!");

    // Prove volatility
    println!("Proving...");
    let start_time = Instant::now();
    let ((s2_out, n_out), proof) = prove_tick_volatility2(ticks, n_inv_sqrt_bytes, n1_inv_bytes);
    let s2_out = Fixed::from_be_bytes(s2_out);
    let n_out = Fixed::from_be_bytes(n_out);
    println!("s2: {:?}, n: {:?}", s2_out, n_out);
    println!("Done!");
    let prove_time = Instant::now() - start_time;
    println!("Prove time: {} seconds", prove_time.as_secs());

    // Verify volatitility
    println!("Verifying...");
    assert!(s2_out == s2);
    assert!(n_out == n);
    assert!(verify_tick_volatility2(proof));
    println!("All checks passed!");
}

fn read_ticks_from_reader<R: BufRead>(reader: &mut R) -> Vec<[u8; 4]> {
    let mut ticks = Vec::new();
    let mut line = String::new();
    // Skip the header line
    reader.read_line(&mut line).expect("Failed to read line");
    line.clear();
    while reader.read_line(&mut line).expect("Failed to read line") > 0 {
        if let Ok(value) = line.trim().parse::<i32>() {
            ticks.push(value.to_be_bytes());
        } else {
            panic!("Invalid number in CSV");
        }
        line.clear();
    }
    ticks
}
