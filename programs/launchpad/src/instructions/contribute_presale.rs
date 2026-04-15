use anchor_lang::prelude::*;
use anchor_lang::system_program;

use crate::errors::LaunchpadError;
use crate::events::PresaleContribution;
use crate::math::fees;
use crate::state::{GlobalConfig, PresalePool, UserPosition};

#[derive(Accounts)]
pub struct ContributePresale<'info> {
    #[account(mut)]
    pub contributor: Signer<'info>,

    #[account(
        seeds = [GlobalConfig::SEED],
        bump = config.bump,
        constraint = !config.is_paused @ LaunchpadError::PlatformPaused,
    )]
    pub config: Account<'info, GlobalConfig>,

    #[account(
        mut,
        seeds = [PresalePool::SEED, pool.mint.as_ref()],
        bump = pool.bump,
        constraint = !pool.is_migrated @ LaunchpadError::AlreadyMigrated,
    )]
    pub pool: Account<'info, PresalePool>,

    /// SOL vault
    #[account(
        mut,
        seeds = [PresalePool::SOL_VAULT_SEED, pool.mint.as_ref()],
        bump = pool.sol_vault_bump,
    )]
    pub sol_vault: SystemAccount<'info>,

    /// User's position for this pool (init-if-needed for first contribution)
    #[account(
        init_if_needed,
        payer = contributor,
        space = 8 + UserPosition::INIT_SPACE,
        seeds = [UserPosition::SEED, pool.key().as_ref(), contributor.key().as_ref()],
        bump,
    )]
    pub user_position: Account<'info, UserPosition>,

    /// Platform wallet for presale fee
    /// CHECK: Validated against config
    #[account(
        mut,
        constraint = platform_wallet.key() == config.platform_wallet @ LaunchpadError::InvalidFeeConfig,
    )]
    pub platform_wallet: SystemAccount<'info>,

    pub system_program: Program<'info, System>,
}

pub fn handle_contribute_presale(ctx: Context<ContributePresale>, sol_amount: u64) -> Result<()> {
    require!(sol_amount > 0, LaunchpadError::ZeroAmount);

    let pool = &ctx.accounts.pool;

    // ── CHECKS ──────────────────────────────────────────────────────

    // Check presale hasn't ended
    let now = Clock::get()?.unix_timestamp;
    require!(now < pool.end_time, LaunchpadError::PresaleEnded);

    // Calculate presale fee (1% platform)
    let (platform_fee, net_amount) =
        fees::calculate_presale_fee(sol_amount, ctx.accounts.config.presale_platform_fee_bps)?;

    // Check max contribution (1% of migration target)
    let max_contribution: u128 = (pool.migration_target as u128)
        .checked_mul(pool.max_buy_bps as u128)
        .ok_or(LaunchpadError::MathOverflow)?
        .checked_div(10_000u128)
        .ok_or(LaunchpadError::DivisionByZero)?;

    let max_contribution =
        u64::try_from(max_contribution).map_err(|_| LaunchpadError::CastOverflow)?;

    let new_total = ctx
        .accounts
        .user_position
        .sol_contributed
        .checked_add(net_amount)
        .ok_or(LaunchpadError::MathOverflow)?;

    require!(
        new_total <= max_contribution,
        LaunchpadError::ExceedsMaxContribution
    );

    // ── PRE-CAPTURE ───────────────────────────────────────────────
    let pool_key = ctx.accounts.pool.key();
    let contributor_key = ctx.accounts.contributor.key();
    let is_new = ctx.accounts.user_position.sol_contributed == 0;

    // ── EFFECTS ─────────────────────────────────────────────────────

    // Update user position — only set identity fields on first init (C-5 fix)
    if is_new {
        ctx.accounts.user_position.user = contributor_key;
        ctx.accounts.user_position.pool = pool_key;
        ctx.accounts.user_position.tokens_claimed = false;
        ctx.accounts.user_position.refund_claimed = false;
        ctx.accounts.user_position.bump = ctx.bumps.user_position;
    }
    ctx.accounts.user_position.sol_contributed = new_total;

    // Update pool
    ctx.accounts.pool.current_raised = ctx.accounts.pool
        .current_raised
        .checked_add(net_amount)
        .ok_or(LaunchpadError::MathOverflow)?;

    if is_new {
        ctx.accounts.pool.num_contributors = ctx.accounts.pool
            .num_contributors
            .checked_add(1)
            .ok_or(LaunchpadError::MathOverflow)?;
    }

    let total_raised = ctx.accounts.pool.current_raised;

    // ── INTERACTIONS ────────────────────────────────────────────────

    // Transfer net SOL to vault
    system_program::transfer(
        CpiContext::new(
            ctx.accounts.system_program.to_account_info(),
            system_program::Transfer {
                from: ctx.accounts.contributor.to_account_info(),
                to: ctx.accounts.sol_vault.to_account_info(),
            },
        ),
        net_amount,
    )?;

    // Transfer platform fee
    if platform_fee > 0 {
        system_program::transfer(
            CpiContext::new(
                ctx.accounts.system_program.to_account_info(),
                system_program::Transfer {
                    from: ctx.accounts.contributor.to_account_info(),
                    to: ctx.accounts.platform_wallet.to_account_info(),
                },
            ),
            platform_fee,
        )?;
    }

    // ── EVENTS ──────────────────────────────────────────────────────

    emit!(PresaleContribution {
        pool: pool_key,
        contributor: contributor_key,
        sol_amount: net_amount,
        total_raised,
        timestamp: now,
    });

    Ok(())
}
