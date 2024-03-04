use anchor_lang::prelude::*;

#[error_code]
#[derive(Eq, PartialEq)]
pub enum JogoError {
    #[msg("Invalid fraction")]
    InvalidFraction,
    #[msg("Fraction overflow")]
    FractionOverflow,
    #[msg("Invalid ED25519 instruction")]
    InvalidED25519Instruction,
    #[msg("Incorrect ED25519 signature Signer")]
    IncorrectED25519Signer,
    #[msg("Invalid ED25519 message")]
    InvalidED25519Message,
    
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
    #[msg("No bet signature")]
    NoBetSignature,
    #[msg("Invalid winning rate")]
    InvalidWinningRate,
    #[msg("Invalid odd")]
    InvalidOdd,
    #[msg("Invalid stake amount")]
    InvalidStakeAmount,
    #[msg("Invalid bet message")]
    InvalidBetMessage,
}