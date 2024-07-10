use super::*;

#[test]
fn happy_path() {
    let env = Env::default();
    env.mock_all_auths();

    let admin = Address::generate(&env);
    let (_, token_address) = helpers::create_token_contract(&env, &admin);
    let (contract, _) = helpers::create_and_initialize_contract(&env, &admin, &token_address);

    contract.pause();
    assert_eq!(contract.get_is_paused(), true);

    contract.resume();
    assert_eq!(contract.get_is_paused(), false);
}

#[test]
fn error_already_initialized() {
    let env = Env::default();
    env.mock_all_auths();

    let (contract, _) = helpers::create_contract(&env);

    assert_eq!(
        contract.try_pause(),
        Err(Ok(ContractError::NotInitialized {}))
    );
    assert_eq!(
        contract.try_resume(),
        Err(Ok(ContractError::NotInitialized {}))
    )
}
