#![cfg_attr(feature = "guest", no_std)]
#![no_main]

extern crate alloc;
use alloc::vec::Vec as Avec;

#[jolt::provable]
fn volatility( 
    s: f64,
    ticks: Avec<u32>,
    inv_n: f64,
    inv_n1: f64 
) -> bool {


    let mut sum_ui = 0f64;
    let mut sum_ui2 = 0f64;
    let n = ticks.len();
    for i in 1..n {
        let ui = ticks[i] as f64 - ticks[i - 1] as f64;
        sum_ui += ui;
        sum_ui2 += ui * ui; 
    }
    
    let s2 = inv_n1 as f64 * (sum_ui2 - sum_ui * sum_ui * inv_n as f64);
    let s_check = s2 == s * s;
    let n_check = n as f64 * inv_n == 1.0;
    let n1_check = (n - 1) as f64 * inv_n1 == 1.0;
    return s_check & n_check & n1_check

}