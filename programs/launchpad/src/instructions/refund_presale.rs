use anchor_lang::prelude::*;

use crate::errors::LaunchpadError;
use crate::events::PresaleRefunded;
use crate::state::{PresalePool, UserPosition};

#[derive(Accounts)]
pub struct RefundPresale<'info> {
    #[account(mut)]
    pub user: Signer<'info>,

    #[account(
        mut,
        seeds = [PresalePool::SEED, pool.mint.as_ref()],
        bump = pool.bump,
        constraint = !pool.is_migrated @ LaunchpadError::AlreadyMigrated,
    )]
    pub pool: Account<'info, PresalePool>,

    #[account(
        mut,
        seeds = [UserPosition::SEED, pool.key().as_ref(), user.key().as_ref()],
        bump = user_position.bump,
        constraint = user_position.user == user.key(),
        constraint = !user_position.refund_claimed @ LaunchpadError::AlreadyRefunded,
    )]
    pub user_position: Account<'info, UserPosition>,

    /// SOL vault
    #[account(
        mut,
        seeds = [PresalePool::SOL_VAULT_SEED, pool.mint.as_ref()],
        bump = pool.sol_vault_bump,
    )]
    pub sol_vault: SystemAccount<'info>,

    pub system_program: Program<'info, System>,
}

pub fn handle_refund_presale(ctx: Context<RefundPresale>) -> Result<()> {
    let pool = &ctx.accounts.pool;
    let position = &ctx.accounts.user_position;

    // ── CHECKS ──────────────────────────────────────────────────────

    let now = Clock::get()?.unix_timestamp;
    require!(now > pool.end_time, LaunchpadError::PresaleNotEnded);
    require!(
        pool.current_raised < pool.migration_target,
        LaunchpadError::TargetReached
    );

    let refund_amount = position.sol_contributed;
    require!(refund_amount > 0, LaunchpadError::ZeroAmount);

    // ── EFFECTS ─────────────────────────────────────────────────────

    let mint_key = pool.mint;
    let sol_vault_bump = pool.sol_vault_bump;

    // M-4: Decrease current_raised on refund
    let pool = &mut ctx.accounts.pool;
    pool.current_raised = pool.current_raised
        .checked_sub(refund_amount)
        .ok_or(LaunchpadError::MathUnderflow)?;

    let position = &mut ctx.accounts.user_position;
    position.refund_claimed = true;

    // ── INTERACTIONS ────────────────────────────────────────────────

    let sol_vault_signer_seeds: &[&[&[u8]]] = &[&[
        PresalePool::SOL_VAULT_SEED,
        mint_key.as_ref(),
        &[sol_vault_bump],
    ]];

    anchor_lang::system_program::transfer(
        CpiContext::new_with_signer(
            ctx.accounts.system_program.to_account_info(),
            anchor_lang::system_program::Transfer {
                from: ctx.accounts.sol_vault.to_account_info(),
                to: ctx.accounts.user.to_account_info(),
            },
            sol_vault_signer_seeds,
        ),
        refund_amount,
    )?;

    emit!(PresaleRefunded {
        pool: ctx.accounts.pool.key(),
        user: ctx.accounts.user.key(),
        sol_amount: refund_amount,
        timestamp: now,
    });

    Ok(())
}
