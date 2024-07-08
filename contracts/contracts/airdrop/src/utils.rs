use soroban_sdk::Env;

use crate::{error::ContractError, storage};

pub fn check_timestamp_validity(
    env: &Env,
    start: Option<u64>,
    expiration: Option<u64>,
) -> Result<(), ContractError> {
    if let Some(start_time) = start {
        let current_timestamp = env.ledger().timestamp();
        if current_timestamp >= start_time {
            return Err(ContractError::InvalidStartTime {});
        }
    }
    if let Some(expiration_time) = expiration {
        let current_timestamp = env.ledger().timestamp();
        if current_timestamp >= expiration_time {
            return Err(ContractError::InvalidEndTime {});
        }
    }
    Ok(())
}

pub fn is_airdrop_started(env: &Env, current_timestamp: u64) -> Result<bool, ContractError> {
    if let Some(start_time) = storage::airdrop::get_start_time(&env)? {
        if current_timestamp < start_time {
            return Ok(false);
        }
    }
    Ok(true)
}

pub fn is_airdrop_ended(env: &Env, current_timestamp: u64) -> Result<bool, ContractError> {
    if let Some(end_time) = storage::airdrop::get_end_time(&env)? {
        if current_timestamp < end_time {
            return Ok(false);
        }
    }
    Ok(true)
}

pub fn get_airdrop_amounts(env: &Env) -> Result<(i128, i128, i128, i128), ContractError> {
    let total_amount = storage::airdrop::get_amount(&env)?;
    let total_claimed = storage::claim::get_total_claimed(&env);
    let admin_claim = storage::claim::get_admin_claim(&env)?;
    Ok((
        total_amount,
        total_claimed,
        admin_claim,
        total_amount - total_claimed - admin_claim,
    ))
}
