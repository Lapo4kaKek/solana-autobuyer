use anchor_lang::prelude::*;

pub mod pool_finder;
pub mod raydium;
pub trait DexAdapter {
    fn get_quote(&self, input_mint: Pubkey, output_mint: Pubkey, amount: u64) -> Result<u64>;

    fn get_required_accounts(&self, route: &SwapRoute) -> Vec<AccountMeta>;
    fn execute_swap<'info>(
        &self,
        ctx: &Context<'_, '_, '_, 'info, crate::AutoSwap<'info>>,
        route: SwapRoute,
    ) -> Result<u64>;
}

#[derive(Debug, Clone)]
pub struct SwapRoute {
    pub input_mint: Pubkey,
    pub output_mint: Pubkey,
    pub amount_in: u64,
    pub min_amount_out: u64,
    pub dex_type: Dex,
    pub route_data: Vec<u8>,
}

#[derive(AnchorSerialize, AnchorDeserialize, Debug, Clone, Copy, PartialEq)]
pub enum Dex {
    Jupiter,
    Raydium,
    Orca,
}

impl Default for Dex {
    fn default() -> Self {
        Dex::Raydium
    }
}

impl Dex {
    pub fn get_adapter(&self) -> Box<dyn DexAdapter> {
        match self {
            Dex::Jupiter => todo!(),
            Dex::Raydium => Box::new(raydium::RaydiumAdapter::new()),
            Dex::Orca => todo!(),
        }
    }
}
