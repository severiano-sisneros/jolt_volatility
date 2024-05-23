#![cfg_attr(feature = "guest", no_std)]
#![no_main]


extern crate alloc;
use alloc::vec::Vec;

#[jolt::provable]
fn tick_volatility2 (values: Vec<u32>) -> (f32, f32) {
    let n = values.len() as f32;
    let mut sum_u: f32 = 0.0;
    let mut sum_u2: f32 = 0.0;
    for idx in 1..n as usize {
        sum_u += values[idx] as f32 - values[idx-1] as f32;
        sum_u2 += (values[idx] as f32 - values[idx-1] as f32) * (values[idx] as f32 - values[idx-1] as f32);
    }
    ((sum_u2 - sum_u/n)/(n-1.0), n)
}