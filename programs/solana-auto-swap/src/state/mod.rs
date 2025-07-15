use crate::dex::Dex;
use anchor_lang::prelude::*;

#[account]
pub struct AutoSwapConfig {
    pub authority: Pubkey,
    pub fee_rate: u64, // basis points
    pub enabled_dexes: Vec<Dex>,
    pub slippage_tolerance: u64,
    pub bump: u8,
}

#[account]
pub struct SwapHistory {
    pub user: Pubkey,
    pub input_mint: Pubkey,
    pub output_mint: Pubkey,
    pub amount_in: u64,
    pub amount_out: u64,
    pub dex_used: Dex,
    pub timestamp: i64,
    pub price_impact: u64,
}

impl AutoSwapConfig {
    pub const SIZE: usize = 8 + 32 + 8 + 4 + 32 * 3 + 8 + 1;
}
