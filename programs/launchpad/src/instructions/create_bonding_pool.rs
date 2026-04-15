use anchor_lang::prelude::*;
use anchor_spl::token::{self, Mint, MintTo, SetAuthority, Token, TokenAccount};
use anchor_spl::token::spl_token::instruction::AuthorityType;

use crate::errors::LaunchpadError;
use crate::events::PoolCreated;
use crate::state::{BondingCurvePool, GlobalConfig};

/// Default initial virtual SOL reserves: 30 SOL (pump.fun standard)
pub const DEFAULT_VIRTUAL_SOL: u64 = 30_000_000_000;
/// Default total token supply: 1 billion (6 decimals)
pub const DEFAULT_TOKEN_SUPPLY: u64 = 1_000_000_000_000_000;
/// Default migration target: 100 SOL
pub const DEFAULT_MIGRATION_TARGET: u64 = 100_000_000_000;

#[derive(AnchorSerialize, AnchorDeserialize)]
pub struct CreateBondingPoolParams {
    /// Token name (for mint metadata, not stored on-chain in this program)
    pub virtual_sol_reserves: Option<u64>,
    pub virtual_token_reserves: Option<u64>,
    pub token_supply: Option<u64>,
    pub migration_target: Option<u64>,
}

#[derive(Accounts)]
pub struct CreateBondingPool<'info> {
    #[account(mut)]
    pub creator: Signer<'info>,

    #[account(
        seeds = [GlobalConfig::SEED],
        bump = config.bump,
        constraint = !config.is_paused @ LaunchpadError::PlatformPaused,
    )]
    pub config: Box<Account<'info, GlobalConfig>>,

    /// Token mint — creator must be mint authority initially
    #[account(
        mut,
        constraint = mint.decimals == 6,
        constraint = mint.supply == 0,
        constraint = mint.mint_authority.contains(&creator.key()),
    )]
    pub mint: Box<Account<'info, Mint>>,

    #[account(
        init,
        payer = creator,
        space = 8 + BondingCurvePool::INIT_SPACE,
        seeds = [BondingCurvePool::SEED, mint.key().as_ref()],
        bump,
    )]
    pub pool: Box<Account<'info, BondingCurvePool>>,

    /// SOL vault PDA holding native SOL lamports.
    #[account(
        mut,
        seeds = [BondingCurvePool::SOL_VAULT_SEED, mint.key().as_ref()],
        bump,
    )]
    pub sol_vault: SystemAccount<'info>,

    /// Token vault holding the token supply
    #[account(
        init,
        payer = creator,
        token::mint = mint,
        token::authority = pool,
        seeds = [BondingCurvePool::TOKEN_VAULT_SEED, mint.key().as_ref()],
        bump,
    )]
    pub token_vault: Box<Account<'info, TokenAccount>>,

    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
}

pub fn handle_create_bonding_pool(
    ctx: Context<CreateBondingPool>,
    params: CreateBondingPoolParams,
) -> Result<()> {
    let token_supply = params.token_supply.unwrap_or(DEFAULT_TOKEN_SUPPLY);
    let virtual_sol = params.virtual_sol_reserves.unwrap_or(DEFAULT_VIRTUAL_SOL);
    let virtual_tokens = params.virtual_token_reserves.unwrap_or(token_supply);
    let migration_target = params.migration_target.unwrap_or(DEFAULT_MIGRATION_TARGET);

    // C-2 FIX: Validate parameter bounds
    require!(token_supply >= 1_000_000 && token_supply <= 1_000_000_000_000_000_000, LaunchpadError::InvalidPoolParams); // 1M..1e18
    require!(virtual_sol >= 1_000_000_000 && virtual_sol <= 1_000_000_000_000, LaunchpadError::InvalidPoolParams); // 1..1000 SOL
    require!(virtual_tokens >= 1_000_000 && virtual_tokens <= token_supply, LaunchpadError::InvalidPoolParams);
    require!(migration_target >= 10_000_000_000 && migration_target <= 100_000_000_000_000, LaunchpadError::InvalidPoolParams); // 10..100k SOL

    // Mint entire supply to token vault
    let cpi_accounts = MintTo {
        mint: ctx.accounts.mint.to_account_info(),
        to: ctx.accounts.token_vault.to_account_info(),
        authority: ctx.accounts.creator.to_account_info(),
    };
    let cpi_ctx = CpiContext::new(ctx.accounts.token_program.to_account_info(), cpi_accounts);
    token::mint_to(cpi_ctx, token_supply)?;

    // Transfer mint authority to pool PDA (no more minting possible)
    let cpi_accounts = SetAuthority {
        current_authority: ctx.accounts.creator.to_account_info(),
        account_or_mint: ctx.accounts.mint.to_account_info(),
    };
    let cpi_ctx = CpiContext::new(ctx.accounts.token_program.to_account_info(), cpi_accounts);
    token::set_authority(cpi_ctx, AuthorityType::MintTokens, None)?;

    // Initialize pool state
    let pool = &mut ctx.accounts.pool;
    pool.creator = ctx.accounts.creator.key();
    pool.mint = ctx.accounts.mint.key();
    pool.virtual_sol_reserves = virtual_sol;
    pool.virtual_token_reserves = virtual_tokens;
    pool.real_sol_reserves = 0;
    pool.real_token_reserves = token_supply;
    pool.initial_real_token_reserves = token_supply;
    pool.migration_target = migration_target;
    pool.max_buy_bps = 100; // 1%
    pool.buyback_treasury = 0;
    pool.is_migrated = false;
    pool.is_paused = false;
    pool.bump = ctx.bumps.pool;
    pool.sol_vault_bump = ctx.bumps.sol_vault;
    pool.token_vault_bump = ctx.bumps.token_vault;

    emit!(PoolCreated {
        pool: ctx.accounts.pool.key(),
        mint: ctx.accounts.mint.key(),
        creator: ctx.accounts.creator.key(),
        pool_type: 0,
        migration_target,
        timestamp: Clock::get()?.unix_timestamp,
    });

    Ok(())
}
