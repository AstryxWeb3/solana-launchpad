use anchor_lang::prelude::*;

use crate::errors::LaunchpadError;
use crate::events::ConfigUpdated;
use crate::state::GlobalConfig;

#[derive(AnchorSerialize, AnchorDeserialize)]
pub struct InitializeParams {
    pub pause_authority: Pubkey,
    pub dev_wallet: Pubkey,
    pub platform_wallet: Pubkey,
    pub dev_fee_bps: u16,
    pub platform_fee_bps: u16,
    pub sell_tax_bps: u16,
    pub presale_platform_fee_bps: u16,
    pub migration_fee_bps: u16,
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(mut)]
    pub admin: Signer<'info>,

    #[account(
        init,
        payer = admin,
        space = 8 + GlobalConfig::INIT_SPACE,
        seeds = [GlobalConfig::SEED],
        bump,
    )]
    pub config: Account<'info, GlobalConfig>,

    pub system_program: Program<'info, System>,
}

pub fn handle_initialize(ctx: Context<Initialize>, params: InitializeParams) -> Result<()> {
    // Validate fee params don't exceed reasonable limits
    require!(
        params.dev_fee_bps <= 500 && params.platform_fee_bps <= 500,
        LaunchpadError::InvalidFeeConfig
    );
    require!(
        params.sell_tax_bps <= 5000, // max 50%
        LaunchpadError::InvalidFeeConfig
    );
    require!(
        params.migration_fee_bps <= 500, // max 5%
        LaunchpadError::InvalidFeeConfig
    );
    require!(
        params.presale_platform_fee_bps <= 500, // max 5%
        LaunchpadError::InvalidFeeConfig
    );

    let config = &mut ctx.accounts.config;
    config.admin = ctx.accounts.admin.key();
    config.pause_authority = params.pause_authority;
    config.dev_wallet = params.dev_wallet;
    config.platform_wallet = params.platform_wallet;
    config.dev_fee_bps = params.dev_fee_bps;
    config.platform_fee_bps = params.platform_fee_bps;
    config.sell_tax_bps = params.sell_tax_bps;
    config.presale_platform_fee_bps = params.presale_platform_fee_bps;
    config.migration_fee_bps = params.migration_fee_bps;
    config.pending_admin = Pubkey::default();
    config.is_paused = false;
    config.bump = ctx.bumps.config;

    emit!(ConfigUpdated {
        admin: ctx.accounts.admin.key(),
        timestamp: Clock::get()?.unix_timestamp,
    });

    Ok(())
}
