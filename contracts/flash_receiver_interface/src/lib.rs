#![no_std]

use soroban_sdk::{contractclient, Address, Bytes, Env};

/// Flash Loan Receiver Interface.
/// Contracts receiving flash loans must implement this trait.
/// The Pair contract invokes `on_flash_loan` after token transfer.
/// Receiver MUST repay principal + fee before the callback returns.
#[contractclient(name = "FlashReceiverClient")]
pub trait FlashReceiver {
    fn on_flash_loan(
        env: Env,
        initiator: Address,
        token_a: Address,
        token_b: Address,
        amount_a: i128,
        amount_b: i128,
        fee_a: i128,
        fee_b: i128,
        data: Bytes,
    );
}
