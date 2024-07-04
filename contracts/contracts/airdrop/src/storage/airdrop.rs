use soroban_sdk::{contracttype, Env};

use crate::error::ContractError;

#[derive(Clone)]
#[contracttype]
enum DataKey {
    StartTime,
    EndTime,
    Amount,
    Paused,
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

pub fn get_stage_paused(env: &Env, stage: u32) -> Result<bool, ContractError> {
    env.storage().instance().get(&DataKey::Paused).unwrap()
}
