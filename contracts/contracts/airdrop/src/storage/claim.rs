use soroban_sdk::{contracttype, Address, Env};

use crate::error::ContractError;

#[derive(Clone)]
#[contracttype]
enum DataKey {
    UserClaim(Address),
    AdminClaim,
    TotalClaimed,
}

// ========== USER CLAIM ==========

pub fn set_user_claim(env: &Env, address: Address, amount: i128) {
    env.storage()
        .instance()
        .set(&DataKey::UserClaim(address), &amount);
}

pub fn get_user_claim(env: &Env, address: Address) -> Result<i128, ContractError> {
    if !check_user_claim(env, address.clone()) {
        return Err(ContractError::UserClaimNotFound);
    }
    env.storage()
        .instance()
        .get(&DataKey::UserClaim(address))
        .unwrap()
}

pub fn check_user_claim(e: &Env, address: Address) -> bool {
    e.storage().instance().has(&DataKey::UserClaim(address))
}

// ========== TOTAL CLAIM ==========

pub fn set_total_claimed(env: &Env, amount: i128) {
    env.storage()
        .instance()
        .set(&DataKey::TotalClaimed, &amount);
}

pub fn get_total_claimed(env: &Env) -> i128 {
    env.storage()
        .instance()
        .get(&DataKey::TotalClaimed)
        .unwrap()
}

// ========== ADMIN CLAIM ==========

pub fn set_admin_claim(env: &Env, amount: i128) {
    env.storage().instance().set(&DataKey::AdminClaim, &amount);
}

pub fn get_admin_claim(env: &Env) -> Result<i128, ContractError> {
    env.storage().instance().get(&DataKey::AdminClaim).unwrap()
}
