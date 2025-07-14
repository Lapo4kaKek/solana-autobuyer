use crate::dex::{DexAdapter, SwapRoute};
use anchor_lang::prelude::*;
use anyhow::Result;
use raydium_amm_cpi::{
    library::SwapInstructionBaseIn, program::RaydiumAmm, Deposit, SwapBaseIn, Withdraw,
};

pub struct RaydiumAdapter {
    pub program_id: Pubkey,
}

impl RaydiumAdapter {
    pub fn new() -> Self {
        Self {
            // Raydium AMM program ID
            program_id: Pubkey::from_str_const("675kPX9MHTjS2zt1qfr1NYHuzeLXfQM9H24wFSUt1Mp8"),
        }
    }
    pub fn raydium_swap_base_in<'info>(
        &self,
        ctx: &Context<crate::AutoSwap>,
        route: SwapRoute,
    ) -> Result<u64> {
        let accounts_iter = &mut ctx.remaining_accounts.iter();

        let amm_program = next_account_info(accounts_iter)?;
        let amm_pool = next_account_info(accounts_iter)?;
        let amm_authority = next_account_info(accounts_iter)?;
        let amm_open_orders = next_account_info(accounts_iter)?;
        let amm_coin_vault = next_account_info(accounts_iter)?;
        let amm_pc_vault = next_account_info(accounts_iter)?;
        let market_program = next_account_info(accounts_iter)?;
        let market = next_account_info(accounts_iter)?;
        let market_bids = next_account_info(accounts_iter)?;
        let market_asks = next_account_info(accounts_iter)?;
        let market_event_queue = next_account_info(accounts_iter)?;
        let market_coin_vault = next_account_info(accounts_iter)?;
        let market_pc_vault = next_account_info(accounts_iter)?;
        let market_vault_signer = next_account_info(accounts_iter)?;

        let swap_accounts = SwapBaseIn {
            amm: UncheckedAccount::try_from(&amm_pool),
            amm_authority: UncheckedAccount::try_from(&amm_authority),
            amm_open_orders: UncheckedAccount::try_from(&amm_open_orders),
            amm_coin_vault: UncheckedAccount::try_from(amm_coin_vault),
            amm_pc_vault: UncheckedAccount::try_from(amm_pc_vault),
            market_program: UncheckedAccount::try_from(market_program),
            market: UncheckedAccount::try_from(market),
            market_bids: UncheckedAccount::try_from(market_bids),
            market_asks: UncheckedAccount::try_from(market_asks),
            market_event_queue: UncheckedAccount::try_from(market_event_queue),
            market_coin_vault: UncheckedAccount::try_from(market_coin_vault),
            market_pc_vault: UncheckedAccount::try_from(market_pc_vault),
            market_vault_signer: UncheckedAccount::try_from(market_vault_signer),
            user_token_source: UncheckedAccount::try_from(&ctx.accounts.user_token_account),
            user_token_destination: UncheckedAccount::try_from(&ctx.accounts.user_quote_account),
            user_source_owner: ctx.accounts.user,
            token_program: ctx.accounts.token_program.clone(),
        };
        let cpi_ctx = CpiContext::new(amm_program.clone(), swap_accounts);
        raydium_amm_cpi::swap_base_in(cpi_ctx, route.amount_in, route.min_amount_out)?;

        msg!(
            "Raydium swap executed: {} -> {}",
            route.amount_in,
            route.min_amount_out
        );
        Ok(route.min_amount_out)
    }
}

impl DexAdapter for RaydiumAdapter {
    fn get_quote(
        &self,
        input_mint: Pubkey,
        output_mint: Pubkey,
        amount: u64,
    ) -> anchor_lang::Result<u64> {
        todo!()
    }

    fn get_required_accounts(
        &self,
        route: &super::SwapRoute,
    ) -> Vec<anchor_lang::prelude::AccountMeta> {
        todo!()
    }

    fn execute_swap<'info>(
        &self,
        ctx: &anchor_lang::prelude::Context<'_, '_, '_, 'info, crate::AutoSwap<'info>>,
        route: super::SwapRoute,
    ) -> anchor_lang::Result<u64> {
        todo!()
    }
}