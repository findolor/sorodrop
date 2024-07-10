use core::str::Utf8Error;

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
    AirdropIsIndefinite = 13,
    // Balance errors
    InsufficientBalance = 14,
    // Merkle errors
    MerkleRootNotFound = 15,
    MerkleVerificationFailed = 16,
    // External errors
    HexError = 17,
    Utf8Error = 18,
}
impl From<HexError> for ContractError {
    fn from(_: HexError) -> Self {
        ContractError::HexError
    }
}
impl From<Utf8Error> for ContractError {
    fn from(_: Utf8Error) -> Self {
        ContractError::Utf8Error
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
pub enum HexError {
    /// An invalid character was found. Valid ones are: `0...9`, `a...f`
    /// or `A...F`.
    InvalidHexCharacter,

    /// A hex string's length needs to be even, as two digits correspond to
    /// one byte.
    OddLength,

    /// If the hex string is decoded into a fixed sized container, such as an
    /// array, the hex string's length * 2 has to match the container's
    /// length.
    InvalidStringLength,
}
