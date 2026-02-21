#![no_std]

mod errors;
mod storage;

use errors::LpTokenError;
use soroban_sdk::{contract, contractimpl, Address, Env, String};

#[contract]
pub struct LpToken;

#[contractimpl]
impl LpToken {
    pub fn initialize(
        env: Env,
        admin: Address,
        decimals: u32,
        name: String,
        symbol: String,
    ) -> Result<(), LpTokenError> {
        todo!()
    }

    pub fn allowance(env: Env, from: Address, spender: Address) -> i128 {
        todo!()
    }

    pub fn approve(
        env: Env,
        from: Address,
        spender: Address,
        amount: i128,
        expiration_ledger: u32,
    ) -> Result<(), LpTokenError> {
        todo!()
    }

    pub fn balance(env: Env, id: Address) -> i128 {
        todo!()
    }

    pub fn transfer(
        env: Env,
        from: Address,
        to: Address,
        amount: i128,
    ) -> Result<(), LpTokenError> {
        todo!()
    }

    pub fn transfer_from(
        env: Env,
        spender: Address,
        from: Address,
        to: Address,
        amount: i128,
    ) -> Result<(), LpTokenError> {
        todo!()
    }

    pub fn mint(env: Env, to: Address, amount: i128) -> Result<(), LpTokenError> {
        todo!()
    }
    pub fn burn(env: Env, from: Address, amount: i128) -> Result<(), LpTokenError> {
        todo!()
    }

    pub fn decimals(env: Env) -> u32 {
        todo!()
    }
    pub fn name(env: Env) -> String {
        todo!()
    }
    pub fn symbol(env: Env) -> String {
        todo!()
    }
    pub fn total_supply(env: Env) -> i128 {
        todo!()
    }
}
