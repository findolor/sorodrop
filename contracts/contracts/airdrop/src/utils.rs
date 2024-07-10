use sha2::Digest;
use soroban_sdk::{token, Address, BytesN, Env, Vec};

extern crate alloc;

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

pub fn is_airdrop_started(
    env: &Env,
    current_timestamp: u64,
) -> Result<Option<bool>, ContractError> {
    if let Some(start_time) = storage::airdrop::get_start_time(&env)? {
        return Ok(Some(current_timestamp >= start_time));
    }
    Ok(None)
}

pub fn is_airdrop_ended(env: &Env, current_timestamp: u64) -> Result<Option<bool>, ContractError> {
    if let Some(end_time) = storage::airdrop::get_end_time(&env)? {
        return Ok(Some(current_timestamp >= end_time));
    }
    Ok(None)
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

    match is_airdrop_ended(&env, current_timestamp)? {
        Some(true) => return Err(ContractError::AirdropExpired {}),
        Some(false) => {}
        None => return Err(ContractError::AirdropIsIndefinite),
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

pub fn verify_merkle_proofs(
    env: &Env,
    recipient: &Address,
    amount: &i128,
    merkle_proofs: Vec<BytesN<32>>,
) -> Result<(), ContractError> {
    let mut recipient_slice = [0u8; 56];
    recipient.to_string().copy_into_slice(&mut recipient_slice);
    let recipient_str = alloc::str::from_utf8(&recipient_slice)?;

    let input_formatted = alloc::format!("{}{}", recipient_str, amount);
    let input = input_formatted.as_bytes();

    let hash: [u8; 32] = sha2::Sha256::digest(input)
        .as_slice()
        .try_into()
        .map_err(|_| ContractError::HexError)?;

    let hash = merkle_proofs.into_iter().try_fold(hash, |hash, p| {
        let mut proof_buf: [u8; 32] = [0; 32];
        p.copy_into_slice(&mut proof_buf);

        let mut hashes = [hash, proof_buf];
        hashes.sort_unstable();
        sha2::Sha256::digest(&hashes.concat())
            .as_slice()
            .try_into()
            .map_err(|_| ContractError::MerkleVerificationFailed {})
    })?;

    let merkle_root = storage::airdrop::get_root(&env)?;

    let mut root_buf: [u8; 32] = [0; 32];
    merkle_root.copy_into_slice(&mut root_buf);

    if root_buf != hash {
        return Err(ContractError::MerkleVerificationFailed {});
    }

    Ok(())
}
