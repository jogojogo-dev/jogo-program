use anchor_lang::prelude::*;

use crate::error::JogoError;

#[account]
pub struct Vault {
    pub admin: Pubkey,
    pub lp_token_mint: Pubkey,
    pub supply_chip_account: Pubkey,

    pub liquidity: u64,
    pub stake: u64,
    pub reserve: u64,
    pub minted_lp: u64,
}

impl Vault {
    pub const SIZE: usize = std::mem::size_of::<Self>();

    fn supply(&self) -> u64 {
        self.liquidity + self.reserve - self.stake
    }

    pub(crate) fn deposit(&mut self, amount: u64) -> Result<u64> {
        if amount == 0 {
            return Err(JogoError::InvalidDepositAmount.into());
        }

        let supply = self.supply();
        let mint_lp = if supply > 0 {
            (self.minted_lp as u128 * amount as u128 / supply as u128) as u64
        } else {
            amount
        };
        self.liquidity += amount;
        self.minted_lp += mint_lp;

        Ok(mint_lp)
    }

    pub(crate) fn withdraw(&mut self, amount: u64) -> Result<u64> {
        if amount == 0 {
            return Err(JogoError::InvalidWithdrawAmount.into());
        }

        let supply = self.supply();
        let withdrawal = (supply as u128 * amount as u128 / self.minted_lp as u128) as u64;
        if withdrawal > self.liquidity {
            return Err(JogoError::InsufficientLiquidity.into());
        }
        self.liquidity -= withdrawal;
        self.minted_lp -= amount;

        Ok(withdrawal)
    }

    pub(crate) fn bet(&mut self, stake: u64, reserve: u64) -> Result<()> {
        self.stake += stake;
        self.liquidity += stake;
        if reserve > self.liquidity {
            return Err(JogoError::InsufficientLiquidity.into());
        }
        self.liquidity -= reserve;
        self.reserve += reserve;

        Ok(())
    }

    pub(crate) fn settle(&mut self, stake: u64, reserve: u64, winning: u64) {
        self.stake -= stake;
        self.liquidity += reserve - winning;
        self.reserve -= reserve;
    }
}
