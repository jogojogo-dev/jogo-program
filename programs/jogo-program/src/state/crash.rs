use anchor_lang::prelude::*;
use solana_program::{clock::UnixTimestamp, hash::hash};
use orao_solana_vrf::state::Randomness;

use crate::{error::JogoError, math::Fraction};

#[account]
pub struct CrashGame {
    pub vault: Pubkey,
    pub operator: Pubkey,
    pub win_rate: Fraction,
    pub max_odd: Fraction,
    pub next_round: u64,
}

impl CrashGame {
    pub const SIZE: usize = std::mem::size_of::<Self>();

    pub(crate) fn new(
        vault: Pubkey,
        operator: Pubkey,
        win_rate: Fraction,
        max_odd: Fraction,
    ) -> Result<Self> {
        if win_rate >= Fraction::one() || win_rate == Fraction::zero() {
            return Err(JogoError::InvalidWinningRate.into());
        }
        if max_odd <= Fraction::one() {
            return Err(JogoError::InvalidOdd.into());
        }

        Ok(Self {
            vault,
            operator,
            win_rate,
            max_odd,
            next_round: 0,
        })
    }

    pub(crate) fn set_operator(&mut self, operator: Pubkey) {
        self.operator = operator;
    }

    pub(crate) fn set_win_rate(&mut self, win_rate: Fraction) -> Result<()> {
        if (win_rate >= Fraction::one() || win_rate == Fraction::zero()) {
            return Err(JogoError::InvalidWinningRate.into());
        }
        self.win_rate = win_rate;

        Ok(())
    }

    pub(crate) fn set_max_odd(&mut self, max_odd: Fraction) -> Result<()> {
        if (max_odd <= Fraction::one()) {
            return Err(JogoError::InvalidOdd.into());
        }
        self.max_odd = max_odd;

        Ok(())
    }

    pub(crate) fn lock(
        &mut self,
        game: Pubkey,
        bump: u8,
        randomness: &Randomness,
    ) -> Result<CrashLock> {
        if let Some(randomness) = randomness.fulfilled() {
            let lock = CrashLock {
                game,
                bump,
                round: self.next_round,
                randomness: randomness.clone(),
            };
            self.next_round += 1;

            Ok(lock)
        } else {
            Err(JogoError::RandomnessNotFulfilled.into())
        }
    }

    pub(crate) fn bet(
        &self,
        owner: Pubkey,
        lock: Pubkey,
        bump: u8,
        stake: u64,
        point: Option<Fraction>,
    ) -> Result<CrashBet> {
        if stake == 0 {
            Err(JogoError::InvalidStakeAmount.into())
        } else {
            Ok(CrashBet {
                owner,
                lock,
                bump,
                stake,
                reserve: self.max_odd.mul_u64(stake),
                point,
            })
        }
    }

    pub(crate) fn crash_point(&self, randomness_sig: &[u8]) -> Result<Fraction> {
        let sig_hash = hash(randomness_sig).to_bytes();
        let final_rand = u32::from_le_bytes(
            <[u8; 4]>::try_from(sig_hash[..4]).unwrap()
        );
        let scale = Fraction::new(1u64 << 32, final_rand as u64 + 1)?;
        self.win_rate.try_mul(scale)
    }
}

#[account]
pub struct CrashLock {
    pub game: Pubkey,
    pub bump: u8,
    pub round: u64,
    pub randomness: [u8; 64],
}

impl CrashLock {
    pub const SIZE: usize = std::mem::size_of::<Self>();
}

#[account]
pub struct CrashBet {
    pub owner: Pubkey,
    pub lock: Pubkey,
    pub bump: u8,
    pub stake: u64,
    pub reserve: u64,
    pub point: Option<Fraction>,
}

impl CrashBet {
    pub const SIZE: usize = 1 + std::mem::size_of::<Self>();

    pub fn message(&self, point: Fraction) -> [u8; 80] {
        let mut data = [0u8; 80];
        data[..32].copy_from_slice(self.owner.as_ref());
        data[32..64].copy_from_slice(self.lock.as_ref());
        point.pack_into(&mut data[64..]);
        data
    }

    pub(crate) fn settle(
        &self,
        point: Option<Fraction>,
        crash_point: Fraction,
    ) -> u64 {
        if let Some(point) = point.or(self.point) {
            if point <= crash_point {
                return point.mul_u64(self.stake).min(self.reserve);
            }
        }
        0
    }
}
