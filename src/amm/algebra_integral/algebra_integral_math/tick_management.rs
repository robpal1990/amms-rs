use alloy::primitives::U256;

pub struct Tick {
    pub liquidity_total: U256,
    pub liquidity_delta: i128,
    pub prev_tick: i32,
    pub next_tick: i32,
    pub outer_fee_growth_0_token: U256,
    pub outer_fee_growth_1_token: U256,
}
