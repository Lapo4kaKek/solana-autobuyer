use anchor_lang::prelude::*;

#[error_code]
pub enum ErrorCode {
    #[msg("Insufficient token balance")]
    InsufficientBalance,

    #[msg("No viable swap route found")]
    NoViableRoute,

    #[msg("Insufficient accounts provided")]
    InsufficientAccounts,
    #[msg("Pool not found for the given token pair")]
    PoolNotFound,

    #[msg("Invalid pool owner - pool does not belong to expected DEX program")]
    InvalidPoolOwner,

    #[msg("Insufficient liquidity in the pool")]
    InsufficientLiquidity,

    #[msg("Slippage exceeded - received amount is less than minimum")]
    SlippageExceeded,

    #[msg("Invalid token mint provided")]
    InvalidTokenMint,

    #[msg("Pool account validation failed")]
    PoolValidationFailed,
    #[msg("Invalid DEX adapter")]
    InvalidDexAdapter,
    #[msg("Unsupported DEX type")]
    UnsupportedDex,
}

#[error_code]
pub enum AutoSwapError {
    #[msg("Insufficient balance for swap")]
    InsufficientBalance,
    #[msg("Slippage tolerance exceeded")]
    SlippageExceeded,
    #[msg("No route found for swap")]
    NoRouteFound,
}
