use anchor_lang::prelude::*;

#[error_code]
pub enum AutoSwapError {
    #[msg("Insufficient balance for swap")]
    InsufficientBalance,
    #[msg("Slippage tolerance exceeded")]
    SlippageExceeded,
    #[msg("No route found for swap")]
    NoRouteFound,
}
#[error_code]
pub enum ErrorCode {
    #[msg("Insufficient token balance")]
    InsufficientBalance,
    
    #[msg("No viable swap route found")]
    NoViableRoute,
    
    #[msg("Insufficient accounts provided")]
    InsufficientAccounts,
    
    #[msg("Slippage tolerance exceeded")]
    SlippageExceeded,
    
    #[msg("Invalid DEX adapter")]
    InvalidDexAdapter,
}