use alloy::primitives::{I256, U160, U256};
use uniswap_v3_math::sqrt_price_math::Q96;

use super::{
    error::AlgebraIntegralMathError,
    full_math::{mul_div, mul_div_rounding_up, unsafe_div_rounding_up},
    token_delta_math::get_token_0_delta,
    FEE_DENOMINATOR, RESOLUTION,
};

pub fn get_new_price(
    price: U256,
    liquidity: u128,
    amount: U256,
    zero_to_one: bool,
    from_input: bool,
) -> Result<U256, AlgebraIntegralMathError> {
    if price == U256::ZERO {
        return Err(AlgebraIntegralMathError::SqrtPriceIsZero);
    }
    if liquidity == 0 {
        return Err(AlgebraIntegralMathError::LiquidityIsZero);
    }
    if amount == U256::ZERO {
        return Ok(price);
    }

    if zero_to_one == from_input {
        let liquidity_shifted = U256::from(liquidity) << RESOLUTION;

        if from_input {
            let product = amount * price;
            if product / amount == price {
                let denominator = liquidity_shifted + product;
                if denominator >= liquidity_shifted {
                    return mul_div_rounding_up(liquidity_shifted, price, denominator);
                }
            }
            return Ok(unsafe_div_rounding_up(
                liquidity_shifted,
                liquidity_shifted / price,
            ));
        } else {
            let product = amount * price;
            if product / amount != price {
                return Err(AlgebraIntegralMathError::ProductDivAmount);
            }
            if liquidity_shifted <= product {
                return Err(AlgebraIntegralMathError::ProductDivAmount);
            }
            return mul_div_rounding_up(liquidity_shifted, price, liquidity_shifted - product);
        }
    } else {
        if from_input {
            let quotient = if amount <= U256::from(U160::MAX) {
                (amount << RESOLUTION) / U256::from(liquidity)
            } else {
                mul_div(amount, Q96, U256::from(liquidity))?
            };
            return Ok(price + quotient);
        } else {
            let quotient = if amount <= U256::from(U160::MAX) {
                unsafe_div_rounding_up(amount << RESOLUTION, U256::from(liquidity))
            } else {
                mul_div_rounding_up(amount, Q96, U256::from(liquidity))?
            };
            if price <= quotient {
                return Err(AlgebraIntegralMathError::SqrtPriceIsLteQuotient);
            }
            return Ok(price - quotient);
        }
    }
}

pub fn get_new_price_after_input(
    price: U256,
    liquidity: u128,
    input: U256,
    zero_to_one: bool,
) -> Result<U256, AlgebraIntegralMathError> {
    // Gets the next sqrt price given an input amount of token0 or token1.
    // Throws if price or liquidity are 0, or if the next price is out of bounds.
    get_new_price(price, liquidity, input, zero_to_one, true)
}

pub fn get_new_price_after_output(
    price: U256,
    liquidity: u128,
    input: U256,
    zero_to_one: bool,
) -> Result<U256, AlgebraIntegralMathError> {
    // Gets the next sqrt price given an output amount of token0 or token1.
    // Throws if price or liquidity are 0, or if the next price is out of bounds.
    get_new_price(price, liquidity, input, zero_to_one, false)
}

pub fn get_input_token_delta_0_1(
    to: U256,
    from: U256,
    liquidity: u128,
) -> Result<U256, AlgebraIntegralMathError> {
    get_token_0_delta(to, from, liquidity, true)
}

pub fn get_input_token_delta_1_0(
    to: U256,
    from: U256,
    liquidity: u128,
) -> Result<U256, AlgebraIntegralMathError> {
    get_token_0_delta(from, to, liquidity, true)
}

pub fn get_output_token_delta_0_1(
    to: U256,
    from: U256,
    liquidity: u128,
) -> Result<U256, AlgebraIntegralMathError> {
    get_token_0_delta(to, from, liquidity, false)
}

pub fn get_output_token_delta_1_0(
    to: U256,
    from: U256,
    liquidity: u128,
) -> Result<U256, AlgebraIntegralMathError> {
    get_token_0_delta(from, to, liquidity, false)
}

fn move_price_towards_target(
    zero_to_one: bool,
    current_price: U256,
    target_price: U256,
    liquidity: u128,
    amount_available: I256,
    fee: u16,
) -> Result<(U256, U256, U256, U256), AlgebraIntegralMathError> {
    // Computes the result of swapping some amount in, or amount out,
    // given the parameters of the swap.
    // The fee, plus the amount in, will never exceed the amount
    // remaining if the swap's `amount_available` is positive.
    let result_price: U256;
    let fee_amount: U256;
    let mut input: U256;
    let mut output: U256;

    let get_input_token_amount: fn(U256, U256, u128) -> Result<U256, AlgebraIntegralMathError> =
        if zero_to_one {
            get_input_token_delta_0_1
        } else {
            get_input_token_delta_1_0
        };

    let get_output_token_amount: fn(U256, U256, u128) -> Result<U256, AlgebraIntegralMathError> =
        if zero_to_one {
            get_output_token_delta_0_1
        } else {
            get_output_token_delta_1_0
        };

    if amount_available >= I256::ZERO {
        let amount_available_after_fee = mul_div(
            amount_available.into_sign_and_abs().1,
            U256::from(FEE_DENOMINATOR - fee as u32),
            U256::from(FEE_DENOMINATOR),
        )?;
        input = get_input_token_amount(target_price, current_price, liquidity)?;
        if U256::try_from(amount_available_after_fee).unwrap() >= input {
            result_price = target_price;
            fee_amount = mul_div_rounding_up(
                input,
                U256::from(fee),
                U256::from(FEE_DENOMINATOR - fee as u32),
            )?;
        } else {
            result_price = get_new_price_after_input(
                current_price,
                liquidity,
                amount_available_after_fee,
                zero_to_one,
            )?;
            // We didn't reach the target, so take the remainder of the maximum input as fee
            input = get_input_token_amount(result_price, current_price, liquidity)?;
            fee_amount = amount_available.into_sign_and_abs().1 - input;
        }
        output = if zero_to_one {
            get_output_token_delta_0_1(result_price, current_price, liquidity)?
        } else {
            get_output_token_delta_1_0(result_price, current_price, liquidity)?
        };
    } else {
        output = get_output_token_amount(target_price, current_price, liquidity)?;
        let amount_available = -amount_available;
        let amount_available_abs = amount_available.into_sign_and_abs().1;
        if amount_available < I256::ZERO {
            return Err(AlgebraIntegralMathError::InvalidAmountRequired);
        }
        if amount_available_abs >= output {
            result_price = target_price;
        } else {
            result_price = get_new_price_after_output(
                current_price,
                liquidity,
                amount_available_abs,
                zero_to_one,
            )?;
            if target_price != result_price {
                output = get_output_token_amount(result_price, current_price, liquidity)?;
            }
            if output > amount_available_abs {
                output = amount_available_abs;
            }
        }
        input = get_input_token_amount(result_price, current_price, liquidity)?;
        fee_amount = mul_div_rounding_up(
            input,
            U256::from(fee),
            U256::from(FEE_DENOMINATOR - fee as u32),
        )?;
    }
    Ok((result_price, input, output, fee_amount))
}
