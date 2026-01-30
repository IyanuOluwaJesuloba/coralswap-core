use soroban_sdk::{Address, Env};

pub struct FactoryEvents;

impl FactoryEvents {
    pub fn pair_created(
        _env: &Env, _token_a: &Address, _token_b: &Address,
        _pair: &Address, _pair_index: u32,
    ) { todo!() }

    pub fn paused(_env: &Env) { todo!() }
    pub fn unpaused(_env: &Env) { todo!() }
    pub fn upgrade_proposed(_env: &Env, _new_wasm_hash: &[u8; 32]) { todo!() }
    pub fn upgrade_executed(_env: &Env, _new_version: u32) { todo!() }
}
