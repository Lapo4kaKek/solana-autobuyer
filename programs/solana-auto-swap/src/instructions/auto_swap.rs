use anchor_lang::prelude::*;
use anchor_spl::token::Token;

use anchor_lang::Accounts;

use crate::dex::{raydium::RaydiumAdapter, SwapRoute};

#[derive(Accounts)]
pub struct AutoSwap<'info> {
    #[account(mut)]
    pub user: Signer<'info>,

    /// CHECK: Token mint account
    pub token_mint: AccountInfo<'info>,

    /// CHECK: Quote mint account  
    pub quote_mint: AccountInfo<'info>,

    /// CHECK: User token account
    #[account(mut)]
    pub user_token_account: AccountInfo<'info>,

    /// CHECK: User quote account
    #[account(mut)]
    pub user_quote_account: AccountInfo<'info>,

    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
}

pub fn handler(
    ctx: Context<AutoSwap>,
    token_mint: Pubkey,
    quote_mint: Pubkey,
    amount_in: u64,
    min_amount_out: u64,
) -> Result<()> {
    let route = SwapRoute {
        input_mint: token_mint,
        output_mint: quote_mint,
        amount_in,
        min_amount_out,
        dex_type: crate::dex::DexType::Raydium,
        route_data: Vec::new(),
    };

    let raydium_dex = RaydiumAdapter::new();

    let actual_amount_out = raydium_dex.raydium_swap_base_in(&ctx, route);
    match actual_amount_out {
        Ok(value) => {
            msg!(
                "Swap completed: {} in -> {} out",
                amount_in,
                value
            );
        }
        Err(e) => {
            msg!("Swap failed with error: {:?}", e);
        }
    }

    Ok(())
}
