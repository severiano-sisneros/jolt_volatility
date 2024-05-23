
extern crate alloc;
use alloc::vec::Vec as Avec;

pub fn main() {
    // Create a vector of u32 tick values for test
    let mut ticks = Avec::new();
    for i in 0..16 {
        ticks.push(i % 8);
    }

    let inv_n = 1.0/ticks.len() as f64;
    let inv_n1 = 1.0/(ticks.len() - 1) as f64;

    let mut sum_ui = 0f64;
    let mut sum_ui2 = 0f64;
    for i in 1..ticks.len() {
        let ui = ticks[i] as f64 - ticks[i - 1] as f64;
        sum_ui += ui;
        sum_ui2 += ui * ui; 
    }
    let s2 = inv_n1 as f64 * (sum_ui2 as f64 - sum_ui as f64 * sum_ui as f64 * inv_n as f64);
    let s = s2.sqrt();
    // s2
    let (prove_volatility, verify_volatility) = guest::build_volatility();
    let (output, proof) = prove_volatility(s, ticks, inv_n, inv_n1);
    let is_valid = verify_volatility(proof);
    assert!(is_valid);
    assert_eq!(output, true);
}
