use anchor_lang::prelude::*;

mod xnft;
mod instructions;

declare_id!("2JmMLmtpKxJSRYmVqbe1zMfhGW5syDksteJtg4Kz9iNp");

#[program]
pub mod xnft_helper {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
