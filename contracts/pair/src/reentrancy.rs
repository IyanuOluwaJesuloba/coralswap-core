use soroban_sdk::Env;
use crate::errors::PairError;

/// Acquired the reentrancy lock. Reverts with Locked if already held.
pub fn acquire(_env: &Env) -> Result<(), PairError> {
    todo!()
}

/// Released the reentrancy lock after flash loan verification.
pub fn release(_env: &Env) {
    todo!()
}
