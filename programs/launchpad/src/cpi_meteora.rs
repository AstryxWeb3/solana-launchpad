//! Meteora DAMM v2 CPI interface.
//!
//! Manual CPI construction — Meteora doesn't publish a crate,
//! so we construct instructions with raw discriminators + serialized params.
//! Discriminators sourced from the on-chain IDL (cp_amm.json).

use anchor_lang::prelude::*;
use anchor_lang::solana_program::{
    instruction::{AccountMeta, Instruction},
    program::invoke_signed,
};

use crate::errors::LaunchpadError;

/// Meteora DAMM v2 program ID: cpamdpZCGKUy5JxQXB4dcpGPiikHawvSWAd6mEn1sGG
pub static METEORA_PROGRAM_ID: Pubkey = Pubkey::new_from_array([
    9, 45, 33, 53, 101, 122, 21, 156, 43, 135, 212, 182, 106, 112, 219, 142,
    151, 82, 56, 159, 247, 106, 175, 32, 108, 237, 6, 58, 56, 249, 90, 237,
]);

/// Anchor instruction discriminator for `initialize_pool`
const INITIALIZE_POOL_DISC: [u8; 8] = [95, 180, 10, 172, 84, 174, 232, 40];

/// Anchor instruction discriminator for `swap`
/// sha256("global:swap")[..8]
const SWAP_DISC: [u8; 8] = [248, 198, 158, 145, 225, 117, 135, 200];

// ── Initialize Pool CPI ─────────────────────────────────────────────────

/// Accounts required by Meteora's `initialize_pool` instruction.
pub struct InitializePoolAccounts<'info> {
    /// Pool creator (signer)
    pub creator: AccountInfo<'info>,
    /// Transaction payer (signer)
    pub payer: AccountInfo<'info>,
    /// Position NFT mint (signer — keypair generated off-chain)
    pub position_nft_mint: AccountInfo<'info>,
    /// Position NFT token account
    pub position_nft_account: AccountInfo<'info>,
    /// Token A mint (SOL/WSOL)
    pub token_a_mint: AccountInfo<'info>,
    /// Token B mint (project token)
    pub token_b_mint: AccountInfo<'info>,
    /// Pool token A vault
    pub token_a_vault: AccountInfo<'info>,
    /// Pool token B vault
    pub token_b_vault: AccountInfo<'info>,
    /// Payer's token A account (source of token A deposit)
    pub payer_token_a: AccountInfo<'info>,
    /// Payer's token B account (source of token B deposit)
    pub payer_token_b: AccountInfo<'info>,
    /// Pool state account
    pub pool: AccountInfo<'info>,
    /// Pool config (fee config)
    pub pool_config: AccountInfo<'info>,
    /// Position state account
    pub position: AccountInfo<'info>,
    /// Position NFT metadata (Metaplex)
    pub position_nft_metadata: AccountInfo<'info>,
    /// Token program A
    pub token_program_a: AccountInfo<'info>,
    /// Token program B
    pub token_program_b: AccountInfo<'info>,
    /// Associated token program
    pub associated_token_program: AccountInfo<'info>,
    /// System program
    pub system_program: AccountInfo<'info>,
    /// Rent sysvar
    pub rent: AccountInfo<'info>,
    /// Meteora DAMM v2 program
    pub meteora_program: AccountInfo<'info>,
}

/// Parameters for pool initialization.
pub struct InitializePoolParams {
    /// Initial liquidity amount (u128)
    pub liquidity: u128,
    /// Initial sqrt price (u128, Q64.64 fixed-point)
    pub sqrt_price: u128,
    /// Optional activation point (slot number)
    pub activation_point: Option<u64>,
}

