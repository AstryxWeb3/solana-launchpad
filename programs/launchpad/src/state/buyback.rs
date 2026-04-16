use anchor_lang::prelude::*;

/// Buyback modes for presale
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, PartialEq, Eq, InitSpace)]
pub enum BuybackMode {
    /// Swap SOL → token, then burn tokens (deflationary)
    Burn,
    /// Swap SOL → token, add as LP to Meteora pool
    AddLiquidity,
}

#[account]
#[derive(InitSpace)]
pub struct BuybackState {
    /// Associated pool pubkey
    pub pool: Pubkey,
    /// Associated token mint
    pub mint: Pubkey,
    /// Meteora DAMM pool created during migration — validated on every buyback
    pub meteora_pool: Pubkey,

    /// SOL remaining in buyback treasury (lamports)
    pub treasury_balance: u64,
    /// Last slot a buyback was executed
    pub last_buyback_slot: u64,
    /// Total SOL spent on buybacks
    pub total_sol_spent: u64,
    /// Total tokens bought back
    pub total_tokens_bought: u64,
    /// Total tokens burned
    pub total_tokens_burned: u64,
    /// Total tokens added as liquidity
    pub total_tokens_lp: u64,

    /// Pool type (0 = bonding, 1 = presale)
    pub pool_type: u8,

    /// PDA bump
    pub bump: u8,
}

impl BuybackState {
    pub const SEED: &'static [u8] = b"buyback";

    /// Minimum slots between buybacks (~4 seconds)
    pub const MIN_BUYBACK_INTERVAL: u64 = 10;

    /// Bonding curve: 20% of pool each buyback cycle
    pub const BONDING_BUYBACK_BPS: u64 = 2000;
    /// Presale: 60% of pool
    pub const PRESALE_BUYBACK_BPS: u64 = 6000;
}
