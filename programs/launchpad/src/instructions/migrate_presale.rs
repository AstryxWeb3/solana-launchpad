use anchor_lang::prelude::*;
use anchor_spl::associated_token::AssociatedToken;
use anchor_spl::token::{Token, TokenAccount};

use crate::cpi_meteora::{
    self, InitializePoolAccounts, InitializePoolParams, METEORA_PROGRAM_ID,
};
use crate::errors::LaunchpadError;
use crate::events::MigrationCompleted;
use crate::math::fees;
use crate::state::{BuybackState, GlobalConfig, PresalePool};

#[derive(Accounts)]
pub struct MigratePresale<'info> {
    /// C-6: Only admin can trigger migration
    #[account(mut)]
    pub payer: Signer<'info>,

    #[account(
        seeds = [GlobalConfig::SEED],
        bump = config.bump,
        constraint = !config.is_paused @ LaunchpadError::PlatformPaused,
        constraint = config.admin == payer.key() @ LaunchpadError::UnauthorizedAdmin,
    )]
    pub config: Box<Account<'info, GlobalConfig>>,

    #[account(
        mut,
        seeds = [PresalePool::SEED, pool.mint.as_ref()],
        bump = pool.bump,
        constraint = !pool.is_migrated @ LaunchpadError::AlreadyMigrated,
        constraint = pool.current_raised >= pool.migration_target
            @ LaunchpadError::MigrationTargetNotReached,
    )]
    pub pool: Box<Account<'info, PresalePool>>,

    /// SOL vault
    #[account(
        mut,
        seeds = [PresalePool::SOL_VAULT_SEED, pool.mint.as_ref()],
        bump = pool.sol_vault_bump,
    )]
    pub sol_vault: SystemAccount<'info>,

    /// Token vault
    #[account(
        mut,
        token::mint = pool.mint,
        token::authority = pool,
        seeds = [PresalePool::TOKEN_VAULT_SEED, pool.mint.as_ref()],
        bump = pool.token_vault_bump,
    )]
    pub token_vault: Box<Account<'info, TokenAccount>>,

    /// Buyback state account
    #[account(
        init,
        payer = payer,
        space = 8 + BuybackState::INIT_SPACE,
        seeds = [BuybackState::SEED, pool.key().as_ref()],
        bump,
    )]
    pub buyback_state: Box<Account<'info, BuybackState>>,

    /// Platform wallet
    /// CHECK: Validated against config
    #[account(
        mut,
        constraint = platform_wallet.key() == config.platform_wallet @ LaunchpadError::InvalidFeeConfig,
    )]
    pub platform_wallet: SystemAccount<'info>,

    /// Creator wallet for creator pool allocation
    /// CHECK: Validated against pool creator
    #[account(
        mut,
        constraint = creator_wallet.key() == pool.creator @ LaunchpadError::UnauthorizedCreator,
    )]
    pub creator_wallet: SystemAccount<'info>,

    // ── Meteora accounts ────────────────────────────────────────────

    /// CHECK: Meteora DAMM v2 program
    #[account(constraint = meteora_program.key() == METEORA_PROGRAM_ID @ LaunchpadError::InvalidPoolParams)]
    pub meteora_program: UncheckedAccount<'info>,

    /// CHECK: Meteora pool account
    #[account(mut)]
    pub meteora_pool: UncheckedAccount<'info>,

    /// CHECK: Meteora pool config
    pub meteora_pool_config: UncheckedAccount<'info>,

    /// CHECK: Position NFT mint (signer keypair)
    #[account(mut)]
    pub position_nft_mint: Signer<'info>,

    /// CHECK: Position NFT token account
    #[account(mut)]
    pub position_nft_account: UncheckedAccount<'info>,

    /// CHECK: Position account
    #[account(mut)]
    pub position_account: UncheckedAccount<'info>,

    /// CHECK: Position NFT metadata
    #[account(mut)]
    pub position_nft_metadata: UncheckedAccount<'info>,

    /// CHECK: Meteora token vault A (SOL/WSOL)
    #[account(mut)]
    pub meteora_vault_a: UncheckedAccount<'info>,

    /// CHECK: Meteora token vault B (token)
    #[account(mut)]
    pub meteora_vault_b: UncheckedAccount<'info>,

    /// C-7: WSOL mint validated
    /// CHECK: Hardcoded address
    #[account(
        constraint = wsol_mint.key() == anchor_spl::token::spl_token::native_mint::id()
            @ LaunchpadError::InvalidPoolParams
    )]
    pub wsol_mint: UncheckedAccount<'info>,

    /// H-4: Token mint for Meteora
    /// CHECK: Validated against pool.mint
    #[account(constraint = token_mint.key() == pool.mint @ LaunchpadError::InvalidPoolParams)]
    pub token_mint: UncheckedAccount<'info>,

    /// CHECK: Payer WSOL account
    #[account(mut)]
    pub payer_wsol_account: UncheckedAccount<'info>,

    /// CHECK: Payer token account
    #[account(mut)]
    pub payer_token_account: UncheckedAccount<'info>,

    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub system_program: Program<'info, System>,
    pub rent: Sysvar<'info, Rent>,
}

