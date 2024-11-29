pub mod algebra_integral_math;

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uniswap_v3_math::tick_math::{MAX_SQRT_RATIO, MIN_SQRT_RATIO};

use alloy::primitives::{Address, I256, U256};

use super::{consts::U256_1, AutomatedMarketMaker};

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct AlgebraIntegralPool {
    pub address: Address,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub factory_address: Option<Address>,
    pub token_a: Address,
    pub token_a_decimals: u8,
    pub token_b: Address,
    pub token_b_decimals: u8,
    pub liquidity: u128,
    pub sqrt_price: U256,
    pub last_fee: u16,
    pub next_tick_global: i32,
    pub prev_tick_global: i32,
    pub tick: i32,
    pub tick_spacing: i32,
    pub tick_table: HashMap<i16, U256>,
    pub ticks: HashMap<i32, Info>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Info {
    pub liquidity_gross: u128,
    pub liquidity_net: i128,
    pub initialized: bool,
}

pub struct Tick {
    pub liquidity_total: u128,
    pub liquidity_delta: i128,
    pub outer_fee_frowth_0_token: U256,
    pub outer_fee_frowth_1_token: U256,
    pub next_tick: i32,
    pub prev_tick: i32,
}

pub struct GlobalState {
    price: U256,
    tick: i32,
    last_fee: u16,
    plugin_config: u8,
    community_fee: u16,
    unlocked: bool,
}

// impl AutomatedMarketMaker for AlgebraIntegralPool {
//     fn address(&self) -> Address {
//         self.address
//     }

//     fn simulate_swap(
//         &self,
//         base_token: Address,
//         quote_token: Address,
//         amount_in: U256,
//     ) -> Result<U256, crate::errors::SwapSimulationError> {
//         if amount_in.is_zero() {
//             return Ok(U256::ZERO);
//         }

//         let zero_for_one = base_token == self.token_a;

//         // Set sqrt_price_limit_x_96 to the max or min sqrt price in the pool depending on zero_for_one
//         let sqrt_price_limit_x_96 = if zero_for_one {
//             MIN_SQRT_RATIO + U256_1
//         } else {
//             MAX_SQRT_RATIO - U256_1
//         };

//         let mut current_state = SwapCalculationCache {
//             ticks_crossed: 0,
//             amount_required_initial: I256::from_raw(amount_in),
//             amount_specified_remaining: I256::from_raw(amount_in),
//             amount_calculated: I256::ZERO,
//             // exact_input: true,  // might remove later
//             fee: self.last_fee, // might change
//             next_initialized_tick: self.next_tick_global,
//             prev_initialized_tick: self.prev_tick_global,
//             current_liquidity: self.liquidity,
//             current_price: self.sqrt_price,
//             current_tick: self.tick,
//         };

//         let fee = self.last_fee;

//         let mut step = PriceMovementCache {
//             step_sqrt_price: U256::ZERO,
//             next_tick_price: U256::ZERO,
//             input: U256::ZERO,
//             output: U256::ZERO,
//             fee_amount: U256::ZERO,
//         };

//         while current_state.amount_specified_remaining != I256::ZERO
//             && current_state.current_price != sqrt_price_limit_x_96
//         {
//             let next_tick = if zero_for_one {
//                 current_state.prev_initialized_tick
//             } else {
//                 current_state.next_initialized_tick
//             };

//             step.step_sqrt_price = current_state.current_price;
//             step.next_tick_price = uniswap_v3_math::tick_math::get_sqrt_ratio_at_tick(next_tick)?;
//         }

//         Ok(0)
//     }

//     fn tokens(&self) -> Vec<Address> {
//         vec![self.token_a, self.token_b]
//     }
// }

struct SwapCalculationCache {
    // community_fee: U256, // The community fee of the selling token, uint256 to minimize casts
    ticks_crossed: i32, //  If we have already crossed at least one active tick
    amount_required_initial: I256, // The initial value of the exact input\output amount
    amount_specified_remaining: I256,
    amount_calculated: I256, // The additive amount of total output\input calculated through the swap
    // total_fee_growth_input: U256, // The initial totalFeeGrowth + the fee growth during a swap
    // total_fee_growth_output: U256, // The initial totalFeeGrowth for output token, should not change during swap
    // exact_input: bool,          // Whether the exact input or output is specified. Negative amount_required = not exact_input
    fee: u16,                   // The current fee value in hundredths of a bip, i.e. 1e-6
    prev_initialized_tick: i32, // The previous initialized tick in linked list
    next_initialized_tick: i32, // The next initialized tick in linked list
    current_liquidity: u128,
    current_price: U256,
    current_tick: i32,
}

struct PriceMovementCache {
    step_sqrt_price: U256, // The Q64.96 sqrt of the price at the start of the step, uint256 to minimize casts
    next_tick_price: U256, // The Q64.96 sqrt of the price calculated from the _nextTick_, uint256 to minimize casts
    input: U256,           // The additive amount of tokens that have been provided
    output: U256,          // The additive amount of token that have been withdrawn
    fee_amount: U256,      // The total amount of fee earned within a current step
}
