use anchor_lang::prelude::*;

#[error_code]
#[derive(Eq, PartialEq)]
pub enum GameError {
    #[msg("Invalid operator")]
    InvalidOperator,

    #[msg("Invalid direction")]
    InvalidDirection,

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
    
    // Game
    #[msg("Direction not equals")]
    DirectionNotEquals,
}