use alloy::primitives::{I256, U256};
use uniswap_v3_math::sqrt_price_math::Q96;

use super::{
    error::AlgebraIntegralMathError,
    full_math::{mul_div, mul_div_rounding_up, unsafe_div_rounding_up},
    RESOLUTION,
};

pub fn get_token_0_delta(
    price_lower: U256,
    price_upper: U256,
    liquidity: u128,
    round_up: bool,
) -> Result<U256, AlgebraIntegralMathError> {
    // Gets the token0 delta between two prices
    let price_delta = price_upper - price_lower;
    if price_delta >= price_upper {
        return Err(AlgebraIntegralMathError::PriceDeltaUnderflow);
    }
    let liquidity_shifted = U256::from(liquidity << RESOLUTION);

    if round_up {
        Ok(unsafe_div_rounding_up(
            mul_div_rounding_up(price_delta, liquidity_shifted, price_upper)?,
            price_lower,
        ))
    } else {
        Ok(mul_div(price_delta, liquidity_shifted, price_upper).unwrap() / price_lower)
    }
}

pub fn get_token_1_delta(
    price_lower: U256,
    price_upper: U256,
    liquidity: u128,
    round_up: bool,
) -> Result<U256, AlgebraIntegralMathError> {
    if price_lower >= price_upper {
        return Err(AlgebraIntegralMathError::PriceDeltaUnderflow);
    }
    let price_delta = price_upper - price_lower;
    if round_up {
        mul_div_rounding_up(price_delta, U256::from(liquidity), Q96)
    } else {
        mul_div(price_delta, U256::from(liquidity), Q96)
    }
}

pub fn get_token_0_delta_signed(
    price_lower: U256,
    price_upper: U256,
    liquidity: i128,
) -> Result<I256, AlgebraIntegralMathError> {
    if liquidity > 0 {
        Ok(I256::from_raw(get_token_0_delta(
            price_lower,
            price_upper,
            u128::try_from(liquidity).unwrap(),
            true,
        )?))
    } else {
        Ok(-I256::from_raw(get_token_0_delta(
            price_lower,
            price_upper,
            u128::try_from(-liquidity).unwrap(),
            false,
        )?))
    }
}

pub fn get_token_1_delta_signed(
    price_lower: U256,
    price_upper: U256,
    liquidity: i128,
) -> Result<I256, AlgebraIntegralMathError> {
    if liquidity > 0 {
        Ok(I256::from_raw(get_token_1_delta(
            price_lower,
            price_upper,
            u128::try_from(liquidity).unwrap(),
            true,
        )?))
    } else {
        Ok(-I256::from_raw(get_token_1_delta(
            price_lower,
            price_upper,
            u128::try_from(-liquidity).unwrap(),
            false,
        )?))
    }
}
