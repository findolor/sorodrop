use super::*;

#[test]
fn happy_path() {
    let env = Env::default();
    env.mock_all_auths();

    let admin = Address::generate(&env);
    let (contract, _) =
        helpers::create_and_initialize_contract(&env, &admin, &Address::generate(&env));

    contract.register_airdrop(
        &test_data::get_merkle_root(&env),
        &test_data::TOTAL_AMOUNT,
        &None,
        &None,
    );

    let response = contract.get_airdrop();

    assert_eq!(response.merkle_root, test_data::get_merkle_root(&env));
    assert_eq!(response.total_amount, test_data::TOTAL_AMOUNT);
    assert_eq!(response.start, None);
    assert_eq!(response.end, None);
}

#[test]
fn error_not_initialized() {
    let env = Env::default();
    env.mock_all_auths();

    let (contract, _) = helpers::create_contract(&env);

    assert_eq!(
        contract.try_register_airdrop(
            &test_data::get_merkle_root(&env),
            &test_data::TOTAL_AMOUNT,
            &None,
            &None,
        ),
        Err(Ok(ContractError::NotInitialized {}))
    )
}

#[test]
fn error_invalid_start_time() {
    let env = Env::default();
    env.mock_all_auths();

    env.ledger().set_timestamp(100);

    let admin = Address::generate(&env);
    let (contract, _) =
        helpers::create_and_initialize_contract(&env, &admin, &Address::generate(&env));

    assert_eq!(
        contract.try_register_airdrop(
            &test_data::get_merkle_root(&env),
            &test_data::TOTAL_AMOUNT,
            &Some(50),
            &None,
        ),
        Err(Ok(ContractError::InvalidStartTime {}))
    );
    assert_eq!(
        contract.try_register_airdrop(
            &test_data::get_merkle_root(&env),
            &test_data::TOTAL_AMOUNT,
            &Some(100),
            &None,
        ),
        Err(Ok(ContractError::InvalidStartTime {}))
    )
}

#[test]
fn error_invalid_expiration_time() {
    let env = Env::default();
    env.mock_all_auths();

    env.ledger().set_timestamp(100);

    let admin = Address::generate(&env);
    let (contract, _) =
        helpers::create_and_initialize_contract(&env, &admin, &Address::generate(&env));

    assert_eq!(
        contract.try_register_airdrop(
            &test_data::get_merkle_root(&env),
            &test_data::TOTAL_AMOUNT,
            &None,
            &Some(50),
        ),
        Err(Ok(ContractError::InvalidEndTime {}))
    );
    assert_eq!(
        contract.try_register_airdrop(
            &test_data::get_merkle_root(&env),
            &test_data::TOTAL_AMOUNT,
            &None,
            &Some(100),
        ),
        Err(Ok(ContractError::InvalidEndTime {}))
    )
}
