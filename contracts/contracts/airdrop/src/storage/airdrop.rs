use soroban_sdk::{contracttype, BytesN, Env};

use crate::error::ContractError;

#[derive(Clone)]
#[contracttype]
enum DataKey {
    MerkleRoot,
    StartTime,
    EndTime,
    Amount,
    Paused,
}

/* MERKLE ROOT */

pub fn set_root(env: &Env, root: BytesN<32>) {
    env.storage().instance().set(&DataKey::MerkleRoot, &root);
}

pub fn get_root(env: &Env) -> Result<BytesN<32>, ContractError> {
    if !root_exists(env) {
        return Err(ContractError::MerkleRootNotFound);
    }
    env.storage().instance().get(&DataKey::MerkleRoot).unwrap()
}

pub fn root_exists(e: &Env) -> bool {
    e.storage().instance().has(&DataKey::MerkleRoot)
}

/* START TIME */

pub fn set_start_time(env: &Env, value: Option<u64>) {
    env.storage().instance().set(&DataKey::StartTime, &value);
}

pub fn get_start_time(env: &Env) -> Result<Option<u64>, ContractError> {
    env.storage().instance().get(&DataKey::StartTime).unwrap()
}

/* END TIME */

pub fn set_end_time(env: &Env, value: Option<u64>) {
    env.storage().instance().set(&DataKey::EndTime, &value);
}

pub fn get_end_time(env: &Env) -> Result<Option<u64>, ContractError> {
    env.storage().instance().get(&DataKey::EndTime).unwrap()
}

/* AMOUNT */

pub fn set_amount(env: &Env, value: i128) {
    env.storage().instance().set(&DataKey::Amount, &value);
}

pub fn get_amount(env: &Env) -> Result<i128, ContractError> {
    env.storage().instance().get(&DataKey::Amount).unwrap()
}

/* PAUSED */

pub fn set_paused(env: &Env, value: bool) {
    env.storage().instance().set(&DataKey::Paused, &value);
}

pub fn get_paused(env: &Env) -> Result<bool, ContractError> {
    env.storage().instance().get(&DataKey::Paused).unwrap()
}
