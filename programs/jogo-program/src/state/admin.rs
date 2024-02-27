use anchor_lang::prelude::*;
use solana_program::secp256k1_recover::{secp256k1_recover, Secp256k1Pubkey};

use crate::error::JogoError;

#[account]
pub struct Admin {
    pub owner: Pubkey,
    pub auth_bump: u8,
}

impl Admin {
    pub const SIZE: usize = std::mem::size_of::<Self>();
}
