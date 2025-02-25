#![no_std]

use soroban_sdk::{Address, contract, contractimpl, Env, String};

#[contract]
pub struct SkirmishTwo;

#[contractimpl]
impl SkirmishTwo {
    pub fn run(_env: &Env, _nft_dest: Address) -> String {
        String::from_str(&_env, "1694-1727")
    }
}

mod test;
