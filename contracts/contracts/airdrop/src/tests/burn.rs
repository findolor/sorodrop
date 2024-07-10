use super::*;

#[test]
fn happy_path() {
    let env = Env::default();
    env.mock_all_auths();

    let admin = Address::generate(&env);
    let (token_contract, token_address) = helpers::create_token_contract(&env, &admin);
    let (contract, contract_address) =
        helpers::create_and_initialize_contract(&env, admin, token_address);

    contract.register_airdrop(
        &test_data::get_merkle_root(&env),
        &test_data::TOTAL_AMOUNT,
        &None,
        &Some(100),
    );
    token_contract.mint(&contract_address, &test_data::TOTAL_AMOUNT);
    contract.claim(
        &test_data::get_account_address(&env, test_data::ACCOUNT_1),
        &test_data::ACCOUNT_1_ALLOCATION,
        &test_data::get_merkle_proofs(&env, test_data::ACCOUNT_1),
    );

    env.ledger().set_timestamp(200);

    let burn_amount = 123782;
    contract.burn(&burn_amount);

    assert_eq!(
        contract.get_remaining_amount(),
        (test_data::TOTAL_AMOUNT - test_data::ACCOUNT_1_ALLOCATION - burn_amount)
    );
    assert_eq!(
        token_contract.balance(&contract_address),
        (test_data::TOTAL_AMOUNT - test_data::ACCOUNT_1_ALLOCATION - burn_amount)
    );
}

#[test]
fn error_not_initialized() {
    let env = Env::default();
    env.mock_all_auths();

    let (contract, _) = helpers::create_contract(&env);

    assert_eq!(
        contract.try_burn(&10),
        Err(Ok(ContractError::NotInitialized {}))
    )
}

#[test]
fn error_airdrop_not_expired() {
    let env = Env::default();
    env.mock_all_auths();

    let admin = Address::generate(&env);
    let (_, token_address) = helpers::create_token_contract(&env, &admin);
    let (contract, _) = helpers::create_and_initialize_contract(&env, admin, token_address);

    contract.register_airdrop(
        &test_data::get_merkle_root(&env),
        &test_data::TOTAL_AMOUNT,
        &None,
        &Some(100),
    );
    env.ledger().set_timestamp(50);

    assert_eq!(
        contract.try_burn(&10),
        Err(Ok(ContractError::AirdropNotExpired {}))
    )
}

#[test]
fn error_airdrop_is_indefinite() {
    let env = Env::default();
    env.mock_all_auths();

    let admin = Address::generate(&env);
    let (_, token_address) = helpers::create_token_contract(&env, &admin);
    let (contract, _) = helpers::create_and_initialize_contract(&env, admin, token_address);

    contract.register_airdrop(
        &test_data::get_merkle_root(&env),
        &test_data::TOTAL_AMOUNT,
        &None,
        &None,
    );

    assert_eq!(
        contract.try_burn(&10),
        Err(Ok(ContractError::AirdropIsIndefinite {}))
    )
}
