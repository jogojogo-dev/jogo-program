use anchor_lang::prelude::*;

#[error_code]
#[derive(Eq, PartialEq)]
pub enum JogoError {
    #[msg("Invalid fraction")]
    InvalidFraction,
    #[msg("Fraction overflow")]
    FractionOverflow,
    #[msg("Invalid Ed25519 instruction data")]
    InvalidEd25519Instruction,
    #[msg("Incorrect Ed25519 signer")]
    IncorrectEd25519Signer,
    #[msg("Invalid Ed25519 message")]
    InvalidEd25519Message,
    
    /// Vault Errors
    #[msg("Invalid deposit amount")]
    InvalidDepositAmount,
    #[msg("Invalid withdraw amount")]
    InvalidWithdrawAmount,
    #[msg("Insufficient liquidity")]
    InsufficientLiquidity,
    
    /// Crash Game Errors
    #[msg("Randomness not fulfilled")]
    RandomnessNotFulfilled,
    #[msg("Invalid winning rate")]
    InvalidWinningRate,
    #[msg("Invalid odd")]
    InvalidOdd,
    #[msg("Invalid point decimals")]
    InvalidPointDecimals,
    #[msg("Invalid stake amount")]
    InvalidStakeAmount,
    #[msg("Invalid bet message")]
    InvalidBetMessage,
    
    /// ThirdParty Game Errors
    #[msg("Invalid 3rd-party player message")]
    InvalidThirdPartyPlayerMessage,
    #[msg("Invalid 3rd-party player nonce")]
    InvalidThirdPartyPlayerNonce,
}