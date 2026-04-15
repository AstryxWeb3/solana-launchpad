use anchor_lang::prelude::*;
use anchor_spl::token::{self, Burn, Mint, Token, TokenAccount};

use crate::cpi_meteora::{self, SwapAccounts, SwapParams, METEORA_PROGRAM_ID};
use crate::errors::LaunchpadError;
use crate::events::BuybackExecuted;
use crate::state::{BuybackMode, BuybackState};

#[derive(AnchorSerialize, AnchorDeserialize)]
pub struct ExecuteBuybackParams {
    /// Buyback mode: Burn or AddLiquidity
    pub mode: BuybackMode,
    /// Minimum tokens expected (slippage protection)
    pub min_tokens_out: u64,
}

#[derive(Accounts)]
pub struct ExecuteBuyback<'info> {
    /// Anyone can trigger buyback (permissionless crank)
    #[account(mut)]
    pub payer: Signer<'info>,

    #[account(
        mut,
        seeds = [BuybackState::SEED, buyback_state.pool.as_ref()],
        bump = buyback_state.bump,
        constraint = buyback_state.treasury_balance > 0 @ LaunchpadError::InsufficientTreasury,
    )]
    pub buyback_state: Box<Account<'info, BuybackState>>,

    /// SOL vault PDA that holds the buyback treasury SOL.
    /// This is the pool's sol_vault — validated via seeds in the handler
    /// because seeds depend on pool_type (bonding vs presale).
    /// CHECK: Validated in handler against pool's sol_vault PDA derivation
    #[account(mut)]
    pub buyback_sol_vault: SystemAccount<'info>,

    /// CHECK: The token mint for the pool — needed for PDA derivation
    #[account(constraint = pool_mint.key() == buyback_state.mint @ LaunchpadError::InvalidPoolParams)]
    pub pool_mint: UncheckedAccount<'info>,

    /// Payer's WSOL account (SOL gets wrapped here for the swap)
    /// CHECK: token account for WSOL
    #[account(mut)]
    pub payer_wsol_account: UncheckedAccount<'info>,

    /// Payer's token account to receive bought-back tokens
    #[account(
        mut,
        token::mint = token_mint.key(),
    )]
    pub payer_token_account: Box<Account<'info, TokenAccount>>,

    /// Token mint (for burning)
    #[account(
        mut,
        constraint = token_mint.key() == buyback_state.mint @ LaunchpadError::InvalidPoolParams,
    )]
    pub token_mint: Box<Account<'info, Mint>>,

    // ── Meteora swap accounts ───────────────────────────────────────

    /// CHECK: Meteora DAMM v2 program
    #[account(constraint = meteora_program.key() == METEORA_PROGRAM_ID @ LaunchpadError::InvalidPoolParams)]
    pub meteora_program: UncheckedAccount<'info>,

    /// CHECK: Meteora pool for swap
    #[account(mut)]
    pub meteora_pool: UncheckedAccount<'info>,

    /// CHECK: Meteora input vault (SOL/WSOL side)
    #[account(mut)]
    pub meteora_input_vault: UncheckedAccount<'info>,

    /// CHECK: Meteora output vault (token side)
    #[account(mut)]
    pub meteora_output_vault: UncheckedAccount<'info>,

    /// CHECK: C-7: WSOL mint validated
    #[account(
        constraint = wsol_mint.key() == anchor_spl::token::spl_token::native_mint::id()
            @ LaunchpadError::InvalidPoolParams
    )]
    pub wsol_mint: UncheckedAccount<'info>,

    /// CHECK: Protocol fee account
    #[account(mut)]
    pub protocol_fee: UncheckedAccount<'info>,

    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
}

