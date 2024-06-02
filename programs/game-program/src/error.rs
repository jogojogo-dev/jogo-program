use anchor_lang::prelude::*;

#[error_code]
#[derive(Eq, PartialEq)]
pub enum GameError {
    #[msg("Invalid deposit amount")]
    InvalidDepositAmount,
    #[msg("Invalid withdraw amount")]
    InvalidWithdrawAmount,
    #[msg("Invalid stake amount")]
    InvalidStakeAmount,
    #[msg("Invalid lock amount")]
    InvalidLockAmount,
    #[msg("Insufficient liquidity")]
    InsufficientLiquidity,
    
    // Player
    #[msg("Mismatched bet round")]
    MismatchedBetRound,
    #[msg("Mismatched settle round")]
    MismatchedSettleRound,
    #[msg("Invalid round")]
    InvalidRound,
    #[msg("Invalid reward amount")]
    InvalidRewardAmount,
}