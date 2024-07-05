use soroban_sdk::{contract, contractimpl, Address, BytesN, Env, Vec};

use crate::{
    error::ContractError,
    msg::{AirdropResponse, InvokeMsg, QueryMsg},
    storage,
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
        start: Option<u64>,
        expiration: Option<u64>,
    ) -> Result<(), ContractError> {
        Ok(())
    }

    fn claim(
        env: Env,
        recipient: Address,
        amount: i128,
        merkle_proofs: Vec<BytesN<32>>,
    ) -> Result<(), ContractError> {
        Ok(())
    }

    fn burn(env: Env, amount: i128) -> Result<(), ContractError> {
        Ok(())
    }

    fn clawback(env: Env, recipient: Address, amount: i128) -> Result<(), ContractError> {
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
            merkle_root: BytesN::from_array(&env, &[0; 32]),
            total_amount: 0,
            start: None,
            expiration: None,
        })
    }

    fn get_is_claimed(env: Env, recipient: Address) -> Result<bool, ContractError> {
        Ok(storage::claim::get_user_claim(&env, recipient).is_ok())
    }

    fn get_total_claimed(env: Env) -> Result<i128, ContractError> {
        Ok(storage::claim::get_total_claimed(&env))
    }

    fn get_remaining_amount(env: Env) -> Result<i128, ContractError> {
        let total_amount = storage::airdrop::get_amount(&env)?;
        let total_claimed = storage::claim::get_total_claimed(&env);
        let admin_claim = storage::claim::get_admin_claim(&env)?;

        Ok(total_amount - total_claimed - admin_claim)
    }

    fn get_is_paused(env: Env) -> Result<bool, ContractError> {
        Ok(storage::airdrop::get_paused(&env)?)
    }
}
