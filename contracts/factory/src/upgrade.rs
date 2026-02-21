use crate::errors::FactoryError;
use soroban_sdk::{BytesN, Env};

/// Proposed a timelocked contract upgrade (72h delay).
pub fn propose_upgrade(_env: &Env, _new_wasm_hash: BytesN<32>) -> Result<(), FactoryError> {
    todo!()
}

/// Executed a previously proposed upgrade after timelock expiry.
pub fn execute_upgrade(_env: &Env) -> Result<(), FactoryError> {
    todo!()
}
