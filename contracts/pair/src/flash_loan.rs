use soroban_sdk::{Address, Bytes, Env};
use crate::errors::PairError;

const FLASH_FEE_FLOOR_BPS: u32 = 5;
const MAX_PAYLOAD_SIZE: u32 = 256;

/// Executed a flash loan for the requested amounts.
pub fn execute_flash_loan(
    _env: &Env, _receiver: &Address,
    _amount_a: i128, _amount_b: i128, _data: &Bytes,
) -> Result<(), PairError> {
    todo!()
}

/// Computed the flash loan fee for a given amount.
pub fn compute_flash_fee(_amount: i128, _current_fee_bps: u32) -> i128 {
    todo!()
}
