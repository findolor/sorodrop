use soroban_sdk::{contract, contractimpl, token, Address, BytesN, Env, Vec};

use crate::{
    error::ContractError,
    msg::{self, AirdropResponse, InvokeMsg, QueryMsg},
    storage,
    utils::{
        check_timestamp_validity, get_airdrop_amounts, is_airdrop_ended, is_airdrop_started,
        process_post_airdrop, verify_merkle_proofs,
    },
};

#[contract]
pub struct SorodropAirdrop;

#[contractimpl]
impl InvokeMsg for SorodropAirdrop {
    fn initialize(env: Env, admin: Address, token_address: Address) -> Result<(), ContractError> {
        if storage::config::has_config(&env) {
            return Err(ContractError::AlreadyInitialized {});
        }

        storage::config::set_config(&env, admin, token_address);

        Ok(())
    }

    fn update_config(
        env: Env,
        admin: Address,
        token_address: Address,
    ) -> Result<(), ContractError> {
        let config = storage::config::get_config(&env)?;

        config.admin.require_auth();

        storage::config::set_config(&env, admin, token_address);

        Ok(())
    }

    fn register_airdrop(
        env: Env,
        merkle_root: BytesN<32>,
        total_amount: i128,
        start_time: Option<u64>,
        end_time: Option<u64>,
    ) -> Result<(), ContractError> {
        let config = storage::config::get_config(&env)?;
        config.admin.require_auth();

        storage::airdrop::set_root(&env, merkle_root);

        check_timestamp_validity(&env, start_time, end_time)?;

        storage::airdrop::set_start_time(&env, start_time);
        storage::airdrop::set_end_time(&env, end_time);

        storage::airdrop::set_paused(&env, false);
        storage::airdrop::set_amount(&env, total_amount);

        storage::amount::set_admin_claim(&env, 0);
        storage::amount::set_total_claimed(&env, 0);
        storage::amount::set_burned(&env, 0);

        Ok(())
    }

    fn claim(
        env: Env,
        recipient: Address,
        amount: i128,
        merkle_proofs: Vec<BytesN<32>>,
    ) -> Result<(), ContractError> {
        storage::config::get_config(&env)?;

        recipient.require_auth();

        if storage::airdrop::get_paused(&env)? {
            return Err(ContractError::AirdropPaused {});
        }

        let current_timestamp = env.ledger().timestamp();
        if let Some(is_airdrop_started) = is_airdrop_started(&env, current_timestamp)? {
            if !is_airdrop_started {
                return Err(ContractError::AirdropNotBegun {});
            }
        }
        if let Some(is_airdrop_ended) = is_airdrop_ended(&env, current_timestamp)? {
            if is_airdrop_ended {
                return Err(ContractError::AirdropExpired {});
            }
        }

        if storage::amount::get_user_claim(&env, recipient.clone()).is_ok() {
            return Err(ContractError::AlreadyClaimed {});
        }

        verify_merkle_proofs(&env, &recipient, &amount, merkle_proofs)?;

        let total_claimed = storage::amount::get_total_claimed(&env)?;
        storage::amount::set_total_claimed(&env, total_claimed + amount);

        storage::amount::set_user_claim(&env, recipient.clone(), amount);

        let config: storage::config::Config = storage::config::get_config(&env)?;
        let token_contract = token::Client::new(&env, &config.token_address);
        token_contract.transfer(&env.current_contract_address(), &recipient, &amount);

        Ok(())
    }

    fn burn(env: Env, amount: i128) -> Result<(), ContractError> {
        let config = storage::config::get_config(&env)?;

        config.admin.require_auth();

        let token_contract = token::Client::new(&env, &config.token_address);

        let burned_amount =
            process_post_airdrop(&env, &token_contract, msg::PostAirdropProcess::Burn, amount)?;

        storage::amount::set_burned(&env, burned_amount + amount);

        token_contract.burn(&env.current_contract_address(), &amount);

        Ok(())
    }

    fn clawback(env: Env, recipient: Address, amount: i128) -> Result<(), ContractError> {
        let config = storage::config::get_config(&env)?;

        config.admin.require_auth();

        let token_contract = token::Client::new(&env, &config.token_address);

        let admin_claim_amount = process_post_airdrop(
            &env,
            &token_contract,
            msg::PostAirdropProcess::Clawback,
            amount,
        )?;

        storage::amount::set_admin_claim(&env, admin_claim_amount + amount);

        token_contract.transfer(&env.current_contract_address(), &recipient, &amount);

        Ok(())
    }

    fn pause(env: Env) -> Result<(), ContractError> {
        let config = storage::config::get_config(&env)?;
        config.admin.require_auth();

        storage::airdrop::set_paused(&env, true);

        Ok(())
    }

    fn resume(env: Env) -> Result<(), ContractError> {
        let config = storage::config::get_config(&env)?;
        config.admin.require_auth();

        storage::airdrop::set_paused(&env, false);

        Ok(())
    }
}

#[contractimpl]
impl QueryMsg for SorodropAirdrop {
    fn get_config(env: Env) -> Result<storage::config::Config, ContractError> {
        Ok(storage::config::get_config(&env)?)
    }

    fn get_airdrop(env: Env) -> Result<AirdropResponse, ContractError> {
        Ok(AirdropResponse {
            merkle_root: storage::airdrop::get_root(&env)?,
            total_amount: storage::airdrop::get_amount(&env)?,
            start: storage::airdrop::get_start_time(&env)?,
            end: storage::airdrop::get_end_time(&env)?,
        })
    }

    fn get_is_claimed(env: Env, recipient: Address) -> Result<bool, ContractError> {
        Ok(storage::amount::get_user_claim(&env, recipient).is_ok())
    }

    fn get_total_claimed(env: Env) -> Result<i128, ContractError> {
        Ok(storage::amount::get_total_claimed(&env)?)
    }

    fn get_remaining_amount(env: Env) -> Result<i128, ContractError> {
        let (_, _, _, _, remaining_amount) = get_airdrop_amounts(&env)?;
        Ok(remaining_amount)
    }

    fn get_is_paused(env: Env) -> Result<bool, ContractError> {
        Ok(storage::airdrop::get_paused(&env)?)
    }
}
