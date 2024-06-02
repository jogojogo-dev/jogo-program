use anchor_lang::prelude::*;

use crate::error::GameError;

#[account]
pub struct PlayerState {
    pub game: Pubkey,
    pub owner: Pubkey,
    
    pub bet_round: u64,
    pub settle_round: u64,
    
    pub stake: u64,
    pub lock: u64,
    pub reward: u64,
}

impl PlayerState {
    pub const SIZE: usize = std::mem::size_of::<Self>();

    #[inline]
    fn check_round(&self) -> Result<()> {
        require_eq!(self.bet_round, self.settle_round + 1, GameError::InvalidRound);
        Ok(())
    }

    pub(crate) fn bet(&mut self, round: u64, stake: u64, lock: u64, reward: u64) -> Result<()> {
        self.check_round()?;
        require_eq!(round, self.bet_round, GameError::MismatchedBetRound);
        require_gt!(stake, 0, GameError::InvalidStakeAmount);
        require_gt!(reward, stake, GameError::InvalidRewardAmount);

        self.bet_round += 1;
        self.stake = stake;
        self.lock = lock;
        self.reward = reward;

        Ok(())
    }

    pub(crate) fn settle(&mut self, round: u64) -> Result<()> {
        self.check_round()?;
        require_eq!(round, self.settle_round, GameError::MismatchedSettleRound);
        
        self.settle_round += 1;
        self.stake = 0;
        self.lock = 0;
        self.reward = 0;

        Ok(())
    }
}
