use crate::{Contract, ContractClient};
use soroban_sdk::{Env, vec, symbol_short};

#[test]
fn test() {
    let env = Env::default();
    let contract_id = env.register_contract(None, Contract);
    let client = ContractClient::new(&env, &contract_id);

    let words = client.hello(&symbol_short!("Dev"));
    assert_eq!(words, vec![&env, symbol_short!("Hello"), symbol_short!("Dev")]);
}
