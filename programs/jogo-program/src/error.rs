use anchor_lang::prelude::*;

#[error_code]
#[derive(Eq, PartialEq)]
pub enum JogoError {
    #[msg("Invalid fraction")]
    InvalidFraction,
    #[msg("Fraction overflow")]
    FractionOverflow,
    #[msg("Failed to verify ED25519 instruction")]
    VerifyED25519InstructionFailure,
    #[msg("Failed to verify ED25519 signature")]
    VerifyED25519HeaderFailure,
    #[msg("Failed to verify ED25519 data")]
    VerifyEd25519DataFailure,
    
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
}