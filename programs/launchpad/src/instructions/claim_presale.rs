use anchor_lang::prelude::*;
use anchor_spl::token::{self, Token, TokenAccount, Transfer};

use crate::errors::LaunchpadError;
use crate::events::PresaleClaimed;
use crate::state::{PresalePool, UserPosition};

#[derive(Accounts)]
pub struct ClaimPresale<'info> {
    pub user: Signer<'info>,

    #[account(
        seeds = [PresalePool::SEED, pool.mint.as_ref()],
        bump = pool.bump,
        constraint = pool.is_migrated @ LaunchpadError::NotMigrated,
    )]
    pub pool: Account<'info, PresalePool>,

    #[account(
        mut,
        seeds = [UserPosition::SEED, pool.key().as_ref(), user.key().as_ref()],
        bump = user_position.bump,
        constraint = user_position.user == user.key(),
        constraint = !user_position.tokens_claimed @ LaunchpadError::AlreadyClaimed,
    )]
    pub user_position: Account<'info, UserPosition>,

    /// Token vault
    #[account(
        mut,
        token::mint = pool.mint,
        token::authority = pool,
        seeds = [PresalePool::TOKEN_VAULT_SEED, pool.mint.as_ref()],
        bump = pool.token_vault_bump,
    )]
    pub token_vault: Account<'info, TokenAccount>,

    /// User's token account
    #[account(
        mut,
        token::mint = pool.mint,
        token::authority = user,
    )]
    pub user_token_account: Account<'info, TokenAccount>,

    pub token_program: Program<'info, Token>,
}

pub fn handle_claim_presale(ctx: Context<ClaimPresale>) -> Result<()> {
    let pool = &ctx.accounts.pool;
    let position = &ctx.accounts.user_position;

    // ── CHECKS ──────────────────────────────────────────────────────

    // M-6: Guard against division by zero
    require!(pool.current_raised > 0, LaunchpadError::ZeroAmount);

    // Calculate user's token share:
    // Distributable tokens = total_supply - creator_pool - liquidity_pool
    // creator_pool = total_supply * creator_pool_bps / 10_000
    // liquidity_pool = total_supply * 2000 / 10_000 (20% for Meteora)
    // Remaining = total_supply * (10_000 - creator_pool_bps - 2000) / 10_000

    let distributable_bps: u64 = 10_000u64
        .checked_sub(pool.creator_pool_bps as u64)
        .ok_or(LaunchpadError::MathUnderflow)?
        .checked_sub(2000) // 20% for liquidity
        .ok_or(LaunchpadError::MathUnderflow)?;

    let distributable_tokens: u128 = (pool.total_token_supply as u128)
        .checked_mul(distributable_bps as u128)
        .ok_or(LaunchpadError::MathOverflow)?
        .checked_div(10_000u128)
        .ok_or(LaunchpadError::DivisionByZero)?;

    // User's share = distributable_tokens * user_sol / total_raised
    let user_tokens: u128 = distributable_tokens
        .checked_mul(position.sol_contributed as u128)
        .ok_or(LaunchpadError::MathOverflow)?
        .checked_div(pool.current_raised as u128)
        .ok_or(LaunchpadError::DivisionByZero)?;

    let user_tokens = u64::try_from(user_tokens).map_err(|_| LaunchpadError::CastOverflow)?;
    require!(user_tokens > 0, LaunchpadError::ZeroAmount);

    // ── EFFECTS ─────────────────────────────────────────────────────

    let position = &mut ctx.accounts.user_position;
    position.tokens_claimed = true;

    // ── INTERACTIONS ────────────────────────────────────────────────

    let mint_key = pool.mint;
    let signer_seeds: &[&[&[u8]]] = &[&[
        PresalePool::SEED,
        mint_key.as_ref(),
        &[pool.bump],
    ]];

    token::transfer(
        CpiContext::new_with_signer(
            ctx.accounts.token_program.to_account_info(),
            Transfer {
                from: ctx.accounts.token_vault.to_account_info(),
                to: ctx.accounts.user_token_account.to_account_info(),
                authority: ctx.accounts.pool.to_account_info(),
            },
            signer_seeds,
        ),
        user_tokens,
    )?;

    emit!(PresaleClaimed {
        pool: ctx.accounts.pool.key(),
        user: ctx.accounts.user.key(),
        token_amount: user_tokens,
        timestamp: Clock::get()?.unix_timestamp,
    });

    Ok(())
}
