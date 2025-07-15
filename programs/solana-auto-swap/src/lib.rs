pub mod constants;
pub mod dex;
pub mod error;
pub mod instructions;
pub mod state;

use anchor_lang::prelude::*;

pub use auto_swap::*;
pub use constants::*;
use instructions::{__client_accounts_auto_swap, auto_swap};

declare_id!("4ErYWQ8xAS1uCsUS9SV91fyPBfE4A6FbJPPpEAWVqmPM");

#[program]
pub mod solana_auto_swap {
    use crate::dex::{raydium::RaydiumAdapter, SwapRoute};

    use super::*;

    pub fn auto_swap(
        ctx: Context<AutoSwap>,
        token_mint: Pubkey,
        quote_mint: Pubkey,
        amount_in: u64,
        min_amount_out: u64,
    ) -> Result<()> {
        msg!(
            "Auto swap: {} -> {}, amount: {}",
            token_mint,
            quote_mint,
            amount_in
        );

        let dex = RaydiumAdapter::new();
        let route = SwapRoute {
            input_mint: token_mint,
            output_mint: quote_mint,
            amount_in,
            min_amount_out,
            dex_type: dex::DexType::Raydium,
            route_data: Vec::new(),
        };

        let result = dex.raydium_swap_base_in(ctx, route);
        match result {
            Ok(res) => {
                msg!("res:{:?}", res);
            } Err(e) => {
                msg!("error:{:?}", e);
            }
        }
        Ok(())
    }
}
