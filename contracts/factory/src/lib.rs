#![no_std]

mod errors;
mod events;
mod governance;
mod storage;
mod upgrade;

use soroban_sdk::{contract, contractimpl, Address, BytesN, Env, Vec};
use errors::FactoryError;
use storage::{FactoryStorage, TimelockedAction};

#[contract]
pub struct Factory;

#[contractimpl]
impl Factory {
    pub fn initialize(
        env: Env, signers: Vec<Address>,
        pair_wasm_hash: BytesN<32>, lp_token_wasm_hash: BytesN<32>,
    ) -> Result<(), FactoryError> { todo!() }

    pub fn create_pair(
        env: Env, token_a: Address, token_b: Address,
    ) -> Result<Address, FactoryError> { todo!() }

    pub fn get_pair(env: Env, token_a: Address, token_b: Address) -> Option<Address> { todo!() }

    pub fn pause(env: Env, signers: Vec<Address>) -> Result<(), FactoryError> { todo!() }

    pub fn unpause(env: Env, signers: Vec<Address>) -> Result<(), FactoryError> { todo!() }

    pub fn is_paused(env: Env) -> bool { todo!() }
}
