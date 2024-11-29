use alloy::primitives::ruint::ParseError;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum AlgebraIntegralMathError {
    #[error("Denominator is 0")]
    DenominatorIsZero,
    #[error("Result is U256::MAX")]
    ResultIsU256MAX,
    #[error("Sqrt price is 0")]
    SqrtPriceIsZero,
    #[error("Sqrt price is less than or equal to quotient")]
    SqrtPriceIsLteQuotient,
    #[error("Can not get most significant bit or least significant bit on zero value")]
    ZeroValue,
    #[error("Liquidity is 0")]
    LiquidityIsZero,
    #[error("Invalid amountRequired")]
    InvalidAmountRequired,
    //TODO: Update this, shield your eyes for now
    #[error(
        "require((product = amount * sqrtPX96) / amount == sqrtPX96 && numerator1 > product);"
    )]
    ProductDivAmount,
    #[error("Denominator is less than or equal to prod_1")]
    DenominatorIsLteProdOne,
    #[error("PriceDeltaUnderflow")]
    PriceDeltaUnderflow,
    #[error("Liquidity Sub")]
    LiquiditySub,
    #[error("Liquidity Add")]
    LiquidityAdd,
    #[error("The given tick must be less than, or equal to, the maximum tick")]
    T,
    #[error(
        "Second inequality must be < because the price can never reach the price at the max tick"
    )]
    R,
    #[error("Overflow when casting to U160")]
    SafeCastToU160Overflow,
    #[error("Tick spacing error")]
    TickSpacingError,
    #[error("Middleware error when getting next_initialized_tick_within_one_word")]
    MiddlewareError(String),
    #[error("Parse error")]
    ParseError(#[from] ParseError),
}
