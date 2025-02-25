#![cfg(test)]

use soroban_sdk::{Address, Env, testutils::Address as _};

use crate::SkirmishOne;
use crate::SkirmishOneClient;

#[test]
fn test() {
    let env = Env::default();
    let contract_id: &Address = &env.register(SkirmishOne, ());
    let client = SkirmishOneClient::new(&env, &contract_id);

    client.run(&Address::generate(&env));
}
