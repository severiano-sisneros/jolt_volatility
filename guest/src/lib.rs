#![cfg_attr(feature = "guest", no_std)]
#![no_main]

extern crate alloc;
use alloc::vec::Vec;
use fixed::types::I10F22 as Fixed;

#[jolt::provable(max_input_size = 10000)]
fn tick_volatility2(
    values: Vec<[u8; 4]>,
    n_inv_sqrt: [u8; 4],
    n1_inv: [u8; 4],
) -> ([u8; 4], [u8; 4]) {
    let n = Fixed::from_num(values.len());
    let n_inv_sqrt = Fixed::from_be_bytes(n_inv_sqrt);
    let n1_inv = Fixed::from_be_bytes(n1_inv);

    let mut sum_u = Fixed::ZERO;
    let mut sum_u2 = Fixed::ZERO;
    let mut ticks_prev = Fixed::from_be_bytes(values[0]);
    for idx in 1..values.len() {
        let ticks_curr = Fixed::from_be_bytes(values[idx]);
        let delta = ticks_curr - ticks_prev;
        ticks_prev = ticks_curr;
        sum_u += delta * n_inv_sqrt;
        sum_u2 += delta * delta * n1_inv;
    }

    let s2_bytes = Fixed::to_be_bytes(sum_u2 - (sum_u * sum_u) * n1_inv);
    let n_bytes = Fixed::to_be_bytes(n);

    (s2_bytes, n_bytes)
}
