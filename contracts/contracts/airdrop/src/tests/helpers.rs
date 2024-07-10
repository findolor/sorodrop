use super::*;

mod token {
    soroban_sdk::contractimport!(file = "../../external_wasm/soroban_token_contract.wasm");
    pub type TokenClient<'a> = Client<'a>;
}
use token::TokenClient;

pub fn create_contract(e: &Env) -> (SorodropAirdropClient, Address) {
    let address = &e.register_contract(None, SorodropAirdrop {});
    (SorodropAirdropClient::new(&e, address), address.clone())
}

pub fn create_and_initialize_contract(
    e: &Env,
    admin: Address,
    token_address: Address,
) -> (SorodropAirdropClient, Address) {
    let address = &e.register_contract(None, SorodropAirdrop {});
    let contract = SorodropAirdropClient::new(&e, address);
    contract.initialize(&admin, &token_address);
    (contract, address.clone())
}

pub fn create_token_contract<'a>(e: &Env, admin: &Address) -> (TokenClient<'a>, Address) {
    let address = e.register_contract_wasm(None, token::WASM);
    let client = TokenClient::new(&e, &address);

    client.initialize(
        admin,
        &7,
        &String::from_str(&e, "Test Token"),
        &String::from_str(&e, "TT"),
    );

    (client, address)
}
