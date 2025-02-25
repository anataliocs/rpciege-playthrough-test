#![no_std]

use soroban_sdk::{Address, contract, contractimpl, Env};

#[contract]
pub struct SkirmishOne;

#[contractimpl]
impl SkirmishOne {
    pub fn run(_env: &Env, _nft_dest: Address) {}
}

mod test;
