use super::tick_math::get_sqrt_ratio_at_tick;
use alloy::primitives::{I256, U256};

use super::error::AlgebraIntegralMathError;

use super::token_delta_math::{get_token_0_delta_signed, get_token_1_delta_signed};

// returns (uint128 z)
pub fn add_delta(x: u128, y: i128) -> Result<u128, AlgebraIntegralMathError> {
    if y < 0 {
        let z = x.overflowing_sub(-y as u128);

        if z.1 {
            Err(AlgebraIntegralMathError::LiquiditySub)
        } else {
            Ok(z.0)
        }
    } else {
        let z = x.overflowing_add(y as u128);
        if z.0 < x {
            Err(AlgebraIntegralMathError::LiquidityAdd)
        } else {
            Ok(z.0)
        }
    }
}

pub fn get_amounts_for_liquidity(
    bottom_tick: i32,
    top_tick: i32,
    liquidity_delta: i128,
    current_tick: i32,
    current_price: U256,
) -> Result<(U256, U256, i128), AlgebraIntegralMathError> {
    let price_at_bottom_tick = get_sqrt_ratio_at_tick(bottom_tick).unwrap();
    let price_at_top_tick = get_sqrt_ratio_at_tick(top_tick).unwrap();

    let mut amount_0 = I256::ZERO;
    let mut amount_1 = I256::ZERO;
    let mut global_liquidity_delta = 0_i128;

    if current_tick < bottom_tick {
        amount_0 =
            get_token_0_delta_signed(price_at_bottom_tick, price_at_top_tick, liquidity_delta)?;
    } else if current_tick < top_tick {
        amount_0 = get_token_0_delta_signed(current_price, price_at_top_tick, liquidity_delta)?;
        amount_1 = get_token_1_delta_signed(price_at_bottom_tick, current_price, liquidity_delta)?;
        global_liquidity_delta = liquidity_delta;
    } else {
        amount_1 =
            get_token_1_delta_signed(price_at_bottom_tick, price_at_top_tick, liquidity_delta)?;
    }
    if liquidity_delta < 0 {
        Ok((
            U256::try_from(-amount_0).unwrap(),
            U256::try_from(-amount_1).unwrap(),
            global_liquidity_delta,
        ))
    } else {
        Ok((
            U256::try_from(amount_0).unwrap(),
            U256::try_from(amount_1).unwrap(),
            global_liquidity_delta,
        ))
    }
}

#[cfg(test)]
mod test {

    use super::add_delta;

    #[test]
    fn test_add_delta() {
        // 1 + 0
        let result = add_delta(1, 0);
        assert_eq!(result.unwrap(), 1);

        // 1 + -1
        let result = add_delta(1, -1);
        assert_eq!(result.unwrap(), 0);

        // 1 + 1
        let result = add_delta(1, 1);
        assert_eq!(result.unwrap(), 2);

        // 2**128-15 + 15 overflows
        let result = add_delta(340282366920938463463374607431768211441, 15);
        assert_eq!(result.err().unwrap().to_string(), "Liquidity Add");

        // 0 + -1 underflows
        let result = add_delta(0, -1);
        assert_eq!(result.err().unwrap().to_string(), "Liquidity Sub");

        // 3 + -4 underflows
        let result = add_delta(3, -4);
        assert_eq!(result.err().unwrap().to_string(), "Liquidity Sub");
    }
}
