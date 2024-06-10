use anchor_lang::prelude::*;

use crate::error::SportsError;

#[account]
pub struct Club {
    pub initialized: bool,
    pub admin: Pubkey,
    pub owner: Pubkey,
    pub token_mint: Pubkey,
    pub identifier: [u8; 32],

    pub staking: u64,
    pub liquidity: u64,
    pub locking_0: u64,
    pub locking_1: u64,
    pub locking_2: u64,
}

impl Club {
    pub const SIZE: usize = std::mem::size_of::<Self>();

    pub(crate) fn can_close(&self) -> bool {
        self.staking == 0
            && self.locking_0 == 0
            && self.locking_1 == 0
            && self.locking_2 == 0
    }

    pub(crate) fn deposit(&mut self, amount: u64) -> Result<()> {
        require_gt!(amount, 0, SportsError::InvalidDepositAmount);
        self.liquidity += amount;

        Ok(())
    }

    pub(crate) fn withdraw(&mut self, amount: u64) -> Result<()> {
        require_gt!(amount, 0, SportsError::InvalidWithdrawAmount);
        self.liquidity -= amount;

        let insurance = self.hedged_locking().max(self.staking);
        require_gte!(self.liquidity, insurance, SportsError::InsufficientLiquidity);

        Ok(())
    }

    pub(crate) fn hedged_locking(&self) -> u64 {
        let lock_0 = self.locking_0.abs_diff(self.locking_1 + self.locking_2);
        let lock_1 = self.locking_1.abs_diff(self.locking_0 + self.locking_2);
        let lock_2 = self.locking_2.abs_diff(self.locking_0 + self.locking_1);
        lock_0.max(lock_1).max(lock_2)
    }

    pub(crate) fn close_game(&mut self, game: &Game) {
        self.staking -= game.staking;
    }

    pub(crate) fn bet(&mut self, direction: u8, stake: u64, lock: u64) -> Result<()> {
        match direction {
            0 => self.locking_0 += lock,
            1 => self.locking_1 += lock,
            2 => self.locking_2 += lock,
            _ => return Err(SportsError::InvalidDirection.into()),
        }
        self.staking += stake;
        self.liquidity += stake;
        require_gte!(self.liquidity, self.hedged_locking(), SportsError::InsufficientLiquidity);

        Ok(())
    }

    pub(crate) fn close_bet(&mut self, credential: &Credential) -> Result<()> {
        match credential.direction {
            0 => self.locking_0 -= credential.lock,
            1 => self.locking_1 -= credential.lock,
            2 => self.locking_2 -= credential.lock,
            _ => return Err(SportsError::InvalidDirection.into()),
        }
        self.staking -= credential.stake;
        self.liquidity -= credential.stake;

        Ok(())
    }

    pub(crate) fn settle(&mut self, win: bool, credential: &Credential) -> Result<()> {
        match credential.direction {
            0 => self.locking_0 -= credential.lock,
            1 => self.locking_1 -= credential.lock,
            2 => self.locking_2 -= credential.lock,
            _ => return Err(SportsError::InvalidDirection.into()),
        }
        if win {
            self.liquidity -= credential.lock;
        }

        Ok(())
    }
}

#[account]
pub struct Game {
    pub club: Pubkey,
    pub identifier: [u8; 32],
    pub staking: u64,
}

impl Game {
    pub const SIZE: usize = std::mem::size_of::<Self>();

    pub(crate) fn can_close(&self, cancel: bool) -> bool {
        if cancel {
            // must close all bets!
            self.staking == 0
        } else {
            true
        }
    }

    pub(crate) fn bet(&mut self, stake: u64) {
        self.staking += stake;
    }

    pub(crate) fn close_bet(&mut self, stake: u64) {
        self.staking -= stake;
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
