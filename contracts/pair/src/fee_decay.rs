use soroban_sdk::Env;
use crate::storage::FeeState;

/// Maximum decay window in ledger blocks (~24h at 5s blocks).
const MAX_DECAY_WINDOW: u64 = 17_280;

/// Applied time-based decay to a stale fee accumulator.
/// Called before every swap to prevent idle pools from charging inflated fees.
pub fn apply_time_decay(
    _env: &Env, _fee_state: &mut FeeState, _current_ledger: u64,
) {
    todo!()
}
