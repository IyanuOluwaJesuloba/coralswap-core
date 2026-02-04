use soroban_sdk::{Address, Env};

pub struct PairEvents;

impl PairEvents {
    pub fn swap(
        _env: &Env, _sender: &Address,
        _amount_a_in: i128, _amount_b_in: i128,
        _amount_a_out: i128, _amount_b_out: i128,
        _fee_bps: u32, _to: &Address,
    ) { todo!() }

    pub fn mint(_env: &Env, _sender: &Address, _amount_a: i128, _amount_b: i128) { todo!() }
    pub fn burn(_env: &Env, _sender: &Address, _amount_a: i128, _amount_b: i128, _to: &Address) { todo!() }
    pub fn sync(_env: &Env, _reserve_a: i128, _reserve_b: i128) { todo!() }

    pub fn flash_loan(
        _env: &Env, _receiver: &Address,
        _amount_a: i128, _amount_b: i128, _fee_a: i128, _fee_b: i128,
    ) { todo!() }
}
