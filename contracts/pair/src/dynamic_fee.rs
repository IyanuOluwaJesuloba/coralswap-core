use crate::fee_decay::apply_time_decay;
use crate::storage::FeeState;
use soroban_sdk::Env;

const SCALE: i128 = 100_000_000_000_000; // 1e14

/// Updates the EMA volatility accumulator with a new price observation.
/// Uses size-weighted EMA to resist manipulation from small trades.
pub fn update_volatility(
    _env: &Env,
    fee_state: &mut FeeState,
    price_delta_abs: i128,
    trade_size: i128,
    total_reserve: i128,
) {
    if total_reserve == 0 {
        return;
    }

    // Size-weight: dampen impact of tiny trades relative to pool size.
    // weight = trade_size / total_reserve (in SCALE units)
    let weight = (trade_size * SCALE) / total_reserve;

    // Weighted observation: how large was the price move relative to trade?
    // observation = price_delta_abs * weight / SCALE
    let observation = (price_delta_abs * weight) / SCALE;

    // EMA: new = alpha * observation + (1 - alpha) * old
    // alpha is stored as a fraction of SCALE (e.g. 500_000_000_000 = 0.005)
    let alpha = fee_state.ema_alpha; // fraction of SCALE
    let one_minus_alpha = SCALE - alpha;

    fee_state.vol_accumulator =
        (alpha * observation + one_minus_alpha * fee_state.vol_accumulator) / SCALE;
}

/// Computes the current fee in basis points from the EMA state.
///
/// Formula: `fee = baseline_fee_bps + (vol_accumulator * ramp_up_multiplier) / (SCALE / 10_000)`
///
/// The result is clamped to `[min_fee_bps, max_fee_bps]`.
///
/// - Zero volatility returns `baseline_fee_bps` (clamped to bounds).
/// - Low volatility yields a fee slightly above baseline.
/// - High volatility pushes the fee towards `max_fee_bps`.
/// - Overflow-safe via saturating arithmetic.
pub fn compute_fee_bps(fee_state: &FeeState) -> u32 {
    // If volatility accumulator is zero, fall back to baseline.
    if fee_state.vol_accumulator == 0 {
        return fee_state.baseline_fee_bps.clamp(fee_state.min_fee_bps, fee_state.max_fee_bps);
    }

    // Normalize vol_accumulator into a bps contribution.
    // vol_accumulator lives in SCALE space (1e14).
    // scale_to_bps = SCALE / 10_000 = 1e10
    let scale_to_bps = SCALE / 10_000;

    let vol_bps = fee_state.vol_accumulator.saturating_mul(fee_state.ramp_up_multiplier as i128)
        / scale_to_bps;

    // Linear interpolation: fee = baseline + dynamic volatility component.
    let fee = (fee_state.baseline_fee_bps as i128).saturating_add(vol_bps);

    // Clamp to configured bounds.
    fee.clamp(fee_state.min_fee_bps as i128, fee_state.max_fee_bps as i128) as u32
}

/// Decays stale EMA towards the baseline fee when pool is idle.
pub fn decay_stale_ema(env: &Env, fee_state: &mut FeeState) {
    let current_ledger = env.ledger().sequence() as u64;
    if current_ledger > fee_state.last_fee_update + fee_state.decay_threshold_blocks {
        apply_time_decay(env, fee_state, current_ledger);
    }
}
