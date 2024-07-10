use super::*;

#[test]
fn happy_path() {
    let env = Env::default();
    env.mock_all_auths();

    let admin = Address::generate(&env);
    let (_, token_address) = helpers::create_token_contract(&env, &admin);
    let (contract, _) = helpers::create_and_initialize_contract(&env, &admin, &token_address);

    let new_admin = Address::generate(&env);
    let new_token_address = Address::generate(&env);

    contract.update_config(&new_admin, &new_token_address);

    let config = contract.get_config();

    assert_eq!(config.admin, new_admin);
    assert_eq!(config.token_address, new_token_address);
}

#[test]
fn error_already_initialized() {
    let env = Env::default();
    env.mock_all_auths();

    let (contract, _) = helpers::create_contract(&env);

    assert_eq!(
        contract.try_update_config(&Address::generate(&env), &Address::generate(&env),),
        Err(Ok(ContractError::NotInitialized {}))
    )
}
