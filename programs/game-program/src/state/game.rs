use anchor_lang::prelude::*;

use crate::error::GameError;

#[account]
pub struct Game {
    pub bump: [u8; 1],
    pub auth_bump: [u8; 1],

    pub admin: Pubkey,
    pub owner: Pubkey,
    pub supply_token_account: Pubkey,

    pub liquidity: u64,
    pub stake: u64,
    pub locked: u64,
}

impl Game {
    pub const SIZE: usize = std::mem::size_of::<Self>();

    pub(crate) fn deposit(&mut self, amount: u64) -> Result<()> {
        require_gt!(amount, 0, GameError::InvalidDepositAmount);
        self.liquidity += amount;

        Ok(())
    }

    pub(crate) fn withdraw(&mut self, amount: u64) -> Result<()> {
        require_gt!(amount, 0, GameError::InvalidWithdrawAmount);
        require_gte!(self.liquidity, amount, GameError::InsufficientLiquidity);
        self.liquidity -= amount;

        Ok(())
    }

    pub(crate) fn bet(&mut self, stake: u64, lock: u64) -> Result<()> {
        self.stake += stake;
        self.liquidity += stake;

        require_gte!(self.liquidity, lock, GameError::InsufficientLiquidity);
        self.liquidity -= lock;
        self.locked += lock;

        Ok(())
    }

    pub(crate) fn settle(&mut self, win: bool, stake: u64, lock: u64) -> Result<()> {
        require_gte!(self.stake, stake, GameError::InvalidStakeAmount);
        self.stake -= stake;

        require_gte!(self.locked, lock, GameError::InvalidLockAmount);
        self.locked -= lock;

        if !win {
            self.liquidity += lock;
        }
        Ok(())
    }
}