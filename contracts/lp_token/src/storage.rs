use soroban_sdk::{contracttype, Address, String};

#[contracttype]
#[derive(Clone, Debug)]
pub struct TokenMetadata {
    pub decimals: u32,
    pub name: String,
    pub symbol: String,
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
