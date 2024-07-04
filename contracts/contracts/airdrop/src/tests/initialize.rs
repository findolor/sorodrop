use super::*;

#[test]
fn happy_path() {
    let env = Env::default();
    let (contract, _) = helpers::create_contract(&env);

    let admin = Address::generate(&env);
    let token_address = Address::generate(&env);

    contract.initialize(&admin, &token_address);

    let config = contract.get_config();

    assert_eq!(config.admin, admin);
    assert_eq!(config.token_address, token_address);
}

#[test]
fn error_already_initialized() {
    let env = Env::default();
    let (contract, _) = helpers::create_contract(&env);

    let admin = Address::generate(&env);
    let token_address = Address::generate(&env);

    contract.initialize(&admin, &token_address);

    assert_eq!(
        contract.try_initialize(&admin, &token_address),
        Err(Ok(ContractError::AlreadyInitialized {}))
    );
}
