use anchor_lang::prelude::*;
use solana_program::hash::{hash, hashv};
use orao_solana_vrf::state::Randomness;

use crate::{error::JogoError, math::Fraction};

const POINT_PRECISION: u64 = 100;

#[account]
pub struct CrashGame {
    pub vault: Pubkey,
    pub operator: Pubkey,
    pub win_rate: Fraction,
    pub max_odd: Fraction,
    pub next_round: u64,
    pub last_randomness: [u8; 64],
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
            last_randomness: [0u8; 64],
        })
    }

    pub fn set_operator(&mut self, operator: Pubkey) {
        self.operator = operator;
    }

    pub fn set_win_rate(&mut self, win_rate: Fraction) -> Result<()> {
        if win_rate > Fraction::zero() && win_rate < Fraction::one() {
            self.win_rate = win_rate;
            Ok(())
        } else {
            Err(JogoError::InvalidWinningRate.into())
        }
    }

    pub fn set_max_odd(&mut self, max_odd: Fraction) -> Result<()> {
        if max_odd > Fraction::one() {
            self.max_odd = max_odd;
            Ok(())
        } else {
            Err(JogoError::InvalidOdd.into())
        }
    }

    pub(crate) fn lock(
        &mut self,
        bump: u8,
        randomness: &Randomness,
    ) -> Result<CrashLock> {
        if let Some(randomness) = randomness.fulfilled() {
            let lock = CrashLock {
                bump,
                round: self.next_round,
                randomness: *randomness,
            };
            self.next_round += 1;
            self.last_randomness = *randomness;
            Ok(lock)
        } else {
            Err(JogoError::RandomnessNotFulfilled.into())
        }
    }

    pub(crate) fn bet(
        &self,
        bump: u8,
        stake: u64,
        point: Option<u64>,
    ) -> Result<CrashBet> {
        if stake == 0 {
            Err(JogoError::InvalidStakeAmount.into())
        } else {
            Ok(CrashBet {
                bump,
                stake,
                reserve: self.max_odd.mul_u64(stake),
                point,
            })
        }
    }

    pub(crate) fn seed(&self, lock: &Pubkey) -> [u8; 32] {
        hashv(&[lock.as_ref(), &self.last_randomness]).to_bytes()
    }

    pub(crate) fn crash_point(&self, randomness_sig: &[u8]) -> Result<u64> {
        let sig_hash = hash(randomness_sig).to_bytes();
        let final_rand = u32::from_le_bytes(
            <[u8; 4]>::try_from(&sig_hash[..4]).unwrap()
        );
        let point = self.win_rate
            .try_mul(Fraction::new(1u64 << 32, final_rand as u64 + 1)?)?
            .mul_u64(POINT_PRECISION);

        Ok(point)
    }
}

#[account]
pub struct CrashLock {
    pub bump: u8,
    pub round: u64,
    pub randomness: [u8; 64],
}

impl CrashLock {
    pub const SIZE: usize = std::mem::size_of::<Self>();
}

#[account]
pub struct CrashBet {
    pub bump: u8,
    pub stake: u64,
    pub reserve: u64,
    pub point: Option<u64>,
}

impl CrashBet {
    pub const SIZE: usize = 1 + std::mem::size_of::<Self>();

    pub(crate) fn unpack_point(&self, bet: &Pubkey, msg: &[u8]) -> Result<u64> {
        if let Some(point) = self.point {
            Ok(point)
        } else {
            if msg.len() != 40 {
                return Err(JogoError::InvalidBetMessage.into());
            }
            let msg_bet = Pubkey::try_from(&msg[..32]).unwrap();
            if &msg_bet != bet {
                return Err(JogoError::InvalidBetMessage.into());
            }

            Ok(u64::from_le_bytes(msg[32..].as_ref().try_into().unwrap()))
        }
    }

    pub(crate) fn settle(
        &self,
        point: Option<u64>,
        crash_point: u64,
    ) -> u64 {
        if let Some(point) = point.or(self.point) {
            if point <= crash_point {
                return Fraction::new(point, POINT_PRECISION)
                    .unwrap()
                    .mul_u64(self.stake)
                    .min(self.reserve);
            }
        }
        0
    }
}
