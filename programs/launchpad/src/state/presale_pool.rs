use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct PresalePool {
    /// Pool creator
    pub creator: Pubkey,
    /// Token mint
    pub mint: Pubkey,

    /// SOL target for migration (lamports, 100-10000 SOL)
    pub migration_target: u64,
    /// Total SOL raised so far (lamports)
    pub current_raised: u64,
    /// Total token supply for distribution
    pub total_token_supply: u64,

    /// Max contribution per wallet in basis points (100 = 1%)
    pub max_buy_bps: u16,
    /// Creator pool percentage in basis points (2000 = 20%)
    pub creator_pool_bps: u16,

    /// Presale end time (unix timestamp)
    pub end_time: i64,

    /// Number of unique contributors
    pub num_contributors: u32,

    /// Pool has been migrated to Meteora
    pub is_migrated: bool,

    /// PDA bump
    pub bump: u8,
    /// SOL vault bump
    pub sol_vault_bump: u8,
    /// Token vault bump
    pub token_vault_bump: u8,
}

impl PresalePool {
    pub const SEED: &'static [u8] = b"presale_pool";
    pub const SOL_VAULT_SEED: &'static [u8] = b"presale_sol_vault";
    pub const TOKEN_VAULT_SEED: &'static [u8] = b"presale_token_vault";

    /// Minimum migration target: 100 SOL
    pub const MIN_MIGRATION_TARGET: u64 = 100_000_000_000; // 100 SOL in lamports
    /// Maximum migration target: 10,000 SOL
    pub const MAX_MIGRATION_TARGET: u64 = 10_000_000_000_000; // 10,000 SOL in lamports
}
