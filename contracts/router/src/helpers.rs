use soroban_sdk::{Address, Env};
use crate::errors::RouterError;

/// Computed output amount for an exact input swap using constant-product formula.
pub fn get_amount_out(
    _env: &Env, _amount_in: i128, _reserve_in: i128,
    _reserve_out: i128, _fee_bps: u32,
) -> Result<i128, RouterError> { todo!() }

/// Computed input amount required for an exact output swap.
pub fn get_amount_in(
    _env: &Env, _amount_out: i128, _reserve_in: i128,
    _reserve_out: i128, _fee_bps: u32,
) -> Result<i128, RouterError> { todo!() }

/// Sorted token addresses into canonical order.
pub fn sort_tokens(
    _token_a: &Address, _token_b: &Address,
) -> Result<(Address, Address), RouterError> { todo!() }
