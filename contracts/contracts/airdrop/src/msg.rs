use soroban_sdk::{contracttype, Address, BytesN, Env, Vec};

use crate::{error::ContractError, storage};

pub trait InvokeMsg {
    fn initialize(env: Env, admin: Address, token_address: Address) -> Result<(), ContractError>;

    fn update_config(env: Env, admin: Address, token_address: Address)
        -> Result<(), ContractError>;

    fn register_airdrop(
        env: Env,
        merkle_root: BytesN<32>,
        total_amount: i128,
        start: Option<u64>,
        expiration: Option<u64>,
    ) -> Result<(), ContractError>;

    fn claim(
        env: Env,
        recipient: Address,
        amount: i128,
        merkle_proofs: Vec<BytesN<32>>,
    ) -> Result<(), ContractError>;

    fn burn(env: Env, amount: i128) -> Result<(), ContractError>;

    fn clawback(
        env: Env,
        recipient: Address,
        amount: i128,
    ) -> Result<(), ContractError>;

    fn pause(env: Env) -> Result<(), ContractError>;

    fn resume(env: Env) -> Result<(), ContractError>;
}

#[derive(Clone, Debug, PartialEq)]
#[contracttype]
pub struct AirdropResponse {
    pub merkle_root: BytesN<32>,
    pub total_amount: i128,
    pub start: Option<u64>,
    pub expiration: Option<u64>,
}

pub trait QueryMsg {
    fn get_config(env: Env) -> Result<storage::config::Config, ContractError>;

    fn get_airdrop(env: Env) -> Result<AirdropResponse, ContractError>;

    fn get_is_claimed(env: Env, recipient: Address) -> Result<bool, ContractError>;

    fn get_total_claimed(env: Env) -> Result<i128, ContractError>;

    fn get_remaining_amount(env: Env) -> Result<i128, ContractError>;

    fn get_is_paused(env: Env) -> Result<bool, ContractError>;
}
