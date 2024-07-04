use super::*;

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
