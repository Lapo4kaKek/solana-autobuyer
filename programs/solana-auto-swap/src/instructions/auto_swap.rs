use anchor_lang::prelude::*;
use anchor_spl::token::Token;

use anchor_lang::Accounts;
use raydium_amm_cpi::SwapBaseIn;

use crate::dex::{pool_finder::find_available_pool, raydium::RaydiumAdapter, Dex, SwapRoute};
use crate::error;

#[derive(Accounts)]
pub struct AutoSwap<'info> {
    /// CHECK: Safe. Token program
    pub token_program: Program<'info, Token>,

    /// CHECK: Safe. AMM program
    pub amm_program: UncheckedAccount<'info>,

    /// CHECK: Safe. AMM account
    #[account(mut)]
    pub amm: UncheckedAccount<'info>,

    /// CHECK: Safe. AMM authority  
    pub amm_authority: UncheckedAccount<'info>,

    /// CHECK: Safe. AMM open orders
    #[account(mut)]
    pub amm_open_orders: UncheckedAccount<'info>,

    /// CHECK: Safe. AMM coin vault
    #[account(mut)]
    pub amm_coin_vault: UncheckedAccount<'info>,

    /// CHECK: Safe. AMM PC vault
    #[account(mut)]
    pub amm_pc_vault: UncheckedAccount<'info>,

    /// CHECK: Safe. Market program
    pub market_program: UncheckedAccount<'info>,

    /// CHECK: Safe. Market
    #[account(mut)]
    pub market: UncheckedAccount<'info>,

    /// CHECK: Safe. Market bids
    #[account(mut)]
    pub market_bids: UncheckedAccount<'info>,

    /// CHECK: Safe. Market asks
    #[account(mut)]
    pub market_asks: UncheckedAccount<'info>,

    /// CHECK: Safe. Market event queue
    #[account(mut)]
    pub market_event_queue: UncheckedAccount<'info>,

    /// CHECK: Safe. Market coin vault
    #[account(mut)]
    pub market_coin_vault: UncheckedAccount<'info>,

    /// CHECK: Safe. Market PC vault
    #[account(mut)]
    pub market_pc_vault: UncheckedAccount<'info>,

    /// CHECK: Safe. Market vault signer
    pub market_vault_signer: UncheckedAccount<'info>,

    /// CHECK: Safe. User token account for swapping
    #[account(mut)]
    pub user_token_account: UncheckedAccount<'info>,

    /// CHECK: Safe. User quote account for swapping  
    #[account(mut)]
    pub user_quote_account: UncheckedAccount<'info>,

    #[account(mut)]
    pub user: Signer<'info>,
}

pub fn auto_swap(
    ctx: Context<AutoSwap>,
    token_mint: Pubkey,
    quote_mint: Pubkey,
    amount_in: u64,
    min_amount_out: u64,
) -> Result<()> {
    let pool_info = find_available_pool(&token_mint, &quote_mint, &ctx.remaining_accounts)?;

    match pool_info.dex_type {
        Dex::Raydium => {
            let dex = RaydiumAdapter::new();
            let route = SwapRoute {
                input_mint: token_mint,
                output_mint: quote_mint,
                amount_in,
                min_amount_out,
                dex_type: Dex::Raydium,
                route_data: Vec::new(),
            };

            let result = dex.raydium_swap_base_in(ctx, route)?;
            msg!("Swap completed, received: {}", result);
            Ok(())
        }
        Dex::Jupiter => {
            return Err(error::ErrorCode::UnsupportedDex.into());
        }
        Dex::Orca => {
            return Err(error::ErrorCode::UnsupportedDex.into());
        }
    }
}

impl<'info> From<&&mut AutoSwap<'info>> for raydium_amm_cpi::SwapBaseIn<'info> {
    fn from(accounts: &&mut AutoSwap<'info>) -> Self {
        SwapBaseIn {
            amm: accounts.amm.clone(),
            amm_authority: accounts.amm_authority.clone(),
            amm_open_orders: accounts.amm_open_orders.clone(),
            amm_coin_vault: accounts.amm_coin_vault.clone(),
            amm_pc_vault: accounts.amm_pc_vault.clone(),
            market_program: accounts.market_program.clone(),
            market: accounts.market.clone(),
            market_bids: accounts.market_bids.clone(),
            market_asks: accounts.market_asks.clone(),
            market_event_queue: accounts.market_event_queue.clone(),
            market_coin_vault: accounts.market_coin_vault.clone(),
            market_pc_vault: accounts.market_pc_vault.clone(),
            market_vault_signer: accounts.market_vault_signer.clone(),
            user_token_source: accounts.user_token_account.clone(),
            user_token_destination: accounts.user_quote_account.clone(),
            user_source_owner: accounts.user.clone(),
            token_program: accounts.token_program.clone(),
        }
    }
}
