use crate::dex::{DexAdapter, SwapRoute};
use crate::AutoSwap;
use anchor_lang::prelude::*;
use raydium_amm_cpi::SwapBaseIn;

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
    pub fn raydium_swap_base_in(&self, ctx: Context<AutoSwap>, route: SwapRoute) -> Result<u64> {
        let token_program = ctx.accounts.token_program.clone();
        msg!("Token program : {:?}", token_program.key());
        msg!("Token program data len : {:?}", token_program.data_len());

        let raydium_swap_accounts = SwapBaseIn::from(&ctx.accounts);

        let cpi_ctx = CpiContext::new(
            ctx.accounts.amm_program.to_account_info(),
            raydium_swap_accounts,
        );
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
        _input_mint: Pubkey,
        _output_mint: Pubkey,
        _amount: u64,
    ) -> anchor_lang::Result<u64> {
        todo!()
    }

    fn get_required_accounts(
        &self,
        _route: &super::SwapRoute,
    ) -> Vec<anchor_lang::prelude::AccountMeta> {
        todo!()
    }

    fn execute_swap<'info>(
        &self,
        _ctx: &anchor_lang::prelude::Context<'_, '_, '_, 'info, crate::AutoSwap<'info>>,
        _route: super::SwapRoute,
    ) -> anchor_lang::Result<u64> {
        todo!()
    }
}
