#![no_std]
use alloy_primitives::U256;

#[derive(Debug, serde::Serialize, serde::Deserialize, Clone, Copy)]
pub struct SwapValues {
    amount0in: U256,
    amount0out: U256,
    amount1in: U256,
    amount1out: U256,
}

impl SwapValues {
    pub fn new(amount0in: U256, amount0out: U256, amount1in: U256, amount1out: U256) -> Self {
        Self {
            amount0in,
            amount0out,
            amount1in,
            amount1out,
        }
    }

    pub fn calculate_price(&self) -> U256 {
        let lhs = self.amount0in - self.amount0out;
        let rhs = self.amount1out - self.amount1in;
        let (div, rem) = lhs.div_rem(rhs);
        let price = div + rem;
        price
    }
}