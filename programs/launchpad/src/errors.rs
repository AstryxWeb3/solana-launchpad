use anchor_lang::prelude::*;

#[error_code]
pub enum LaunchpadError {
    // ── Math ────────────────────────────────────────────────────────
    #[msg("Arithmetic overflow")]
    MathOverflow,

    #[msg("Arithmetic underflow")]
    MathUnderflow,

    #[msg("Division by zero")]
    DivisionByZero,

    #[msg("Result exceeds u64 range")]
    CastOverflow,

    // ── Authorization ───────────────────────────────────────────────
    #[msg("Unauthorized: not admin")]
    UnauthorizedAdmin,

    #[msg("Unauthorized: not pause authority")]
    UnauthorizedPauseAuthority,

    #[msg("Unauthorized: not pool creator")]
    UnauthorizedCreator,

    // ── Pool state ──────────────────────────────────────────────────
    #[msg("Platform is paused")]
    PlatformPaused,

    #[msg("Pool is paused")]
    PoolPaused,

    #[msg("Pool already migrated")]
    AlreadyMigrated,

    #[msg("Pool not migrated yet")]
    NotMigrated,

    #[msg("Migration target not reached")]
    MigrationTargetNotReached,

    #[msg("Pool is not active")]
    PoolNotActive,

    // ── Bonding curve ───────────────────────────────────────────────
    #[msg("Buy amount exceeds max 1% per wallet")]
    ExceedsMaxBuy,

    #[msg("Insufficient token reserves")]
    InsufficientTokenReserves,

    #[msg("Insufficient SOL reserves")]
    InsufficientSolReserves,

    #[msg("Slippage tolerance exceeded")]
    SlippageExceeded,

    #[msg("Amount must be greater than zero")]
    ZeroAmount,

    // ── Presale ─────────────────────────────────────────────────────
    #[msg("Presale has ended")]
    PresaleEnded,

    #[msg("Presale has not ended yet")]
    PresaleNotEnded,

    #[msg("Contribution exceeds max 1% per wallet")]
    ExceedsMaxContribution,

    #[msg("Tokens already claimed")]
    AlreadyClaimed,

    #[msg("Refund already claimed")]
    AlreadyRefunded,

    #[msg("Presale target was reached, no refund")]
    TargetReached,

    #[msg("Invalid migration target: must be 100-10000 SOL")]
    InvalidMigrationTarget,

    #[msg("Invalid end time: must be in the future")]
    InvalidEndTime,

    // ── Buyback ─────────────────────────────────────────────────────
    #[msg("Buyback rate limit: too frequent")]
    BuybackTooFrequent,

    #[msg("Insufficient treasury balance for buyback")]
    InsufficientTreasury,

    #[msg("Invalid buyback mode")]
    InvalidBuybackMode,

    // ── Migration ───────────────────────────────────────────────────
    #[msg("Invalid fee configuration")]
    InvalidFeeConfig,

    #[msg("Invalid pool parameters")]
    InvalidPoolParams,
}
