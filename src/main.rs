

extern crate alloc;
extern crate clap;

use alloc::vec::Vec;
use core::f32::EPSILON as EPSILON_F32;
use clap::Parser;
use std::io::{self, BufRead};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// The input CSV file (use '-' for stdin)
    #[arg(short, long)]
    input: String,
}

pub fn main() {
    let args = Args::parse();
    let ticks: Vec<u32> = if args.input == "-" {
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

    // Calculate the volatility squared, s2, using ticks
    let mut sum_u = 0.0;
    let mut sum_u2 = 0.0;
    for idx in 1..ticks.len() {
        let delta = ticks[idx] as f32 - ticks[idx - 1] as f32;
        sum_u += delta;
        sum_u2 += delta * delta;
    }

    let n = ticks.len() as f32;
    let s2: f32 = (sum_u2 - (sum_u * sum_u) / n) / (n - 1.0);

    // Prove and verify the volatility squared
    let (prove_tick_volatility2, verify_tick_volatility2) = guest::build_tick_volatility2();
    let ((sum_u2_out, n_out), proof) = prove_tick_volatility2(ticks);
    assert!((sum_u2_out - s2).abs() <= EPSILON_F32);
    assert!((n_out - n).abs() <= EPSILON_F32);
    assert!(verify_tick_volatility2(proof));
}

fn read_ticks_from_reader<R: BufRead>(reader: &mut R) -> Vec<u32> {
    let mut ticks = Vec::new();
    let mut line = String::new();
    // Skip the header line
    reader.read_line(&mut line).expect("Failed to read line");
    line.clear();
    while reader.read_line(&mut line).expect("Failed to read line") > 0 {
        if let Ok(value) = line.trim().parse::<u32>() {
            ticks.push(value);
        } else {
            panic!("Invalid number in CSV");
        }
        line.clear();
    }
    ticks
}
