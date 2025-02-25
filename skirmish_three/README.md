# Skirmish Three

Morning dawns on day 3 of the RPCiege. For today’s quarrel we’re enacting a new tactic:
brute force. To win the battle you must submit an invocation to the game_3 function of the
CDUZZ624GWOCKRWO2POLWHRNM2YKOTBH7D6MXQIRDXQICJ46BXNM6JBQ contract.

```rust
#![no_std]
use soroban_sdk::{
    Address, contract, contracterror, contractimpl,
    Env, panic_with_error, Symbol, xdr::ToXdr,
};

#[contract]
pub struct Contract;
#[contracterror]
#[derive(Copy, Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
#[repr(u32)]
pub enum Error {
    MissingPew = 1,
    UsedPew = 2,
}
#[contractimpl]
impl Contract {
    pub fn run(
        env: Env,
        symbol: Symbol,
        _nft_dest: Option<Address>,
    ) -> Result<(), Error> {
        if env.storage().persistent().has(&symbol) {
            panic_with_error!(env, Error::UsedPew);
        }
        let bytes = symbol.clone().to_xdr(&env);
        let hash = env.crypto().sha256(&bytes);
        let mut i = 0;
        let mut has_pew = false;
        for v in hash.clone().iter() {
            if v == 112
                && hash.get(i + 1).unwrap_or(0) == 101
                && hash.get(i + 2).unwrap_or(0) == 119
            {
                has_pew = true;
            }
            i += 1;
        }
        if !has_pew {
            panic_with_error!(env, Error::MissingPew);
        } else {
            env.storage().persistent().set(&symbol, &true);
        }
        Ok(())
    }
}
```