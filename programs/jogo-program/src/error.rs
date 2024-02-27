use anchor_lang::prelude::*;

#[error_code]
#[derive(Eq, PartialEq)]
pub enum JogoError {
    #[msg("Invalid fraction")]
    InvalidFraction,
    #[msg("Fraction overflow")]
    FractionOverflow,
    #[msg("Randomness not fulfilled")]
    RandomnessNotFulfilled,
    #[msg("Randomness already set")]
    RandomnessAlreadySet,
    #[msg("Randomness not set")]
    RandomnessNotSet,
    #[msg("Invalid deposit amount")]
    InvalidDepositAmount,
    #[msg("Invalid withdraw amount")]
    InvalidWithdrawAmount,
    #[msg("Insufficient liquidity")]
    InsufficientLiquidity,

    /// Crash Game Errors
    #[msg("No bet signature")]
    NoBetSignature,
    #[msg("Invalid winning rate")]
    InvalidWinningRate,
    #[msg("Invalid odd")]
    InvalidOdd,
    #[msg("Invalid stake amount")]
    InvalidStakeAmount,
    #[msg("Bet disabled")]
    BetDisabled,
}