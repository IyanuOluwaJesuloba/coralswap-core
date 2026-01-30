use soroban_sdk::{contracttype, Address, BytesN};

#[contracttype]
#[derive(Clone, Debug)]
pub struct FactoryStorage {
    pub signers: [Address; 3],
    pub pair_wasm_hash: BytesN<32>,
    pub lp_token_wasm_hash: BytesN<32>,
    pub pair_count: u32,
    pub protocol_version: u32,
    pub paused: bool,
}

#[contracttype]
#[derive(Clone, Debug)]
pub struct TimelockedAction {
    pub proposed_at: u64,
    pub delay_seconds: u64,
    pub action_id: u32,
}
