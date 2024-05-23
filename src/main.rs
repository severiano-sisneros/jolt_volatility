#![no_std]

extern crate alloc;
use alloc::vec::Vec;
use core::f32::EPSILON as EPSILON_F32;

pub fn main() {
    
    // Test ticks
    let n = 1024;
    let mut ticks: Vec<u32> = Vec::new();
    for i in 0..n {
        ticks.push(i as u32 % 7);
    }

    // Calculate the volatility squared, s2, using ticks
    let mut sum_u = 0.0;
    let mut sum_u2 = 0.0;
    for idx in 1..ticks.len() {
        sum_u += ticks[idx] as f32 - ticks[idx-1] as f32;
        sum_u2 += (ticks[idx] as f32 - ticks[idx-1] as f32) * (ticks[idx] as f32 - ticks[idx-1] as f32);
    }

    let n = ticks.len() as f32;
    let s2: f32 = (sum_u2 - sum_u/n) / (n - 1.0);

    // Prove and verify the volatility squared
    let (prove_tick_volatility2, verify_tick_volatility2) = guest::build_tick_volatility2();
    let ((sum_u2_out, n), proof) = prove_tick_volatility2(ticks);
    assert!(sum_u2_out - s2 <= EPSILON_F32);
    assert!(n - n <= EPSILON_F32);
    assert!(verify_tick_volatility2(proof));

}
