mod math;
pub(crate) mod xnft;
pub mod error;
pub mod state;
pub mod instructions;

use anchor_lang::prelude::*;

use math::Fraction;
use instructions::*;

declare_id!("D9jrqbFTaYCP1eiBNr1fy8ALDfkF31gAc2eiif51aUUg");

#[program]
pub mod jogo_program {
    use super::*;

    pub fn init_admin(ctx: Context<InitAdmin>) -> Result<()> {
        _init_admin(ctx)
    }
    
    pub fn init_vault(ctx: Context<InitVault>) -> Result<()> {
        _init_vault(ctx)
    }
    
    pub fn deposit_or_withdraw(
        ctx: Context<DepositOrWithdraw>,
        is_deposit: bool,
        amount: u64,
    ) -> Result<()> {
        _deposit_or_withdraw(ctx, is_deposit, amount)
    }
    
    pub fn init_exchange(ctx: Context<InitExchange>, operator: Pubkey) -> Result<()> {
        _init_exchange(ctx, operator)
    }
    
    pub fn swap(ctx: Context<Swap>, is_in: bool, amount: u64) -> Result<()> {
        _swap(ctx, is_in, amount)
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
    
    pub fn settle_crash_bet(ctx: Context<SettleCrashBet>) -> Result<()> {
        _settle_crash_bet(ctx)
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
