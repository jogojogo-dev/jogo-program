mod state;
mod error;
mod instructions;

use anchor_lang::prelude::*;

use instructions::*;

declare_id!("C7nbA2iATFvC74iN4gVxdWBA2YT3eVg13ps2PBJXMTwh");

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
