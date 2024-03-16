use anchor_lang::prelude::*;

#[account]
pub struct Exchange {
    pub admin: Pubkey,
    pub operator: Pubkey,
    pub exchange_currency_account: Pubkey,
    pub chip_mint: Pubkey,
}

impl Exchange {
    pub const SIZE: usize = std::mem::size_of::<Self>();
}