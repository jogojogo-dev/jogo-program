mod math;
pub mod error;
pub mod state;
pub mod instructions;

use anchor_lang::prelude::*;

use math::Fraction;

declare_id!("G4pgWxx2YTE2rGezW3cULwbAruCjh1TNYTu2Df6HCVP2");

#[program]
pub mod jogo_program {
    use super::*;

    pub fn init_admin(ctx: Context<instructions::InitAdmin>) -> Result<()> {
        instructions::init_admin(ctx)
    }

    pub fn init_vault(ctx: Context<instructions::InitVault>) -> Result<()> {
        instructions::init_vault(ctx)
    }

    pub fn deposit(ctx: Context<instructions::Deposit>, amount: u64) -> Result<()> {
        instructions::deposit(ctx, amount)
    }

    pub fn withdraw(ctx: Context<instructions::Withdraw>, amount: u64) -> Result<()> {
        instructions::withdraw(ctx, amount)
    }

    pub fn init_crash_game(
        ctx: Context<instructions::InitCrashGame>,
        operator: Pubkey,
        win_rate: Fraction,
        max_odd: Fraction,
    ) -> Result<()> {
        instructions::init_crash_game(ctx, operator, win_rate, max_odd)
    }

    pub fn init_crash_bet(
        ctx: Context<instructions::InitCrashBet>,
        stake: u64,
        point: Option<Fraction>,
    ) -> Result<()> {
        instructions::init_crash_bet(ctx, stake, point)
    }

    pub fn lock_crash_bet(ctx: Context<instructions::LockCrash>) -> Result<()> {
        instructions::lock_crash(ctx)
    }

    pub fn settle_crash(
        ctx: Context<instructions::SettleCrash>,
        randomness_sig: [u8; 64],
        bet_sig: Option<[u8; 64]>,
        point: Option<Fraction>,
    ) -> Result<()> {
        instructions::settle_crash(ctx, randomness_sig, bet_sig, point)
    }
}
