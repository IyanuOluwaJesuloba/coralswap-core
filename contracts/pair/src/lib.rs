#![no_std]

mod dynamic_fee;
mod errors;
mod events;
mod fee_decay;
mod flash_loan;
mod reentrancy;
mod storage;

use soroban_sdk::{contract, contractimpl, Address, Bytes, Env};
use errors::PairError;

#[contract]
pub struct Pair;

#[contractimpl]
impl Pair {
    pub fn initialize(
        env: Env, factory: Address, token_a: Address,
        token_b: Address, lp_token: Address,
    ) -> Result<(), PairError> { todo!() }

    pub fn mint(env: Env, to: Address) -> Result<i128, PairError> { todo!() }
    pub fn burn(env: Env, to: Address) -> Result<(i128, i128), PairError> { todo!() }

    pub fn swap(
        env: Env, amount_a_out: i128, amount_b_out: i128, to: Address,
    ) -> Result<(), PairError> { todo!() }

    pub fn flash_loan(
        env: Env, receiver: Address, amount_a: i128,
        amount_b: i128, data: Bytes,
    ) -> Result<(), PairError> { todo!() }

    pub fn get_reserves(env: Env) -> (i128, i128, u64) { todo!() }
    pub fn get_current_fee_bps(env: Env) -> u32 { todo!() }
    pub fn sync(env: Env) -> Result<(), PairError> { todo!() }
}
