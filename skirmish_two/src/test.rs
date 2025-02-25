#![cfg(test)]

use soroban_sdk::{Address, Env, String, testutils::Address as _};

use crate::{SkirmishTwo, SkirmishTwoClient};

#[test]
fn test() {
    let env = Env::default();
    let contract_id = env.register(SkirmishTwo, ());
    let client = SkirmishTwoClient::new(&env, &contract_id);

    let word = client.run(&Address::generate(&env));
    let expected_value: String = String::from_str(&env, "1694-1727");
    assert_eq!(word, expected_value);
}
