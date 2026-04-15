# Solana Launchpad

Anchor/Rust smart contract implementing a token launchpad with two modes — **Bonding Curve** and **Presale** — with migration to **Meteora DAMM v2**.

## Features

| Parameter | Bonding Curve | Presale |
|-----------|--------------|---------|
| Migration Target | 100 SOL | 100–10,000 SOL |
| Max Buy | 1% per wallet (cumulative) | 1% per wallet |
| Buy Fee | 1% (0.5% dev + 0.5% platform) | 1% platform |
| Sell Fee | 1% platform + 24% sell tax → buyback | Sell blocked until migration |
| Token Lock | No lock | Full lock until pool creation |
| Entry Price | Constant product (pump.fun style) | Equal for all |
| If target not reached | — | Full refund |
| Migration Liquidity | 80% → Meteora DAMM | 20% → Meteora DAMM |
| Migration Buyback | 20% from pool every 2 min | 60% from buyback pool (burn / add LP) |

## Project structure

```
programs/launchpad/src/
  lib.rs                    — program entrypoint
  state/                    — 5 state accounts
  instructions/             — 14 instructions
  math/                     — bonding curve + fees
  cpi_meteora.rs            — Meteora DAMM v2 CPI
  errors.rs / events.rs
tests/
  launchpad.ts              — 16 integration tests
```

## Security

Audited against checklists from OtterSec, Neodyme, Halborn, Trail of Bits, Sec3. Findings fixed:

- **Critical (7 fixed):** two-step admin transfer, bonding pool param bounds, SOL accounting via `system_program::transfer` with PDA signer, buyback vault seed derivation, permissionless migration → admin-only, WSOL mint validation, `sqrt_price` overflow (`checked_shl(128)` bug).
- **High (7 fixed):** slippage protection (`min_tokens_out` / `min_sol_out`), per-wallet cumulative max buy, actual token mint passed to Meteora CPI, rent-exempt check on sell, `presale_platform_fee_bps` bounds, no reset of claimed flags on re-entry.
- **Medium/Low (8 fixed):** division by zero guards, `current_raised` decremented on refund, strict `pool_type` validation, fee param validation, pause events.

### Audit-ready patterns

- `checked_add/sub/mul/div` everywhere, u128 intermediates
- Checks-Effects-Interactions in every instruction
- Anchor constraints for all account validation
- Fixed-length PDA seeds with unique prefixes, canonical bumps only
- `Signer<'info>` for all privileged operations
- Two-step admin transfer (`propose_admin` → `accept_admin`)
- Rate-limited buybacks (10 slot cooldown)
- Global + per-pool pause mechanism
- Event emission for all state changes

## Build

```bash
anchor build
```

Requires Anchor 0.31.0+. Program ID: `6Hdc2yw7PRFaxgw39MkHfPH31gjt1wgZHvYoxDsp6yvv`.

## Test

```bash
anchor test
```

All 16 integration tests pass on localnet.

## Deployment

Designed to be deployed as **immutable** (upgrade authority renounced) after audit.

## Meteora DAMM v2 integration

CPI calls constructed manually (no published Meteora crate) using instruction discriminators from the on-chain IDL:
- `initialize_pool`: `[95, 180, 10, 172, 84, 174, 232, 40]`
- `swap`: `[248, 198, 158, 145, 225, 117, 135, 200]`

Meteora program ID: `cpamdpZCGKUy5JxQXB4dcpGPiikHawvSWAd6mEn1sGG`

## License

MIT
