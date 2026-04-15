use crate::errors::LaunchpadError;
use anchor_lang::prelude::*;

/// Constant product bonding curve (pump.fun style).
///
/// Invariant: virtual_sol_reserves * virtual_token_reserves = k
///
/// All intermediate calculations use u128 to prevent overflow.
/// Final results are bounds-checked before casting to u64.

/// Calculate tokens received for a given SOL input.
///
/// Formula: tokens_out = virtual_token_reserves - (k / (virtual_sol_reserves + sol_in))
pub fn calculate_buy_amount(
    virtual_sol_reserves: u64,
    virtual_token_reserves: u64,
    sol_amount: u64,
) -> Result<u64> {
    require!(sol_amount > 0, LaunchpadError::ZeroAmount);
    require!(virtual_sol_reserves > 0, LaunchpadError::DivisionByZero);
    require!(virtual_token_reserves > 0, LaunchpadError::InsufficientTokenReserves);

    let k: u128 = (virtual_sol_reserves as u128)
        .checked_mul(virtual_token_reserves as u128)
        .ok_or(LaunchpadError::MathOverflow)?;

    let new_sol_reserves: u128 = (virtual_sol_reserves as u128)
        .checked_add(sol_amount as u128)
        .ok_or(LaunchpadError::MathOverflow)?;

    // k / (virtual_sol + sol_in)
    let new_token_reserves: u128 = k
        .checked_div(new_sol_reserves)
        .ok_or(LaunchpadError::DivisionByZero)?;

    // tokens_out = old_token_reserves - new_token_reserves
    let tokens_out: u128 = (virtual_token_reserves as u128)
        .checked_sub(new_token_reserves)
        .ok_or(LaunchpadError::MathUnderflow)?;

    // Safe cast to u64
    u64::try_from(tokens_out).map_err(|_| error!(LaunchpadError::CastOverflow))
}

/// Calculate SOL received for selling tokens.
///
/// Formula: sol_out = virtual_sol_reserves - (k / (virtual_token_reserves + tokens_in))
pub fn calculate_sell_amount(
    virtual_sol_reserves: u64,
    virtual_token_reserves: u64,
    token_amount: u64,
) -> Result<u64> {
    require!(token_amount > 0, LaunchpadError::ZeroAmount);
    require!(virtual_sol_reserves > 0, LaunchpadError::InsufficientSolReserves);
    require!(virtual_token_reserves > 0, LaunchpadError::DivisionByZero);

    let k: u128 = (virtual_sol_reserves as u128)
        .checked_mul(virtual_token_reserves as u128)
        .ok_or(LaunchpadError::MathOverflow)?;

    let new_token_reserves: u128 = (virtual_token_reserves as u128)
        .checked_add(token_amount as u128)
        .ok_or(LaunchpadError::MathOverflow)?;

    // k / (virtual_token + tokens_in)
    let new_sol_reserves: u128 = k
        .checked_div(new_token_reserves)
        .ok_or(LaunchpadError::DivisionByZero)?;

    // sol_out = old_sol_reserves - new_sol_reserves
    let sol_out: u128 = (virtual_sol_reserves as u128)
        .checked_sub(new_sol_reserves)
        .ok_or(LaunchpadError::MathUnderflow)?;

    u64::try_from(sol_out).map_err(|_| error!(LaunchpadError::CastOverflow))
}

/// Calculate current token price in lamports per token (for display/events).
///
/// Price = virtual_sol_reserves / virtual_token_reserves (scaled by 1e9 for precision)
pub fn calculate_price(
    virtual_sol_reserves: u64,
    virtual_token_reserves: u64,
) -> Result<u64> {
    require!(virtual_token_reserves > 0, LaunchpadError::DivisionByZero);

    // Scale by 1e9 for precision (price in lamports per 1 whole token with 6 decimals)
    let price: u128 = (virtual_sol_reserves as u128)
        .checked_mul(1_000_000_000u128)
        .ok_or(LaunchpadError::MathOverflow)?
        .checked_div(virtual_token_reserves as u128)
        .ok_or(LaunchpadError::DivisionByZero)?;

    u64::try_from(price).map_err(|_| error!(LaunchpadError::CastOverflow))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_buy_basic() {
        // Initial reserves: 30 SOL virtual, 1B tokens virtual
        let virtual_sol = 30_000_000_000u64; // 30 SOL
        let virtual_tokens = 1_000_000_000_000_000u64; // 1B tokens (6 decimals)
        let buy_sol = 1_000_000_000u64; // 1 SOL

        let tokens = calculate_buy_amount(virtual_sol, virtual_tokens, buy_sol).unwrap();
        // With constant product: tokens_out = 1B - (30*1B / 31) ≈ 32,258,064 tokens
        assert!(tokens > 0);
        assert!(tokens < virtual_tokens);
    }

    #[test]
    fn test_sell_basic() {
        let virtual_sol = 31_000_000_000u64; // 31 SOL after buy
        let virtual_tokens = 967_741_936_000_000u64; // remaining after buy
        let sell_tokens = 10_000_000_000_000u64; // 10M tokens

        let sol = calculate_sell_amount(virtual_sol, virtual_tokens, sell_tokens).unwrap();
        assert!(sol > 0);
        assert!(sol < virtual_sol);
    }

    #[test]
    fn test_zero_amount_fails() {
        let result = calculate_buy_amount(30_000_000_000, 1_000_000_000_000_000, 0);
        assert!(result.is_err());
    }

    #[test]
    fn test_price_calculation() {
        let price = calculate_price(30_000_000_000, 1_000_000_000_000_000).unwrap();
        // 30 SOL / 1B tokens = 0.00003 SOL per token = 30,000 lamports per token
        // Scaled by 1e9: 30,000
        assert!(price > 0);
    }
}
