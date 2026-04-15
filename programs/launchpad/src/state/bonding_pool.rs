use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct BondingCurvePool {
    /// Pool creator
    pub creator: Pubkey,
    /// Token mint
    pub mint: Pubkey,

    /// Virtual SOL reserves for price calculation
    pub virtual_sol_reserves: u64,
    /// Virtual token reserves for price calculation
    pub virtual_token_reserves: u64,
    /// Actual SOL collected (in lamports)
    pub real_sol_reserves: u64,
    /// Actual tokens remaining in vault
    pub real_token_reserves: u64,
    /// Initial token supply loaded into vault
    pub initial_real_token_reserves: u64,

    /// SOL target for migration (lamports, default: 100 SOL)
    pub migration_target: u64,
    /// Max buy percentage in basis points (100 = 1%)
    pub max_buy_bps: u16,

    /// Accumulated sell tax SOL for buyback treasury (lamports)
    pub buyback_treasury: u64,

    /// Pool has been migrated to Meteora
    pub is_migrated: bool,
    /// Pool-level pause
    pub is_paused: bool,

    /// PDA bump
    pub bump: u8,
    /// SOL vault bump
    pub sol_vault_bump: u8,
    /// Token vault bump
    pub token_vault_bump: u8,
}

impl BondingCurvePool {
    pub const SEED: &'static [u8] = b"bonding_pool";
    pub const SOL_VAULT_SEED: &'static [u8] = b"bonding_sol_vault";
    pub const TOKEN_VAULT_SEED: &'static [u8] = b"bonding_token_vault";
}
