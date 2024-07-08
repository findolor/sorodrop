use soroban_sdk::{token, Env};

use crate::{error::ContractError, msg, storage};

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

pub fn get_airdrop_amounts(env: &Env) -> Result<(i128, i128, i128, i128, i128), ContractError> {
    let total_amount = storage::airdrop::get_amount(&env)?;
    let admin_claim = storage::amount::get_admin_claim(&env)?;
    let total_claimed = storage::amount::get_total_claimed(&env)?;
    let burned = storage::amount::get_burned(&env)?;
    Ok((
        total_amount,
        admin_claim,
        total_claimed,
        burned,
        total_amount - total_claimed - admin_claim - burned,
    ))
}

pub fn process_post_airdrop(
    env: &Env,
    token_contract: &token::TokenClient,
    process: msg::PostAirdropProcess,
    amount: i128,
) -> Result<i128, ContractError> {
    let current_timestamp = env.ledger().timestamp();
    if !is_airdrop_ended(&env, current_timestamp)? {
        return Err(ContractError::AirdropNotExpired {});
    }

    let (_, admin_claim_amount, _, burned_amount, remaining_amount) = get_airdrop_amounts(&env)?;
    if amount > remaining_amount {
        return Err(ContractError::InsufficientBalance {});
    }

    let balance = token_contract.balance(&env.current_contract_address());
    if balance < amount {
        return Err(ContractError::InsufficientBalance {});
    }

    match process {
        msg::PostAirdropProcess::Burn => Ok(burned_amount),
        msg::PostAirdropProcess::Clawback => Ok(admin_claim_amount),
    }
}
