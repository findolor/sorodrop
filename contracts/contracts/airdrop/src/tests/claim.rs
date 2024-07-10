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
        &None,
    );

    token_contract.mint(&contract_address, &test_data::TOTAL_AMOUNT);

    let balance =
        token_contract.balance(&test_data::get_account_address(&env, test_data::ACCOUNT_1));
    assert_eq!(balance, 0);

    contract.claim(
        &test_data::get_account_address(&env, test_data::ACCOUNT_1),
        &test_data::ACCOUNT_1_ALLOCATION,
        &test_data::get_merkle_proofs(&env, test_data::ACCOUNT_1),
    );

    let balance =
        token_contract.balance(&test_data::get_account_address(&env, test_data::ACCOUNT_1));
    assert_eq!(balance, test_data::ACCOUNT_1_ALLOCATION);
}

#[test]
fn error_not_initialized() {
    let env = Env::default();
    env.mock_all_auths();

    let (contract, _) = helpers::create_contract(&env);

    assert_eq!(
        contract.try_claim(
            &test_data::get_account_address(&env, test_data::ACCOUNT_1),
            &test_data::ACCOUNT_1_ALLOCATION,
            &test_data::get_merkle_proofs(&env, test_data::ACCOUNT_1),
        ),
        Err(Ok(ContractError::NotInitialized {}))
    )
}

#[test]
fn error_airdrop_not_begun() {
    let env = Env::default();
    env.mock_all_auths();

    let admin = Address::generate(&env);
    let (_, token_address) = helpers::create_token_contract(&env, &admin);
    let (contract, _) = helpers::create_and_initialize_contract(&env, admin, token_address);

    contract.register_airdrop(
        &test_data::get_merkle_root(&env),
        &test_data::TOTAL_AMOUNT,
        &Some(env.ledger().timestamp() + 100),
        &None,
    );

    assert_eq!(
        contract.try_claim(
            &test_data::get_account_address(&env, test_data::ACCOUNT_1),
            &test_data::ACCOUNT_1_ALLOCATION,
            &test_data::get_merkle_proofs(&env, test_data::ACCOUNT_1),
        ),
        Err(Ok(ContractError::AirdropNotBegun {}))
    )
}

#[test]
fn error_airdrop_expired() {
    let env = Env::default();
    env.mock_all_auths();

    let admin = Address::generate(&env);
    let (_, token_address) = helpers::create_token_contract(&env, &admin);
    let (contract, _) = helpers::create_and_initialize_contract(&env, admin, token_address);

    contract.register_airdrop(
        &test_data::get_merkle_root(&env),
        &test_data::TOTAL_AMOUNT,
        &None,
        &Some(50),
    );

    env.ledger().set_timestamp(100);

    assert_eq!(
        contract.try_claim(
            &test_data::get_account_address(&env, test_data::ACCOUNT_1),
            &test_data::ACCOUNT_1_ALLOCATION,
            &test_data::get_merkle_proofs(&env, test_data::ACCOUNT_1),
        ),
        Err(Ok(ContractError::AirdropExpired {}))
    )
}

#[test]
fn error_airdrop_paused() {
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

    contract.pause();

    assert_eq!(
        contract.try_claim(
            &test_data::get_account_address(&env, test_data::ACCOUNT_1),
            &test_data::ACCOUNT_1_ALLOCATION,
            &test_data::get_merkle_proofs(&env, test_data::ACCOUNT_1),
        ),
        Err(Ok(ContractError::AirdropPaused {}))
    )
}

#[test]
fn error_already_claimed() {
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
        &None,
    );

    token_contract.mint(&contract_address, &test_data::TOTAL_AMOUNT);

    contract.claim(
        &test_data::get_account_address(&env, test_data::ACCOUNT_1),
        &test_data::ACCOUNT_1_ALLOCATION,
        &test_data::get_merkle_proofs(&env, test_data::ACCOUNT_1),
    );

    assert_eq!(
        contract.try_claim(
            &test_data::get_account_address(&env, test_data::ACCOUNT_1),
            &test_data::ACCOUNT_1_ALLOCATION,
            &test_data::get_merkle_proofs(&env, test_data::ACCOUNT_1),
        ),
        Err(Ok(ContractError::AlreadyClaimed {}))
    )
}

#[test]
fn error_merkle_verification_failed() {
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
        &None,
    );

    token_contract.mint(&contract_address, &test_data::TOTAL_AMOUNT);

    assert_eq!(
        contract.try_claim(
            &test_data::get_account_address(&env, test_data::ACCOUNT_1),
            &test_data::ACCOUNT_1_ALLOCATION,
            &test_data::get_merkle_proofs(&env, test_data::ACCOUNT_2),
        ),
        Err(Ok(ContractError::MerkleVerificationFailed {}))
    )
}