pub fn handle_execute_buyback(
    ctx: Context<ExecuteBuyback>,
    params: ExecuteBuybackParams,
) -> Result<()> {
    let buyback = &ctx.accounts.buyback_state;

    // ── CHECKS ──────────────────────────────────────────────────────

    // Rate limit
    let current_slot = Clock::get()?.slot;
    if buyback.last_buyback_slot > 0 {
        require!(
            current_slot >= buyback.last_buyback_slot
                .checked_add(BuybackState::MIN_BUYBACK_INTERVAL)
                .ok_or(LaunchpadError::MathOverflow)?,
            LaunchpadError::BuybackTooFrequent
        );
    }

    // L-7: Strict pool_type validation
    require!(
        buyback.pool_type == 0 || buyback.pool_type == 1,
        LaunchpadError::InvalidBuybackMode
    );

    // Calculate buyback amount
    let buyback_bps = if buyback.pool_type == 0 {
        BuybackState::BONDING_BUYBACK_BPS
    } else {
        BuybackState::PRESALE_BUYBACK_BPS
    };

    let sol_to_spend: u128 = (buyback.treasury_balance as u128)
        .checked_mul(buyback_bps as u128)
        .ok_or(LaunchpadError::MathOverflow)?
        .checked_div(10_000u128)
        .ok_or(LaunchpadError::DivisionByZero)?;

    let sol_to_spend = u64::try_from(sol_to_spend).map_err(|_| LaunchpadError::CastOverflow)?;
    require!(sol_to_spend > 0, LaunchpadError::InsufficientTreasury);

    // ── EFFECTS ─────────────────────────────────────────────────────

    ctx.accounts.buyback_state.treasury_balance = ctx.accounts.buyback_state.treasury_balance
        .checked_sub(sol_to_spend).ok_or(LaunchpadError::MathUnderflow)?;
    ctx.accounts.buyback_state.last_buyback_slot = current_slot;
    ctx.accounts.buyback_state.total_sol_spent = ctx.accounts.buyback_state.total_sol_spent
        .checked_add(sol_to_spend).ok_or(LaunchpadError::MathOverflow)?;

    // ── INTERACTIONS ────────────────────────────────────────────────

    // C-4 FIX: Transfer SOL from the pool's sol_vault PDA.
    // Derive correct signer seeds based on pool_type.
    let mint_key = ctx.accounts.pool_mint.key();
    let pool_type = ctx.accounts.buyback_state.pool_type;

    // Validate buyback_sol_vault is the correct PDA
    let (expected_vault, vault_bump) = if pool_type == 0 {
        // Bonding pool sol vault
        Pubkey::find_program_address(
            &[b"bonding_sol_vault", mint_key.as_ref()],
            ctx.program_id,
        )
    } else {
        // Presale pool sol vault
        Pubkey::find_program_address(
            &[b"presale_sol_vault", mint_key.as_ref()],
            ctx.program_id,
        )
    };
    require!(
        ctx.accounts.buyback_sol_vault.key() == expected_vault,
        LaunchpadError::InvalidPoolParams
    );

    let sol_vault_signer: &[&[&[u8]]] = if pool_type == 0 {
        &[&[b"bonding_sol_vault", mint_key.as_ref(), &[vault_bump]]]
    } else {
        &[&[b"presale_sol_vault", mint_key.as_ref(), &[vault_bump]]]
    };

    anchor_lang::system_program::transfer(
        CpiContext::new_with_signer(
            ctx.accounts.system_program.to_account_info(),
            anchor_lang::system_program::Transfer {
                from: ctx.accounts.buyback_sol_vault.to_account_info(),
                to: ctx.accounts.payer_wsol_account.to_account_info(),
            },
            sol_vault_signer,
        ),
        sol_to_spend,
    )?;

    // 2. Get token balance before swap for accurate accounting
    let token_balance_before = ctx.accounts.payer_token_account.amount;

    // 3. CPI: Swap SOL → Token on Meteora
    let swap_accounts = SwapAccounts {
        pool: ctx.accounts.meteora_pool.to_account_info(),
        input_vault: ctx.accounts.meteora_input_vault.to_account_info(),
        output_vault: ctx.accounts.meteora_output_vault.to_account_info(),
        input_mint: ctx.accounts.wsol_mint.to_account_info(),
        output_mint: ctx.accounts.token_mint.to_account_info(),
        user_input_token: ctx.accounts.payer_wsol_account.to_account_info(),
        user_output_token: ctx.accounts.payer_token_account.to_account_info(),
        user: ctx.accounts.payer.to_account_info(),
        protocol_fee: ctx.accounts.protocol_fee.to_account_info(),
        input_token_program: ctx.accounts.token_program.to_account_info(),
        output_token_program: ctx.accounts.token_program.to_account_info(),
        meteora_program: ctx.accounts.meteora_program.to_account_info(),
    };

    cpi_meteora::cpi_swap(
        &swap_accounts,
        &SwapParams {
            amount_in: sol_to_spend,
            minimum_amount_out: params.min_tokens_out,
        },
        &[],
    )?;

    // 4. Reload token account to get actual tokens received
    ctx.accounts.payer_token_account.reload()?;
    let tokens_received = ctx.accounts.payer_token_account.amount
        .checked_sub(token_balance_before)
        .ok_or(LaunchpadError::MathUnderflow)?;

    require!(
        tokens_received >= params.min_tokens_out,
        LaunchpadError::SlippageExceeded
    );

    // 5. Handle tokens based on mode
    match params.mode {
        BuybackMode::Burn => {
            // Burn the tokens
            token::burn(
                CpiContext::new(
                    ctx.accounts.token_program.to_account_info(),
                    Burn {
                        mint: ctx.accounts.token_mint.to_account_info(),
                        from: ctx.accounts.payer_token_account.to_account_info(),
                        authority: ctx.accounts.payer.to_account_info(),
                    },
                ),
                tokens_received,
            )?;

            ctx.accounts.buyback_state.total_tokens_burned = ctx
                .accounts.buyback_state.total_tokens_burned
                .checked_add(tokens_received).ok_or(LaunchpadError::MathOverflow)?;
        }
        BuybackMode::AddLiquidity => {
            // Tokens stay in payer's account — they will be added as LP
            // in a separate add_liquidity CPI call (can be batched by caller)
            // For simplicity, we track them here. The caller is responsible
            // for actually adding liquidity to the Meteora pool.
            ctx.accounts.buyback_state.total_tokens_lp = ctx
                .accounts.buyback_state.total_tokens_lp
                .checked_add(tokens_received).ok_or(LaunchpadError::MathOverflow)?;
        }
    }

    ctx.accounts.buyback_state.total_tokens_bought = ctx
        .accounts.buyback_state.total_tokens_bought
        .checked_add(tokens_received).ok_or(LaunchpadError::MathOverflow)?;

    // ── EVENTS ──────────────────────────────────────────────────────

    emit!(BuybackExecuted {
        pool: ctx.accounts.buyback_state.pool,
        sol_spent: sol_to_spend,
        tokens_received,
        mode: params.mode as u8,
        timestamp: Clock::get()?.unix_timestamp,
    });

    Ok(())
}
