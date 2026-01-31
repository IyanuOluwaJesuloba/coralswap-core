use soroban_sdk::{Address, Env, Vec};
use crate::errors::FactoryError;

/// 2-of-3 multi-sig verification.
pub fn verify_multisig(
    _env: &Env, _signers: &Vec<Address>, _required: u32,
) -> Result<(), FactoryError> {
    todo!()
}
