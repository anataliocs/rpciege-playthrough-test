#![cfg(test)]

use soroban_sdk::{Address, Env, Symbol, testutils::Address as _};

use crate::{SkirmishThree, SkirmishThreeClient};

#[test]
#[should_panic]
fn test() {
    let env = Env::default();
    let contract_id = env.register(SkirmishThree, ());
    let client = SkirmishThreeClient::new(&env, &contract_id);

    client.game_3(&Symbol::new(&env, "pew"), &Some(Address::generate(&env)));
}

