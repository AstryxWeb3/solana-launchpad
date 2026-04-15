use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct GlobalConfig {
    /// Full admin authority
    pub admin: Pubkey,
    /// Can pause/unpause only
    pub pause_authority: Pubkey,
    /// Receives dev portion of fees
    pub dev_wallet: Pubkey,
    /// Receives platform portion of fees
    pub platform_wallet: Pubkey,

    /// Dev fee in basis points (default: 50 = 0.5%)
    pub dev_fee_bps: u16,
    /// Platform fee in basis points (default: 50 = 0.5%)
    pub platform_fee_bps: u16,
    /// Sell tax in basis points (default: 2400 = 24%)
    pub sell_tax_bps: u16,
    /// Presale buy fee in basis points (default: 100 = 1%)
    pub presale_platform_fee_bps: u16,
    /// Migration fee in basis points (default: 100 = 1%)
    pub migration_fee_bps: u16,

    /// Pending admin for two-step transfer (Pubkey::default() = none)
    pub pending_admin: Pubkey,

    /// Global pause flag
    pub is_paused: bool,

    /// PDA bump
    pub bump: u8,
}

impl GlobalConfig {
    pub const SEED: &'static [u8] = b"config";
}
