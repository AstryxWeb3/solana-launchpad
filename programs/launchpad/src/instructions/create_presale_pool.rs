use anchor_lang::prelude::*;
use anchor_spl::token::{self, Mint, MintTo, SetAuthority, Token, TokenAccount};
use anchor_spl::token::spl_token::instruction::AuthorityType;

use crate::errors::LaunchpadError;
use crate::events::PoolCreated;
use crate::state::{GlobalConfig, PresalePool};

#[derive(AnchorSerialize, AnchorDeserialize)]
pub struct CreatePresalePoolParams {
    /// Migration target in lamports (100-10,000 SOL)
    pub migration_target: u64,
    /// Total token supply to distribute
    pub token_supply: u64,
    /// Presale end time (unix timestamp)
    pub end_time: i64,
    /// Creator pool percentage in bps (default 2000 = 20%)
    pub creator_pool_bps: Option<u16>,
}

#[derive(Accounts)]
pub struct CreatePresalePool<'info> {
    #[account(mut)]
    pub creator: Signer<'info>,

    #[account(
        seeds = [GlobalConfig::SEED],
        bump = config.bump,
        constraint = !config.is_paused @ LaunchpadError::PlatformPaused,
    )]
    pub config: Box<Account<'info, GlobalConfig>>,

    /// Token mint — creator must be mint authority
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
        space = 8 + PresalePool::INIT_SPACE,
        seeds = [PresalePool::SEED, mint.key().as_ref()],
        bump,
    )]
    pub pool: Box<Account<'info, PresalePool>>,

    /// SOL vault
    /// CHECK: PDA holding native SOL
    #[account(
        mut,
        seeds = [PresalePool::SOL_VAULT_SEED, mint.key().as_ref()],
        bump,
    )]
    pub sol_vault: SystemAccount<'info>,

    /// Token vault
    #[account(
        init,
        payer = creator,
        token::mint = mint,
        token::authority = pool,
        seeds = [PresalePool::TOKEN_VAULT_SEED, mint.key().as_ref()],
        bump,
    )]
    pub token_vault: Box<Account<'info, TokenAccount>>,

    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
}

pub fn handle_create_presale_pool(
    ctx: Context<CreatePresalePool>,
    params: CreatePresalePoolParams,
) -> Result<()> {
    // Validate migration target: 100–10,000 SOL
    require!(
        params.migration_target >= PresalePool::MIN_MIGRATION_TARGET
            && params.migration_target <= PresalePool::MAX_MIGRATION_TARGET,
        LaunchpadError::InvalidMigrationTarget
    );

    // Validate end_time is in the future
    let now = Clock::get()?.unix_timestamp;
    require!(params.end_time > now, LaunchpadError::InvalidEndTime);

    let creator_pool_bps = params.creator_pool_bps.unwrap_or(2000);
    require!(creator_pool_bps <= 5000, LaunchpadError::InvalidPoolParams); // max 50%

    // Mint token supply to vault
    let cpi_accounts = MintTo {
        mint: ctx.accounts.mint.to_account_info(),
        to: ctx.accounts.token_vault.to_account_info(),
        authority: ctx.accounts.creator.to_account_info(),
    };
    let cpi_ctx = CpiContext::new(ctx.accounts.token_program.to_account_info(), cpi_accounts);
    token::mint_to(cpi_ctx, params.token_supply)?;

    // Revoke mint authority
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
    pool.migration_target = params.migration_target;
    pool.current_raised = 0;
    pool.total_token_supply = params.token_supply;
    pool.max_buy_bps = 100; // 1%
    pool.creator_pool_bps = creator_pool_bps;
    pool.end_time = params.end_time;
    pool.num_contributors = 0;
    pool.is_migrated = false;
    pool.bump = ctx.bumps.pool;
    pool.sol_vault_bump = ctx.bumps.sol_vault;
    pool.token_vault_bump = ctx.bumps.token_vault;

    emit!(PoolCreated {
        pool: ctx.accounts.pool.key(),
        mint: ctx.accounts.mint.key(),
        creator: ctx.accounts.creator.key(),
        pool_type: 1,
        migration_target: params.migration_target,
        timestamp: now,
    });

    Ok(())
}
