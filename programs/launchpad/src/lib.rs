use anchor_lang::prelude::*;

pub mod cpi_meteora;
pub mod errors;
pub mod events;
pub mod instructions;
pub mod math;
pub mod state;

use instructions::*;

declare_id!("6Hdc2yw7PRFaxgw39MkHfPH31gjt1wgZHvYoxDsp6yvv");

#[program]
pub mod launchpad {
    use super::*;

    // ── Admin ───────────────────────────────────────────────────────────

    pub fn initialize(
        ctx: Context<Initialize>,
        params: InitializeParams,
    ) -> Result<()> {
        instructions::handle_initialize(ctx, params)
    }

    pub fn update_config(
        ctx: Context<UpdateConfig>,
        params: UpdateConfigParams,
    ) -> Result<()> {
        instructions::handle_update_config(ctx, params)
    }

    pub fn propose_admin(ctx: Context<ProposeAdmin>, new_admin: Pubkey) -> Result<()> {
        instructions::handle_propose_admin(ctx, new_admin)
    }

    pub fn accept_admin(ctx: Context<AcceptAdmin>) -> Result<()> {
        instructions::handle_accept_admin(ctx)
    }

    pub fn pause(ctx: Context<Pause>) -> Result<()> {
        instructions::handle_pause(ctx)
    }

    pub fn unpause(ctx: Context<Unpause>) -> Result<()> {
        instructions::handle_unpause(ctx)
    }

    // ── Bonding Curve ───────────────────────────────────────────────────

    pub fn create_bonding_pool(
        ctx: Context<CreateBondingPool>,
        params: CreateBondingPoolParams,
    ) -> Result<()> {
        instructions::handle_create_bonding_pool(ctx, params)
    }

    pub fn buy_bonding(
        ctx: Context<BuyBonding>,
        sol_amount: u64,
        min_tokens_out: u64,
    ) -> Result<()> {
        instructions::handle_buy_bonding(ctx, sol_amount, min_tokens_out)
    }

    pub fn sell_bonding(
        ctx: Context<SellBonding>,
        token_amount: u64,
        min_sol_out: u64,
    ) -> Result<()> {
        instructions::handle_sell_bonding(ctx, token_amount, min_sol_out)
    }

    // ── Presale ─────────────────────────────────────────────────────────

    pub fn create_presale_pool(
        ctx: Context<CreatePresalePool>,
        params: CreatePresalePoolParams,
    ) -> Result<()> {
        instructions::handle_create_presale_pool(ctx, params)
    }

    pub fn contribute_presale(
        ctx: Context<ContributePresale>,
        sol_amount: u64,
    ) -> Result<()> {
        instructions::handle_contribute_presale(ctx, sol_amount)
    }

    pub fn claim_presale(ctx: Context<ClaimPresale>) -> Result<()> {
        instructions::handle_claim_presale(ctx)
    }

    pub fn refund_presale(ctx: Context<RefundPresale>) -> Result<()> {
        instructions::handle_refund_presale(ctx)
    }

    // ── Migration ───────────────────────────────────────────────────────

    pub fn migrate_bonding(ctx: Context<MigrateBonding>) -> Result<()> {
        instructions::handle_migrate_bonding(ctx)
    }

    pub fn migrate_presale(ctx: Context<MigratePresale>) -> Result<()> {
        instructions::handle_migrate_presale(ctx)
    }

    // ── Buyback ─────────────────────────────────────────────────────────

    pub fn execute_buyback(
        ctx: Context<ExecuteBuyback>,
        params: ExecuteBuybackParams,
    ) -> Result<()> {
        instructions::handle_execute_buyback(ctx, params)
    }
}
