use anchor_lang::prelude::*;

#[account]
pub struct Admin {
    pub owner: Pubkey,
    pub auth_bump: [u8; 1],
}

impl Admin {
    pub const SIZE: usize = std::mem::size_of::<Self>();
}
