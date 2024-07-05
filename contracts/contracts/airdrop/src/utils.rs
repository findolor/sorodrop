use soroban_sdk::Env;

use crate::error::ContractError;

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
