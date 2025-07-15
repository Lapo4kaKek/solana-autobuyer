use anchor_lang::prelude::*;

use crate::dex::Dex;

use crate::error;

#[derive(Clone, Debug)]
pub struct PoolInfo {
    pub address: Pubkey,
    pub dex_type: Dex,
    pub token_a_vault: Pubkey,
    pub token_b_vault: Pubkey,
    pub authority: Pubkey,
}

fn parse_raydium_pool_data(pool_account: &AccountInfo) -> Result<PoolInfo> {
    // Raydium pool structure
    let data = pool_account.try_borrow_data()?;

    // Offsets for Raydium AMM pool (need to check the documentation)
    let token_a_vault = Pubkey::new_from_array(
        data[100..132]
            .try_into()
            .map_err(|_| error::ErrorCode::PoolValidationFailed)?,
    );
    let token_b_vault = Pubkey::new_from_array(
        data[132..164]
            .try_into()
            .map_err(|_| error::ErrorCode::PoolValidationFailed)?,
    );
    let authority = Pubkey::new_from_array(
        data[164..196]
            .try_into()
            .map_err(|_| error::ErrorCode::PoolValidationFailed)?,
    );

    Ok(PoolInfo {
        address: pool_account.key(),
        dex_type: Dex::Raydium,
        token_a_vault,
        token_b_vault,
        authority,
    })
}

pub fn find_available_pool(
    token_a: &Pubkey,
    token_b: &Pubkey,
    remaining_accounts: &[AccountInfo],
) -> Result<PoolInfo> {
    find_raydium_pool(token_a, token_b, remaining_accounts)
        .or_else(|_| find_meteora_pool(token_a, token_b, remaining_accounts))
        .or_else(|_| find_hardcoded_pool(token_a, token_b))
}

fn find_raydium_pool(
    token_a: &Pubkey,
    token_b: &Pubkey,
    remaining_accounts: &[AccountInfo],
) -> Result<PoolInfo> {
    // Generate PDA for raydium pool
    let (base, quote) = if token_a < token_b {
        (token_a, token_b)
    } else {
        (token_b, token_a)
    };

    let seeds = &[b"amm", base.as_ref(), quote.as_ref()];
    let (pool_address, _) =
        Pubkey::find_program_address(seeds, &crate::constants::RAYDIUM_PROGRAM_ID);

    // Find pool in remaining_accounts
    let pool_account = remaining_accounts
        .iter()
        .find(|acc| acc.key() == pool_address)
        .ok_or(error::ErrorCode::PoolNotFound)?;

    if pool_account.owner != &crate::constants::RAYDIUM_PROGRAM_ID {
        return Err(error::ErrorCode::InvalidPoolOwner.into());
    }

    parse_raydium_pool_data(pool_account)
}

fn find_meteora_pool(
    _token_a: &Pubkey,
    _token_b: &Pubkey,
    _remaining_accounts: &[AccountInfo],
) -> Result<PoolInfo> {
    Err(error::ErrorCode::PoolNotFound.into())
}

fn find_hardcoded_pool(_token_a: &Pubkey, _token_b: &Pubkey) -> Result<PoolInfo> {
    Err(error::ErrorCode::PoolNotFound.into())
}
