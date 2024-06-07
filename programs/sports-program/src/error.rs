use anchor_lang::prelude::*;

#[error_code]
#[derive(Eq, PartialEq)]
pub enum SportsError {
    #[msg("Invalid operator")]
    InvalidOperator,

    // Club
    #[msg("Cannot close club")]
    CannotCloseClub,
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
    #[msg("Settle is active")]
    SettleActive,
    
    // Game
    #[msg("Invalid direction")]
    InvalidDirection,
}