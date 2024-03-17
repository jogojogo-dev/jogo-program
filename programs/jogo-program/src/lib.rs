mod math;
pub mod error;
pub mod state;
pub mod instructions;

use anchor_lang::prelude::*;

use math::Fraction;
use instructions::*;

declare_id!("FN3jzv7cNEyLqtymV9NqiUQi8DqhYc9DshPQhLBqn3uY");

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

    pub fn init_exchange(ctx: Context<InitExchange>, operator: Pubkey) -> Result<()> {
        _init_exchange(ctx, operator)
    }
    
    pub fn swap_in(ctx: Context<SwapIn>, amount: u64) -> Result<()> {
        _swap_in(ctx, amount)
    }
    
    pub fn swap_out(ctx: Context<SwapOut>, amount: u64) -> Result<()> {
        _swap_out(ctx, amount)
    }
    
    pub fn mint_chip(ctx: Context<MintChip>, amount: u64) -> Result<()> {
        _mint_chip(ctx, amount)
    }

    pub fn init_crash_game(
        ctx: Context<InitCrashGame>,
        operator: Pubkey,
        win_rate: Fraction,
        max_odd: Fraction,
    ) -> Result<()> {
        _init_crash_game(ctx, operator, win_rate, max_odd)
    }

    pub fn lock_crash_game(ctx: Context<LockCrashGame>) -> Result<()> {
        _lock_crash_game(ctx)
    }
    
    pub fn create_crash_bet(
        ctx: Context<CreateCrashBet>,
        stake: u64,
        point: Option<u64>,
    ) -> Result<()> {
        _create_crash_bet(ctx, stake, point)
    }
    
    pub fn settle_crash_game(ctx: Context<SettleCrashGame>) -> Result<()> {
        _settle_crash_game(ctx)
    }
    
    pub fn close_crash_lock(ctx: Context<CloseCrashLock>) -> Result<()> {
        _close_crash_lock(ctx)
    }

    pub fn init_third_party_game(ctx: Context<InitThirtyPartyGame>, operator: Pubkey) -> Result<()> {
        _init_third_party_game(ctx, operator)
    }

    pub fn init_third_party_player_state(ctx: Context<InitThirdPartyPlayerState>) -> Result<()> {
        _init_third_party_player_state(ctx)
    }

    pub fn settle_third_party_player_state(ctx: Context<SettleThirdPlayerState>) -> Result<()> {
        _settle_third_party_player_state(ctx)
    }
}
