use soroban_sdk::{contracttype, Address, Env};

use crate::error::ContractError;

#[derive(Clone)]
#[contracttype]
enum DataKey {
    Config,
}

#[derive(Clone, Debug, PartialEq)]
#[contracttype]
pub struct Config {
    pub admin: Address,
    pub token_address: Address,
}

pub fn set_config(env: &Env, admin: Address, token_address: Address) {
    env.storage().instance().set(
        &DataKey::Config,
        &Config {
            admin,
            token_address,
        },
    );
}

pub fn get_config(env: &Env) -> Result<Config, ContractError> {
    if !has_config(env) {
        return Err(ContractError::NotInitialized);
    }
    env.storage().instance().get(&DataKey::Config).unwrap()
}

pub fn update_config(env: &Env, admin: Address, token_address: Address) {
    env.storage().instance().set(
        &DataKey::Config,
        &Config {
            admin,
            token_address,
        },
    );
}

pub fn has_config(e: &Env) -> bool {
    e.storage().instance().has(&DataKey::Config)
}
