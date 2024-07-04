use soroban_sdk::{testutils::Address as _, Address, Env};

use crate::{
    contract::{SorodropAirdrop, SorodropAirdropClient},
    error::ContractError,
};

mod helpers;
mod initialize;
