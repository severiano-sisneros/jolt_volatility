#![cfg_attr(feature = "guest", no_std)]
#![no_main]

extern crate alloc;
use alloc::vec::Vec;

use alloy_primitives::U256;
use swap::SwapValues;



#[jolt::provable(max_input_size = 10000, stack_size = 10000, memory_size = 10000000)]
fn volatility(swap_values: Vec<SwapValues>) -> U256 {
    // let n = U256::from(swap_values.len()); // Convert swap_values.len() to U256
    let mut sum_ui = U256::from(0); // Convert 0 to U256
    // let mut sum_ui2 = U256::from(0); // Convert 0 to U256
    for i in 1..swap_values.len() {
        let u_i = swap_values[i].calculate_price() / swap_values[i - 1].calculate_price();
        // let u_i = swap_values[i].calculate_price() ;
        sum_ui += u_i;
        // sum_ui2 += u_i*u_i;
    }

    // let s2 =(sum_ui2 / (n - U256::from(1))) - (sum_ui*sum_ui / (n*(n - U256::from(1)))); 
    sum_ui
    // s2
    // swap_values[0].calculate_price()
}

#[jolt::provable]
fn fib(n: u32) -> u128 {
    let mut a: u128 = 0;
    let mut b: u128 = 1;
    let mut sum: u128;
    for _ in 1..n {
        sum = a + b;
        a = b;
        b = sum;
    }

    b
}
