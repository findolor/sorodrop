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
    AlreadyClaimed = 6,
    // Time errors
    InvalidStartTime = 4,
    InvalidEndTime = 5,
    // Airdrop errors
    AirdropNotBegun = 8,
    AirdropExpired = 9,
    AirdropNotExpired = 10,
    AirdropPaused = 11,
    AirdropNotPaused = 12,
    // Balance errors
    InsufficientBalance = 13,
}