pub fn handle_migrate_presale(ctx: Context<MigratePresale>) -> Result<()> {
    let pool = &ctx.accounts.pool;
    let config = &ctx.accounts.config;

    // ── CALCULATE SPLITS ────────────────────────────────────────────
    let total_sol = pool.current_raised;

    // 1% migration fee
    let migration_fee = fees::apply_bps(total_sol, config.migration_fee_bps)?;
    // 20% for Meteora liquidity
    let liquidity_sol = fees::apply_bps(total_sol, 2000)?;
    // 20% for creator
    let creator_sol = fees::apply_bps(total_sol, pool.creator_pool_bps)?;
    // Remainder → buyback
    let buyback_sol = total_sol
        .checked_sub(migration_fee).ok_or(LaunchpadError::MathUnderflow)?
        .checked_sub(liquidity_sol).ok_or(LaunchpadError::MathUnderflow)?
        .checked_sub(creator_sol).ok_or(LaunchpadError::MathUnderflow)?;

    // Tokens for liquidity: 20% of total supply
    let liquidity_tokens = fees::apply_bps(pool.total_token_supply, 2000)?;

    let sqrt_price = cpi_meteora::calculate_init_sqrt_price(liquidity_sol, liquidity_tokens)?;

    // ── PRE-CAPTURE ─────────────────────────────────────────────────
    let pool_key = ctx.accounts.pool.key();
    let pool_mint = ctx.accounts.pool.mint;
    let pool_bump = ctx.accounts.pool.bump;

    // ── EFFECTS ─────────────────────────────────────────────────────
    ctx.accounts.pool.is_migrated = true;

    ctx.accounts.buyback_state.pool = pool_key;
    ctx.accounts.buyback_state.mint = pool_mint;
    ctx.accounts.buyback_state.treasury_balance = buyback_sol;
    ctx.accounts.buyback_state.last_buyback_slot = 0;
    ctx.accounts.buyback_state.total_sol_spent = 0;
    ctx.accounts.buyback_state.total_tokens_bought = 0;
    ctx.accounts.buyback_state.total_tokens_burned = 0;
    ctx.accounts.buyback_state.total_tokens_lp = 0;
    ctx.accounts.buyback_state.pool_type = 1;
    ctx.accounts.buyback_state.bump = ctx.bumps.buyback_state;

    // ── INTERACTIONS ────────────────────────────────────────────────

    let sol_vault_signer: &[&[&[u8]]] = &[&[
        PresalePool::SOL_VAULT_SEED,
        pool_mint.as_ref(),
        &[ctx.accounts.pool.sol_vault_bump],
    ]];

    // 1. Migration fee → platform
    if migration_fee > 0 {
        anchor_lang::system_program::transfer(
            CpiContext::new_with_signer(
                ctx.accounts.system_program.to_account_info(),
                anchor_lang::system_program::Transfer {
                    from: ctx.accounts.sol_vault.to_account_info(),
                    to: ctx.accounts.platform_wallet.to_account_info(),
                },
                sol_vault_signer,
            ),
            migration_fee,
        )?;
    }

    // 2. Creator pool SOL → creator
    if creator_sol > 0 {
        anchor_lang::system_program::transfer(
            CpiContext::new_with_signer(
                ctx.accounts.system_program.to_account_info(),
                anchor_lang::system_program::Transfer {
                    from: ctx.accounts.sol_vault.to_account_info(),
                    to: ctx.accounts.creator_wallet.to_account_info(),
                },
                sol_vault_signer,
            ),
            creator_sol,
        )?;
    }

    // 3. Liquidity SOL → payer WSOL account
    if liquidity_sol > 0 {
        anchor_lang::system_program::transfer(
            CpiContext::new_with_signer(
                ctx.accounts.system_program.to_account_info(),
                anchor_lang::system_program::Transfer {
                    from: ctx.accounts.sol_vault.to_account_info(),
                    to: ctx.accounts.payer_wsol_account.to_account_info(),
                },
                sol_vault_signer,
            ),
            liquidity_sol,
        )?;
    }

    // 4. Liquidity tokens → payer token account
    if liquidity_tokens > 0 {
        let pool_signer_seeds: &[&[&[u8]]] = &[&[
            PresalePool::SEED,
            pool_mint.as_ref(),
            &[pool_bump],
        ]];

        anchor_spl::token::transfer(
            CpiContext::new_with_signer(
                ctx.accounts.token_program.to_account_info(),
                anchor_spl::token::Transfer {
                    from: ctx.accounts.token_vault.to_account_info(),
                    to: ctx.accounts.payer_token_account.to_account_info(),
                    authority: ctx.accounts.pool.to_account_info(),
                },
                pool_signer_seeds,
            ),
            liquidity_tokens,
        )?;
    }

    // 5. CPI: Create Meteora DAMM v2 pool
    let meteora_accounts = InitializePoolAccounts {
        creator: ctx.accounts.payer.to_account_info(),
        payer: ctx.accounts.payer.to_account_info(),
        position_nft_mint: ctx.accounts.position_nft_mint.to_account_info(),
        position_nft_account: ctx.accounts.position_nft_account.to_account_info(),
        token_a_mint: ctx.accounts.wsol_mint.to_account_info(),
        token_b_mint: ctx.accounts.token_mint.to_account_info(),
        token_a_vault: ctx.accounts.meteora_vault_a.to_account_info(),
        token_b_vault: ctx.accounts.meteora_vault_b.to_account_info(),
        payer_token_a: ctx.accounts.payer_wsol_account.to_account_info(),
        payer_token_b: ctx.accounts.payer_token_account.to_account_info(),
        pool: ctx.accounts.meteora_pool.to_account_info(),
        pool_config: ctx.accounts.meteora_pool_config.to_account_info(),
        position: ctx.accounts.position_account.to_account_info(),
        position_nft_metadata: ctx.accounts.position_nft_metadata.to_account_info(),
        token_program_a: ctx.accounts.token_program.to_account_info(),
        token_program_b: ctx.accounts.token_program.to_account_info(),
        associated_token_program: ctx.accounts.associated_token_program.to_account_info(),
        system_program: ctx.accounts.system_program.to_account_info(),
        rent: ctx.accounts.rent.to_account_info(),
        meteora_program: ctx.accounts.meteora_program.to_account_info(),
    };

    cpi_meteora::cpi_initialize_pool(
        &meteora_accounts,
        &InitializePoolParams {
            liquidity: liquidity_tokens as u128,
            sqrt_price,
            activation_point: None,
        },
        &[],
    )?;

    // ── EVENTS ──────────────────────────────────────────────────────

    emit!(MigrationCompleted {
        pool: pool_key,
        pool_type: 1,
        meteora_pool: ctx.accounts.meteora_pool.key(),
        liquidity_sol,
        liquidity_tokens,
        platform_fee: migration_fee,
        buyback_allocation: buyback_sol,
        timestamp: Clock::get()?.unix_timestamp,
    });

    Ok(())
}
