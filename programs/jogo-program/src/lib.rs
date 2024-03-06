mod math;
pub mod error;
pub mod state;
pub mod instructions;

use anchor_lang::prelude::*;

use math::Fraction;
use instructions::*;

declare_id!("CxomvBTcp32f75sgtxCZzzeYpfzGRtubquuQhodh79Nw");

#[program]
pub mod jogo_program {
    use super::*;

    pub fn init_admin(ctx: Context<InitAdmin>) -> Result<()> {
        _init_admin(ctx)
    }
    
    pub fn init_vault(ctx: Context<InitVault>) -> Result<()> {
        _init_vault(ctx)
    }
    
    pub fn deposit(ctx: Context<Deposit>, amount: u64) -> Result<()> {
        _deposit(ctx, amount)
    }
    
    pub fn withdraw(ctx: Context<Withdraw>, amount: u64) -> Result<()> {
        _withdraw(ctx, amount)
    }
    
    pub fn init_crash_game(
        ctx: Context<InitCrashGame>,
        operator: Pubkey,
        win_rate: Fraction,
        max_odd: Fraction,
    ) -> Result<()> {
        _init_crash_game(ctx, operator, win_rate, max_odd)
    }

    pub fn lock_crash(ctx: Context<LockCrash>) -> Result<()> {
        _lock_crash(ctx)
    }
    
    pub fn init_crash_bet(
        ctx: Context<InitCrashBet>,
        stake: u64,
        point: Option<u64>,
    ) -> Result<()> {
        _init_crash_bet(ctx, stake, point)
    }
    
    pub fn settle_crash(ctx: Context<SettleCrash>) -> Result<()> {
        _settle_crash(ctx)
    }
    
    pub fn close_crash_lock(ctx: Context<CloseCrashLock>) -> Result<()> {
        _close_crash_lock(ctx)
    }
}