/// Call Meteora's `initialize_pool` via CPI.
///
/// `signer_seeds` — PDA seeds if the creator/payer is a PDA.
pub fn cpi_initialize_pool<'info>(
    accounts: &InitializePoolAccounts<'info>,
    params: &InitializePoolParams,
    signer_seeds: &[&[&[u8]]],
) -> Result<()> {
    let mut data = Vec::with_capacity(8 + 16 + 16 + 9);
    data.extend_from_slice(&INITIALIZE_POOL_DISC);
    data.extend_from_slice(&params.liquidity.to_le_bytes());
    data.extend_from_slice(&params.sqrt_price.to_le_bytes());

    // Option<u64> serialization: 1 byte discriminator + 8 bytes if Some
    match params.activation_point {
        Some(ap) => {
            data.push(1);
            data.extend_from_slice(&ap.to_le_bytes());
        }
        None => {
            data.push(0);
        }
    }

    let account_metas = vec![
        AccountMeta::new(accounts.creator.key(), true),
        AccountMeta::new(accounts.payer.key(), true),
        AccountMeta::new(accounts.position_nft_mint.key(), true),
        AccountMeta::new(accounts.position_nft_account.key(), false),
        AccountMeta::new_readonly(accounts.token_a_mint.key(), false),
        AccountMeta::new_readonly(accounts.token_b_mint.key(), false),
        AccountMeta::new(accounts.token_a_vault.key(), false),
        AccountMeta::new(accounts.token_b_vault.key(), false),
        AccountMeta::new(accounts.payer_token_a.key(), false),
        AccountMeta::new(accounts.payer_token_b.key(), false),
        AccountMeta::new(accounts.pool.key(), false),
        AccountMeta::new_readonly(accounts.pool_config.key(), false),
        AccountMeta::new(accounts.position.key(), false),
        AccountMeta::new(accounts.position_nft_metadata.key(), false),
        AccountMeta::new_readonly(accounts.token_program_a.key(), false),
        AccountMeta::new_readonly(accounts.token_program_b.key(), false),
        AccountMeta::new_readonly(accounts.associated_token_program.key(), false),
        AccountMeta::new_readonly(accounts.system_program.key(), false),
        AccountMeta::new_readonly(accounts.rent.key(), false),
    ];

    let ix = Instruction {
        program_id: METEORA_PROGRAM_ID,
        accounts: account_metas,
        data,
    };

    let account_infos = &[
        accounts.creator.clone(),
        accounts.payer.clone(),
        accounts.position_nft_mint.clone(),
        accounts.position_nft_account.clone(),
        accounts.token_a_mint.clone(),
        accounts.token_b_mint.clone(),
        accounts.token_a_vault.clone(),
        accounts.token_b_vault.clone(),
        accounts.payer_token_a.clone(),
        accounts.payer_token_b.clone(),
        accounts.pool.clone(),
        accounts.pool_config.clone(),
        accounts.position.clone(),
        accounts.position_nft_metadata.clone(),
        accounts.token_program_a.clone(),
        accounts.token_program_b.clone(),
        accounts.associated_token_program.clone(),
        accounts.system_program.clone(),
        accounts.rent.clone(),
        accounts.meteora_program.clone(),
    ];

    invoke_signed(&ix, account_infos, signer_seeds)
        .map_err(|_| error!(LaunchpadError::InvalidPoolParams))?;

    Ok(())
}

// ── Swap CPI ────────────────────────────────────────────────────────────

/// Accounts required by Meteora's `swap` instruction.
pub struct SwapAccounts<'info> {
    /// Pool state
    pub pool: AccountInfo<'info>,
    /// Input token vault (pool side)
    pub input_vault: AccountInfo<'info>,
    /// Output token vault (pool side)
    pub output_vault: AccountInfo<'info>,
    /// Input token mint
    pub input_mint: AccountInfo<'info>,
    /// Output token mint
    pub output_mint: AccountInfo<'info>,
    /// User's input token account
    pub user_input_token: AccountInfo<'info>,
    /// User's output token account
    pub user_output_token: AccountInfo<'info>,
    /// User/authority (signer)
    pub user: AccountInfo<'info>,
    /// Protocol fee account
    pub protocol_fee: AccountInfo<'info>,
    /// Input token program
    pub input_token_program: AccountInfo<'info>,
    /// Output token program
    pub output_token_program: AccountInfo<'info>,
    /// Meteora program
    pub meteora_program: AccountInfo<'info>,
}

/// Parameters for swap.
pub struct SwapParams {
    /// Amount of input token
    pub amount_in: u64,
    /// Minimum output token (slippage protection)
    pub minimum_amount_out: u64,
}

