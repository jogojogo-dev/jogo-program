mod state;
mod error;
mod instructions;

use anchor_lang::prelude::*;

use instructions::*;

declare_id!("7bAmA2uhe4QDhVGnUzAXwpaDNGX4F4VzMM6Lb3nctYYH");

#[program]
pub mod game_program {
    use super::*;

    pub fn init_admin(ctx: Context<InitAdmin>) -> Result<()> {
        _init_admin(ctx)
    }
    
    pub fn assign_operator(ctx: Context<AssignOperator>) -> Result<()> {
        _assign_operator(ctx)
    }
    
    pub fn remove_operator(ctx: Context<RemoveOperator>) -> Result<()> {
        _remove_operator(ctx)
    }
    
    pub fn init_club(ctx: Context<InitClub>, identifier: [u8; 32]) -> Result<()> {
        _init_club(ctx, identifier)
    }
    
    pub fn deposit(ctx: Context<Deposit>, amount: u64) -> Result<()> {
        _deposit(ctx, amount)
    }
    
    pub fn withdraw(ctx: Context<Withdraw>, amount: u64) -> Result<()> {
        _withdraw(ctx, amount)
    }
    
    pub fn bet(
        ctx: Context<Bet>,
        identifier: [u8; 32],
        direction: u8,
        stake: u64,
        lock: u64,
    ) -> Result<()> {
        _bet(ctx, identifier, direction, stake, lock)
    }
    
    pub fn settle(ctx: Context<Settle>, direction: u8) -> Result<()> {
        _settle(ctx, direction)
    }
}
