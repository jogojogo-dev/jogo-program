mod state;
mod error;
mod instructions;

use anchor_lang::prelude::*;

use instructions::*;

declare_id!("9D6mp8PMLU6L1M4r6LcGHeQo67RMov3kT515ieurQqoT");

#[program]
pub mod game_program {
    use super::*;

    pub fn init_admin(ctx: Context<InitAdmin>) -> Result<()> {
        _init_admin(ctx)
    }
    
    pub fn assign_operator(ctx: Context<AssignOperator>, operator: Pubkey) -> Result<()> {
        _assign_operator(ctx, operator)
    }
    
    pub fn remove_operator(ctx: Context<RemoveOperator>, operator: Pubkey) -> Result<()> {
        _remove_operator(ctx, operator)
    }
    
    pub fn init_game(ctx: Context<InitGame>) -> Result<()> {
        _init_game(ctx)
    }
    
    pub fn deposit(ctx: Context<Deposit>, amount: u64) -> Result<()> {
        _deposit(ctx, amount)
    }
    
    pub fn withdraw(ctx: Context<Withdraw>, amount: u64) -> Result<()> {
        _withdraw(ctx, amount)
    }
    
    pub fn bet(ctx: Context<Bet>, round: u64, stake: u64, lock: u64, reward: u64) -> Result<()> {
        _bet(ctx, round, stake, lock, reward)
    }
    
    pub fn settle(ctx: Context<Settle>, round: u64, win: bool) -> Result<()> {
        _settle(ctx, round, win)
    }
}
