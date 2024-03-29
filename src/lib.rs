#! [no_std]

use soroban_sdk::{contract, contractimpl, contracttype, symbol_short, token, vec, Address, Env, IntoVal, Symbol, Vec};

#[contracttype]
pub enum DataKey {
    Counter(Address),
}

#[contract]
pub struct Contract;
#[contract]
pub struct AtomicSwapContract;


#[contractimpl]
impl Contract {
    pub fn hello(env: Env, to: Symbol, user: Address) -> (Vec<Symbol>, u32) {
        user.require_auth();
        let key = DataKey::Counter(user.clone());
        let mut count = env.storage().persistent().get(&key).unwrap_or_default();
        count += 1;
        env.storage().persistent().set(&key, &count);
        (vec![&env, symbol_short!("Hello"), to], count)
    }
}
#[contractimpl]
impl AtomicSwapContract {
    // Swap token A for token B atomically. Settle for the minimum requested price
    // for each party (this is an arbitrary choice; both parties could have
    // received the full amount as well).
    pub fn swap(
        env: Env,
        a: Address,
        b: Address,
        token_a: Address,
        token_b: Address,
        amount_a: i128,
        min_b_for_a: i128,
        amount_b: i128,
        min_a_for_b: i128,
    ) {
        if amount_b < min_b_for_a {
            panic!("not enough token B for token A");
        }
        if amount_a < min_a_for_b {
            panic!("not enough token A for token B");
        }
        a.require_auth_for_args(
            (token_a.clone(), token_b.clone(), amount_a, min_b_for_a).into_val(&env),
        );
        b.require_auth_for_args(
            (token_b.clone(), token_a.clone(), amount_b, min_a_for_b).into_val(&env),
        );

        move_token(&env, &token_a, &a, &b, amount_a, min_a_for_b);
        move_token(&env, &token_b, &b, &a, amount_b, min_b_for_a);
    }
}

fn move_token(
    env: &Env,
    token: &Address,
    from: &Address,
    to: &Address,
    max_spend_amount: i128,
    transfer_amount: i128,
) {
    let token = token::Client::new(env, token);
    let contract_address = env.current_contract_address();

    token.transfer(from, &contract_address, &max_spend_amount);
    token.transfer(&contract_address, to, &transfer_amount);
    token.transfer(&contract_address,from,&(&max_spend_amount - &transfer_amount),);
}


mod admin;
mod allowance;
mod balance;
mod contract;
mod metadata;
mod storage_types;

#[cfg(test)]
mod test;
mod token_tests;

pub use crate::contract::TokenClient;