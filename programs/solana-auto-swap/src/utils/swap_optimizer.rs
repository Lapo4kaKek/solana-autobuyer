use anchor_lang::prelude::*;
use crate::dex::{DexType, SwapRoute};
use crate::errors::ErrorCode;

pub struct SwapOptimizer;

impl SwapOptimizer {
    pub fn find_best_route(
        input_mint: Pubkey,
        output_mint: Pubkey,
        amount_in: u64,
        enabled_dexes: &[DexType],
    ) -> Result<SwapRoute> {
        let mut best_route: Option<SwapRoute> = None;
        let mut best_amount_out = 0u64;
        
        for dex_type in enabled_dexes {
            let adapter = dex_type.get_adapter();
            
            match adapter.get_quote(input_mint, output_mint, amount_in) {
                Ok(amount_out) => {
                    if amount_out > best_amount_out {
                        best_amount_out = amount_out;
                        best_route = Some(SwapRoute {
                            input_mint,
                            output_mint,
                            amount_in,
                            min_amount_out: amount_out,
                            dex_type: *dex_type,
                            route_data: vec![],
                        });
                    }
                }
                Err(_) => continue, // skip DEX if liquidity empty
            }
        }
        
        best_route.ok_or(ErrorCode::NoViableRoute.into())
    }
    
    pub fn calculate_price_impact(
        amount_in: u64,
        amount_out: u64,
        market_price: u64,
    ) -> u64 {
        // Simple calculate price impact
        let expected_out = amount_in * market_price / 10000;
        if expected_out > amount_out {
            ((expected_out - amount_out) * 10000) / expected_out
        } else {
            0
        }
    }
}
