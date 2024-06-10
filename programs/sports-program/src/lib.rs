mod state;
mod error;
mod instructions;

use anchor_lang::prelude::*;

use instructions::*;

declare_id!("4BGS57PnHpNr3Sm9yAfVyKrSPweTC8TScKd8QzoLg6qa");

#[program]
pub mod sports_program {
    use super::*;

    pub fn init_admin(ctx: Context<InitAdmin>) -> Result<()> {
        _init_admin(ctx)
    }
    
    pub fn change_fee_receiver(ctx: Context<ChangeFeeReceiver>) -> Result<()> {
        _change_fee_receiver(ctx)
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
    
    pub fn close_club(ctx: Context<CloseClub>) -> Result<()> {
        _close_club(ctx)
    }
    
    pub fn deposit(ctx: Context<Deposit>, amount: u64) -> Result<()> {
        _deposit(ctx, amount)
    }
    
    pub fn withdraw(ctx: Context<Withdraw>, amount: u64) -> Result<()> {
        _withdraw(ctx, amount)
    }

    pub fn start_game(ctx: Context<StartGame>, identifier: [u8; 32]) -> Result<()> {
        _start_game(ctx, identifier)
    }
    
    pub fn close_game(ctx: Context<CloseGame>, cancel: bool) -> Result<()> {
        _close_game(ctx, cancel)
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
    
    pub fn close_bet(ctx: Context<CloseBet>) -> Result<()> {
        _close_bet(ctx)
    }
    
    pub fn settle(ctx: Context<Settle>, direction: u8) -> Result<()> {
        _settle(ctx, direction)
    }
}
