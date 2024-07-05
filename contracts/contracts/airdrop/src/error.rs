use soroban_sdk::contracterror;

#[contracterror]
#[derive(Copy, Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
#[repr(u32)]
pub enum ContractError {
    // Initialization errors
    NotInitialized = 1,
    AlreadyInitialized = 2,
    // Claim errors
    UserClaimNotFound = 3,
    // Time period errors
    InvalidStartTime = 4,
    InvalidEndTime = 5,
}
