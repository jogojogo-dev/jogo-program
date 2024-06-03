use anchor_lang::prelude::*;

use crate::error::GameError;

#[account]
pub struct Club {
    pub admin: Pubkey,
    pub owner: Pubkey,
    pub supply_token_account: Pubkey,
    pub identifier: [u8; 32],

    pub liquidity: u64,
    pub locking_0: u64,
    pub locking_1: u64,
    pub locking_2: u64,
}

impl Club {
    pub const SIZE: usize = std::mem::size_of::<Self>();

    pub(crate) fn deposit(&mut self, amount: u64) -> Result<()> {
        require_gt!(amount, 0, GameError::InvalidDepositAmount);
        self.liquidity += amount;

        Ok(())
    }

    pub(crate) fn withdraw(&mut self, amount: u64) -> Result<()> {
        require_gt!(amount, 0, GameError::InvalidWithdrawAmount);
        require_gte!(
            self.liquidity,
            self.hedged_locking() + amount,
            GameError::InsufficientLiquidity,
        );
        self.liquidity -= amount;

        Ok(())
    }

    pub(crate) fn hedged_locking(&self) -> u64 {
        let lock_0 = self.locking_0.abs_diff(self.locking_1 + self.locking_2);
        let lock_1 = self.locking_1.abs_diff(self.locking_0 + self.locking_2);
        let lock_2 = self.locking_2.abs_diff(self.locking_0 + self.locking_1);
        lock_0.max(lock_1).max(lock_2)
    }

    pub(crate) fn bet(&mut self, direction: u8, stake: u64, lock: u64) -> Result<()> {
        match direction {
            0 => self.locking_0 += lock,
            1 => self.locking_1 += lock,
            2 => self.locking_2 += lock,
            _ => return Err(GameError::InvalidDirection.into()),
        }
        self.liquidity += stake;

        if self.liquidity < self.hedged_locking() {
            Err(GameError::InsufficientLiquidity.into())
        } else {
            Ok(())
        }
    }

    pub(crate) fn settle(
        &mut self,
        win: bool,
        bet_direction: u8,
        lock: u64,
    ) -> Result<()> {
        match bet_direction {
            0 => self.locking_0 -= lock,
            1 => self.locking_1 -= lock,
            2 => self.locking_2 -= lock,
            _ => return Err(GameError::InvalidDirection.into()),
        }

        if win {
            self.liquidity -= lock;
        }
        Ok(())
    }
}

#[account]
pub struct Credential {
    pub club: Pubkey,
    pub player: Pubkey,
    pub identifier: [u8; 32],

    pub direction: u8,
    pub stake: u64,
    pub lock: u64,
}

impl Credential {
    pub const SIZE: usize = std::mem::size_of::<Self>();
}
