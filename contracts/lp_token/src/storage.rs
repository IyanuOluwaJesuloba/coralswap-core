use soroban_sdk::{contracttype, Address};

#[contracttype]
#[derive(Clone, Debug)]
pub struct TokenMetadata {
    pub decimals: u32,
    pub name: [u8; 32],
    pub symbol: [u8; 8],
}

#[contracttype]
#[derive(Clone, Debug)]
pub struct AllowanceEntry {
    pub amount: i128,
    pub expiration_ledger: u32,
}

#[contracttype]
#[derive(Clone, Debug)]
pub enum LpTokenKey {
    Balance(Address),
    Allowance(Address, Address),
    TotalSupply,
    Metadata,
    Admin,
}
