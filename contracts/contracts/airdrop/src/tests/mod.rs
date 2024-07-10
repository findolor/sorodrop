use soroban_sdk::{
    testutils::{Address as _, Ledger},
    vec, Address, BytesN, Env, String, Vec,
};

use crate::{
    contract::{SorodropAirdrop, SorodropAirdropClient},
    error::ContractError,
};

mod test_data;

mod claim;
mod helpers;
mod initialize;
mod register_airdrop;
