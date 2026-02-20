//! Unit tests for `PairEvents` — verifies that every event method publishes
//! the correct topics and data using `env.events().all()`.
//!
//! Each test:
//!   1. Registers a minimal stub contract so events have a valid `contract_id`.
//!   2. Calls the event method inside `env.as_contract()`.
//!   3. Asserts that exactly one event was emitted with the expected shape.

#![cfg(test)]

use soroban_sdk::{
    contract, contractimpl,
    testutils::{Address as _, Events as _},
    Address, Env, IntoVal, Symbol,
    symbol_short, vec,
};

use crate::events::PairEvents;

// ---------------------------------------------------------------------------
// Minimal stub contract — needed so event emissions have a contract context.
// ---------------------------------------------------------------------------
#[contract]
struct EventStub;

#[contractimpl]
impl EventStub {}

// ---------------------------------------------------------------------------
// swap
// ---------------------------------------------------------------------------
#[test]
fn swap_event_emits_correct_topics_and_data() {
    let env = Env::default();
    let contract_id = env.register_contract(None, EventStub);
    let sender = Address::generate(&env);
    let to = Address::generate(&env);

    env.as_contract(&contract_id, || {
        PairEvents::swap(&env, &sender, 100_i128, 0_i128, 0_i128, 99_i128, 30_u32, &to);
    });

    let all = env.events().all();
    assert_eq!(all.len(), 1, "expected exactly one swap event");

    assert_eq!(
        all,
        vec![
            &env,
            (
                contract_id,
                (symbol_short!("swap"), sender.clone()).into_val(&env),
                (100_i128, 0_i128, 0_i128, 99_i128, 30_u32, to.clone()).into_val(&env),
            )
        ]
    );
}

// ---------------------------------------------------------------------------
// mint
// ---------------------------------------------------------------------------
#[test]
fn mint_event_emits_correct_topics_and_data() {
    let env = Env::default();
    let contract_id = env.register_contract(None, EventStub);
    let sender = Address::generate(&env);

    env.as_contract(&contract_id, || {
        PairEvents::mint(&env, &sender, 1_000_i128, 2_000_i128);
    });

    let all = env.events().all();
    assert_eq!(all.len(), 1, "expected exactly one mint event");

    assert_eq!(
        all,
        vec![
            &env,
            (
                contract_id,
                (symbol_short!("mint"), sender.clone()).into_val(&env),
                (1_000_i128, 2_000_i128).into_val(&env),
            )
        ]
    );
}

// ---------------------------------------------------------------------------
// burn
// ---------------------------------------------------------------------------
#[test]
fn burn_event_emits_correct_topics_and_data() {
    let env = Env::default();
    let contract_id = env.register_contract(None, EventStub);
    let sender = Address::generate(&env);
    let to = Address::generate(&env);

    env.as_contract(&contract_id, || {
        PairEvents::burn(&env, &sender, 500_i128, 750_i128, &to);
    });

    let all = env.events().all();
    assert_eq!(all.len(), 1, "expected exactly one burn event");

    assert_eq!(
        all,
        vec![
            &env,
            (
                contract_id,
                (symbol_short!("burn"), sender.clone()).into_val(&env),
                (500_i128, 750_i128, to.clone()).into_val(&env),
            )
        ]
    );
}

// ---------------------------------------------------------------------------
// sync
// ---------------------------------------------------------------------------
#[test]
fn sync_event_emits_correct_topics_and_data() {
    let env = Env::default();
    let contract_id = env.register_contract(None, EventStub);

    env.as_contract(&contract_id, || {
        PairEvents::sync(&env, 10_000_i128, 20_000_i128);
    });

    let all = env.events().all();
    assert_eq!(all.len(), 1, "expected exactly one sync event");

    assert_eq!(
        all,
        vec![
            &env,
            (
                contract_id,
                (symbol_short!("sync"),).into_val(&env),
                (10_000_i128, 20_000_i128).into_val(&env),
            )
        ]
    );
}

// ---------------------------------------------------------------------------
// flash_loan
// ---------------------------------------------------------------------------
#[test]
fn flash_loan_event_emits_correct_topics_and_data() {
    let env = Env::default();
    let contract_id = env.register_contract(None, EventStub);
    let receiver = Address::generate(&env);

    env.as_contract(&contract_id, || {
        PairEvents::flash_loan(&env, &receiver, 5_000_i128, 0_i128, 25_i128, 0_i128);
    });

    let all = env.events().all();
    assert_eq!(all.len(), 1, "expected exactly one flash_loan event");

    assert_eq!(
        all,
        vec![
            &env,
            (
                contract_id,
                (Symbol::new(&env, "flash_loan"), receiver.clone()).into_val(&env),
                (5_000_i128, 0_i128, 25_i128, 0_i128).into_val(&env),
            )
        ]
    );
}

// ---------------------------------------------------------------------------
// Guard: multiple events stay independent (no cross-contamination)
// ---------------------------------------------------------------------------
#[test]
fn multiple_events_are_independent() {
    let env = Env::default();
    let contract_id = env.register_contract(None, EventStub);
    let sender = Address::generate(&env);

    env.as_contract(&contract_id, || {
        PairEvents::sync(&env, 100_i128, 200_i128);
        PairEvents::mint(&env, &sender, 10_i128, 20_i128);
    });

    let all = env.events().all();
    assert_eq!(all.len(), 2, "expected two events in order");
}
