#! [no_std]
use soroban_sdk::{contractimpl, vec, Env, Symbol, Vec, contract, symbol_short};

#[contract]
pub struct Contract;

#[contractimpl]
impl Contract {
    pub fn hello(env: Env, to: Symbol) -> Vec<Symbol> {
        vec![&env, symbol_short!("Hello"), to]
    }
}


#[cfg(test)]
mod test;