/// Call Meteora's `swap` via CPI.
pub fn cpi_swap<'info>(
    accounts: &SwapAccounts<'info>,
    params: &SwapParams,
    signer_seeds: &[&[&[u8]]],
) -> Result<()> {
    let mut data = Vec::with_capacity(8 + 8 + 8);
    data.extend_from_slice(&SWAP_DISC);
    data.extend_from_slice(&params.amount_in.to_le_bytes());
    data.extend_from_slice(&params.minimum_amount_out.to_le_bytes());

    let account_metas = vec![
        AccountMeta::new(accounts.pool.key(), false),
        AccountMeta::new(accounts.input_vault.key(), false),
        AccountMeta::new(accounts.output_vault.key(), false),
        AccountMeta::new_readonly(accounts.input_mint.key(), false),
        AccountMeta::new_readonly(accounts.output_mint.key(), false),
        AccountMeta::new(accounts.user_input_token.key(), false),
        AccountMeta::new(accounts.user_output_token.key(), false),
        AccountMeta::new(accounts.user.key(), true),
        AccountMeta::new(accounts.protocol_fee.key(), false),
        AccountMeta::new_readonly(accounts.input_token_program.key(), false),
        AccountMeta::new_readonly(accounts.output_token_program.key(), false),
    ];

    let ix = Instruction {
        program_id: METEORA_PROGRAM_ID,
        accounts: account_metas,
        data,
    };

    let account_infos = &[
        accounts.pool.clone(),
        accounts.input_vault.clone(),
        accounts.output_vault.clone(),
        accounts.input_mint.clone(),
        accounts.output_mint.clone(),
        accounts.user_input_token.clone(),
        accounts.user_output_token.clone(),
        accounts.user.clone(),
        accounts.protocol_fee.clone(),
        accounts.input_token_program.clone(),
        accounts.output_token_program.clone(),
        accounts.meteora_program.clone(),
    ];

    invoke_signed(&ix, account_infos, signer_seeds)
        .map_err(|_| error!(LaunchpadError::SlippageExceeded))?;

    Ok(())
}

// ── Price helpers ────────────────────────────────────────────────────────

/// Calculate initial sqrt_price for Meteora pool from bonding curve final state.
///
/// sqrt_price = sqrt(sol_reserves / token_reserves) * 2^64 (Q64.64 format)
///
/// Approach: sqrt_price = isqrt(sol) * 2^64 / isqrt(tokens)
/// This avoids the impossible `shl(128)` on u128.
pub fn calculate_init_sqrt_price(
    sol_amount: u64,
    token_amount: u64,
) -> Result<u128> {
    require!(token_amount > 0, LaunchpadError::DivisionByZero);
    require!(sol_amount > 0, LaunchpadError::ZeroAmount);

    // sqrt_price = sqrt(sol / tokens) * 2^64
    // Split: = (sqrt(sol) / sqrt(tokens)) * 2^64
    // = sqrt(sol) * 2^64 / sqrt(tokens)

    let sqrt_sol = isqrt_u128(sol_amount as u128);
    let sqrt_tokens = isqrt_u128(token_amount as u128);

    require!(sqrt_tokens > 0, LaunchpadError::DivisionByZero);

    // sqrt_sol * 2^64 — this fits in u128 since sqrt_sol <= 2^32 and 2^32 * 2^64 = 2^96 < 2^128
    let numerator = sqrt_sol
        .checked_mul(1u128 << 64)
        .ok_or(LaunchpadError::MathOverflow)?;

    let sqrt_price = numerator
        .checked_div(sqrt_tokens)
        .ok_or(LaunchpadError::DivisionByZero)?;

    require!(sqrt_price > 0, LaunchpadError::ZeroAmount);

    Ok(sqrt_price)
}

/// Integer square root for u128.
fn isqrt_u128(n: u128) -> u128 {
    if n == 0 {
        return 0;
    }
    let mut x = n;
    let mut y = (x + 1) / 2;
    while y < x {
        x = y;
        y = (x + n / x) / 2;
    }
    x
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_isqrt() {
        assert_eq!(isqrt_u128(0), 0);
        assert_eq!(isqrt_u128(1), 1);
        assert_eq!(isqrt_u128(4), 2);
        assert_eq!(isqrt_u128(9), 3);
        assert_eq!(isqrt_u128(100), 10);
        assert_eq!(isqrt_u128(u128::MAX), 18446744073709551615); // 2^64 - 1
    }

    #[test]
    fn test_sqrt_price_calculation() {
        // 100 SOL with 1B tokens (6 decimals)
        let sqrt_price = calculate_init_sqrt_price(
            100_000_000_000,       // 100 SOL in lamports
            1_000_000_000_000_000, // 1B tokens (6 decimals)
        )
        .unwrap();

        assert!(sqrt_price > 0);
        // sqrt(100e9 / 1e15) * 2^64 = sqrt(1e-4) * 2^64 = 0.01 * 2^64 ≈ 1.84e17
        // Our integer sqrt approach gives approximate value
        assert!(sqrt_price > 1_000_000_000_000_000); // > 1e15 (sanity)
    }

    #[test]
    fn test_sqrt_price_equal_amounts() {
        // 1:1 ratio → sqrt(1) * 2^64 = 2^64
        let sqrt_price = calculate_init_sqrt_price(1_000_000_000, 1_000_000_000).unwrap();
        // Should be close to 2^64 = 18446744073709551616
        assert!(sqrt_price > 18_000_000_000_000_000_000u128);
        assert!(sqrt_price < 19_000_000_000_000_000_000u128);
    }
}
