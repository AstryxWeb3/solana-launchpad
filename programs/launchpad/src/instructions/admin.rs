use anchor_lang::prelude::*;

use crate::errors::LaunchpadError;
use crate::events::ConfigUpdated;
use crate::state::GlobalConfig;

// ── Update Config (fees, wallets — NOT admin) ───────────────────────────

#[derive(AnchorSerialize, AnchorDeserialize)]
pub struct UpdateConfigParams {
    pub new_pause_authority: Option<Pubkey>,
    pub new_dev_wallet: Option<Pubkey>,
    pub new_platform_wallet: Option<Pubkey>,
    pub new_dev_fee_bps: Option<u16>,
    pub new_platform_fee_bps: Option<u16>,
    pub new_sell_tax_bps: Option<u16>,
    pub new_presale_platform_fee_bps: Option<u16>,
    pub new_migration_fee_bps: Option<u16>,
}

#[derive(Accounts)]
pub struct UpdateConfig<'info> {
    #[account(
        constraint = config.admin == admin.key() @ LaunchpadError::UnauthorizedAdmin
    )]
    pub admin: Signer<'info>,

    #[account(
        mut,
        seeds = [GlobalConfig::SEED],
        bump = config.bump,
    )]
    pub config: Account<'info, GlobalConfig>,
}

pub fn handle_update_config(
    ctx: Context<UpdateConfig>,
    params: UpdateConfigParams,
) -> Result<()> {
    let config = &mut ctx.accounts.config;

    if let Some(v) = params.new_pause_authority {
        config.pause_authority = v;
    }
    if let Some(v) = params.new_dev_wallet {
        config.dev_wallet = v;
    }
    if let Some(v) = params.new_platform_wallet {
        config.platform_wallet = v;
    }
    if let Some(v) = params.new_dev_fee_bps {
        require!(v <= 500, LaunchpadError::InvalidFeeConfig);
        config.dev_fee_bps = v;
    }
    if let Some(v) = params.new_platform_fee_bps {
        require!(v <= 500, LaunchpadError::InvalidFeeConfig);
        config.platform_fee_bps = v;
    }
    if let Some(v) = params.new_sell_tax_bps {
        require!(v <= 5000, LaunchpadError::InvalidFeeConfig);
        config.sell_tax_bps = v;
    }
    if let Some(v) = params.new_presale_platform_fee_bps {
        require!(v <= 500, LaunchpadError::InvalidFeeConfig);
        config.presale_platform_fee_bps = v;
    }
    if let Some(v) = params.new_migration_fee_bps {
        require!(v <= 500, LaunchpadError::InvalidFeeConfig);
        config.migration_fee_bps = v;
    }

    emit!(ConfigUpdated {
        admin: ctx.accounts.admin.key(),
        timestamp: Clock::get()?.unix_timestamp,
    });

    Ok(())
}

// ── Two-Step Admin Transfer ─────────────────────────────────────────────

#[derive(Accounts)]
pub struct ProposeAdmin<'info> {
    #[account(
        constraint = config.admin == admin.key() @ LaunchpadError::UnauthorizedAdmin
    )]
    pub admin: Signer<'info>,

    #[account(
        mut,
        seeds = [GlobalConfig::SEED],
        bump = config.bump,
    )]
    pub config: Account<'info, GlobalConfig>,
}

pub fn handle_propose_admin(ctx: Context<ProposeAdmin>, new_admin: Pubkey) -> Result<()> {
    ctx.accounts.config.pending_admin = new_admin;
    msg!("Admin transfer proposed to {}", new_admin);
    Ok(())
}

#[derive(Accounts)]
pub struct AcceptAdmin<'info> {
    #[account(
        constraint = config.pending_admin == new_admin.key() @ LaunchpadError::UnauthorizedAdmin
    )]
    pub new_admin: Signer<'info>,

    #[account(
        mut,
        seeds = [GlobalConfig::SEED],
        bump = config.bump,
    )]
    pub config: Account<'info, GlobalConfig>,
}

pub fn handle_accept_admin(ctx: Context<AcceptAdmin>) -> Result<()> {
    let config = &mut ctx.accounts.config;
    config.admin = config.pending_admin;
    config.pending_admin = Pubkey::default();

    emit!(ConfigUpdated {
        admin: config.admin,
        timestamp: Clock::get()?.unix_timestamp,
    });

    Ok(())
}

// ── Pause / Unpause ─────────────────────────────────────────────────────

#[derive(Accounts)]
pub struct Pause<'info> {
    #[account(
        constraint = (
            config.admin == authority.key() || config.pause_authority == authority.key()
        ) @ LaunchpadError::UnauthorizedPauseAuthority
    )]
    pub authority: Signer<'info>,

    #[account(
        mut,
        seeds = [GlobalConfig::SEED],
        bump = config.bump,
    )]
    pub config: Account<'info, GlobalConfig>,
}

pub fn handle_pause(ctx: Context<Pause>) -> Result<()> {
    ctx.accounts.config.is_paused = true;
    msg!("Platform paused by {}", ctx.accounts.authority.key());
    Ok(())
}

#[derive(Accounts)]
pub struct Unpause<'info> {
    #[account(
        constraint = config.admin == authority.key() @ LaunchpadError::UnauthorizedAdmin
    )]
    pub authority: Signer<'info>,

    #[account(
        mut,
        seeds = [GlobalConfig::SEED],
        bump = config.bump,
    )]
    pub config: Account<'info, GlobalConfig>,
}

pub fn handle_unpause(ctx: Context<Unpause>) -> Result<()> {
    ctx.accounts.config.is_paused = false;
    msg!("Platform unpaused by {}", ctx.accounts.authority.key());
    Ok(())
}
