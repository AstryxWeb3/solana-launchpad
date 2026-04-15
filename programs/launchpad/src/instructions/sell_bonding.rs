use anchor_lang::prelude::*;
use anchor_spl::token::{self, Token, TokenAccount, Transfer};

use crate::errors::LaunchpadError;
use crate::events::TokensSold;
use crate::math::{bonding_curve, fees};
use crate::state::{BondingCurvePool, GlobalConfig};

#[derive(Accounts)]
pub struct SellBonding<'info> {
    #[account(mut)]
    pub seller: Signer<'info>,

    #[account(
        seeds = [GlobalConfig::SEED],
        bump = config.bump,
        constraint = !config.is_paused @ LaunchpadError::PlatformPaused,
    )]
    pub config: Account<'info, GlobalConfig>,

    #[account(
        mut,
        seeds = [BondingCurvePool::SEED, pool.mint.as_ref()],
        bump = pool.bump,
        constraint = !pool.is_migrated @ LaunchpadError::AlreadyMigrated,
        constraint = !pool.is_paused @ LaunchpadError::PoolPaused,
    )]
    pub pool: Account<'info, BondingCurvePool>,

    /// SOL vault PDA
    #[account(
        mut,
        seeds = [BondingCurvePool::SOL_VAULT_SEED, pool.mint.as_ref()],
        bump = pool.sol_vault_bump,
    )]
    pub sol_vault: SystemAccount<'info>,

    /// Token vault
    #[account(
        mut,
        token::mint = pool.mint,
        token::authority = pool,
        seeds = [BondingCurvePool::TOKEN_VAULT_SEED, pool.mint.as_ref()],
        bump = pool.token_vault_bump,
    )]
    pub token_vault: Account<'info, TokenAccount>,

    /// Seller's token account
    #[account(
        mut,
        token::mint = pool.mint,
        token::authority = seller,
    )]
    pub seller_token_account: Account<'info, TokenAccount>,

    /// Platform wallet receives platform fee
    /// CHECK: Validated against config
    #[account(
        mut,
        constraint = platform_wallet.key() == config.platform_wallet @ LaunchpadError::InvalidFeeConfig,
    )]
    pub platform_wallet: SystemAccount<'info>,

    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
}

pub fn handle_sell_bonding(ctx: Context<SellBonding>, token_amount: u64, min_sol_out: u64) -> Result<()> {
    require!(token_amount > 0, LaunchpadError::ZeroAmount);

    let pool = &ctx.accounts.pool;
    let config = &ctx.accounts.config;

    // ── CHECKS ──────────────────────────────────────────────────────

    // Calculate gross SOL out from bonding curve
    let gross_sol_out = bonding_curve::calculate_sell_amount(
        pool.virtual_sol_reserves,
        pool.virtual_token_reserves,
        token_amount,
    )?;

    require!(gross_sol_out > 0, LaunchpadError::ZeroAmount);

    // Calculate sell fees: 1% platform + 24% sell tax
    let sell_fees = fees::calculate_sell_fees(
        gross_sol_out,
        config.platform_fee_bps,  // 1% platform on sell
        config.sell_tax_bps,      // 24% sell tax
    )?;

    // H-1: Slippage protection
    require!(sell_fees.net_amount >= min_sol_out, LaunchpadError::SlippageExceeded);

    // Ensure enough SOL in vault
    require!(
        gross_sol_out <= pool.real_sol_reserves,
        LaunchpadError::InsufficientSolReserves
    );

    // H-6: Ensure vault stays above rent-exempt minimum after transfer
    let total_out = sell_fees.net_amount
        .checked_add(sell_fees.platform_fee)
        .ok_or(LaunchpadError::MathOverflow)?;
    let vault_lamports = ctx.accounts.sol_vault.to_account_info().lamports();
    let rent_exempt_min = Rent::get()?.minimum_balance(0);
    require!(
        vault_lamports.checked_sub(total_out).unwrap_or(0) >= rent_exempt_min,
        LaunchpadError::InsufficientSolReserves
    );

    // ── EFFECTS ─────────────────────────────────────────────────────

    let pool = &mut ctx.accounts.pool;

    pool.virtual_sol_reserves = pool
        .virtual_sol_reserves
        .checked_sub(gross_sol_out)
        .ok_or(LaunchpadError::MathUnderflow)?;

    pool.virtual_token_reserves = pool
        .virtual_token_reserves
        .checked_add(token_amount)
        .ok_or(LaunchpadError::MathOverflow)?;

    pool.real_sol_reserves = pool
        .real_sol_reserves
        .checked_sub(gross_sol_out)
        .ok_or(LaunchpadError::MathUnderflow)?;

    pool.real_token_reserves = pool
        .real_token_reserves
        .checked_add(token_amount)
        .ok_or(LaunchpadError::MathOverflow)?;

    // Sell tax accumulates in buyback treasury
    pool.buyback_treasury = pool
        .buyback_treasury
        .checked_add(sell_fees.sell_tax)
        .ok_or(LaunchpadError::MathOverflow)?;

    // ── INTERACTIONS ────────────────────────────────────────────────

    // Transfer tokens: seller → token_vault
    token::transfer(
        CpiContext::new(
            ctx.accounts.token_program.to_account_info(),
            Transfer {
                from: ctx.accounts.seller_token_account.to_account_info(),
                to: ctx.accounts.token_vault.to_account_info(),
                authority: ctx.accounts.seller.to_account_info(),
            },
        ),
        token_amount,
    )?;

    // Transfer SOL from sol_vault PDA (system_program::transfer with PDA signer)
    let mint_key = pool.mint;
    let sol_vault_signer_seeds: &[&[&[u8]]] = &[&[
        BondingCurvePool::SOL_VAULT_SEED,
        mint_key.as_ref(),
        &[pool.sol_vault_bump],
    ]];

    // Transfer net SOL to seller
    if sell_fees.net_amount > 0 {
        anchor_lang::system_program::transfer(
            CpiContext::new_with_signer(
                ctx.accounts.system_program.to_account_info(),
                anchor_lang::system_program::Transfer {
                    from: ctx.accounts.sol_vault.to_account_info(),
                    to: ctx.accounts.seller.to_account_info(),
                },
                sol_vault_signer_seeds,
            ),
            sell_fees.net_amount,
        )?;
    }

    // Transfer platform fee to platform wallet
    if sell_fees.platform_fee > 0 {
        anchor_lang::system_program::transfer(
            CpiContext::new_with_signer(
                ctx.accounts.system_program.to_account_info(),
                anchor_lang::system_program::Transfer {
                    from: ctx.accounts.sol_vault.to_account_info(),
                    to: ctx.accounts.platform_wallet.to_account_info(),
                },
                sol_vault_signer_seeds,
            ),
            sell_fees.platform_fee,
        )?;
    }

    // Sell tax stays in sol_vault (tracked by pool.buyback_treasury)
    // It will be moved during migration

    // ── EVENTS ──────────────────────────────────────────────────────

    let new_price = bonding_curve::calculate_price(
        pool.virtual_sol_reserves,
        pool.virtual_token_reserves,
    )?;

    emit!(TokensSold {
        pool: ctx.accounts.pool.key(),
        seller: ctx.accounts.seller.key(),
        token_amount,
        sol_amount: sell_fees.net_amount,
        platform_fee: sell_fees.platform_fee,
        sell_tax: sell_fees.sell_tax,
        new_price,
        timestamp: Clock::get()?.unix_timestamp,
    });

    Ok(())
}
