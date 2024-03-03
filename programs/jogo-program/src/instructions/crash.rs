use anchor_lang::prelude::*;
use anchor_spl::token::{Token, TokenAccount, Transfer, transfer};
use orao_solana_vrf::{state::Randomness, RANDOMNESS_ACCOUNT_SEED};
use solana_program::sysvar::instructions::load_instruction_at_checked;
use solana_program::sysvar::SysvarId;

use crate::error::JogoError;
use crate::math::{Fraction, verify_ed25519_ix};
use crate::state::{Admin, Vault, CrashGame, CrashLock, CrashBet};

#[derive(Accounts)]
#[instruction(operator: Pubkey, win_rate: Fraction, max_odd: Fraction)]
pub struct InitCrashGame<'info> {
    #[account(mut)]
    pub owner: Signer<'info>,
    // jogo accounts
    #[account(has_one = owner)]
    pub admin: Account<'info, Admin>,
    #[account(has_one = admin)]
    pub vault: Account<'info, Vault>,
    #[account(init, payer = owner, space = 8 + CrashGame::SIZE)]
    pub game: Account<'info, CrashGame>,
    // system accounts
    pub system_program: Program<'info, System>,
}

pub(crate) fn _init_crash_game(
    ctx: Context<InitCrashGame>,
    operator: Pubkey,
    win_rate: Fraction,
    max_odd: Fraction,
) -> Result<()> {
    let game = CrashGame::new(
        ctx.accounts.vault.key(),
        operator,
        win_rate,
        max_odd,
    )?;
    ctx.accounts.game.set_inner(game);

    Ok(())
}

#[derive(Accounts)]
#[instruction(stake: u64, point: Option<Fraction>)]
pub struct InitCrashBet<'info> {
    #[account(mut)]
    pub player: Signer<'info>,
    // jogo accounts
    #[account(mut, has_one = supply_token_account)]
    pub vault: Account<'info, Vault>,
    #[account(mut, has_one = vault)]
    pub game: Account<'info, CrashGame>,
    #[account(seeds = [game.key().as_ref(), game.next_round.to_le_bytes().as_ref()], bump)]
    pub lock: SystemAccount<'info>,
    #[account(
        init,
        payer = player,
        space = 8 + CrashBet::SIZE,
        seeds = [lock.key().as_ref(), player.key().as_ref()],
        bump,
    )]
    pub bet: Account<'info, CrashBet>,
    // token accounts
    #[account(mut)]
    pub supply_token_account: Account<'info, TokenAccount>,
    #[account(mut)]
    pub player_token_account: Account<'info, TokenAccount>,
    pub token_program: Program<'info, Token>,
    // system program
    pub system_program: Program<'info, System>,
}

pub(crate) fn _init_crash_bet(
    ctx: Context<InitCrashBet>,
    stake: u64,
    point: Option<Fraction>,
) -> Result<()> {
    let cpi_ctx = CpiContext::new(
        ctx.accounts.token_program.to_account_info(),
        Transfer {
            from: ctx.accounts.player_token_account.to_account_info(),
            to: ctx.accounts.supply_token_account.to_account_info(),
            authority: ctx.accounts.player.to_account_info(),
        },
    );
    transfer(cpi_ctx, stake)?;

    let bet = ctx.accounts.game.bet(ctx.bumps.bet, stake, point)?;
    ctx.accounts.vault.bet(bet.stake, bet.reserve)?;
    ctx.accounts.bet.set_inner(bet);

    Ok(())
}

#[derive(Accounts)]
pub struct LockCrash<'info> {
    #[account(mut)]
    pub operator: Signer<'info>,
    // jogo accounts
    #[account(mut, has_one = operator)]
    pub game: Account<'info, CrashGame>,
    #[account(
        init,
        payer = operator,
        space = 8 + CrashLock::SIZE,
        seeds = [game.key().as_ref(), game.next_round.to_le_bytes().as_ref()],
        bump,
    )]
    pub lock: Account<'info, CrashLock>,
    // vrf randomness
    #[account(
        seeds = [RANDOMNESS_ACCOUNT_SEED, &game.seed(&lock.key())],
        bump,
        seeds::program = orao_solana_vrf::ID,
    )]
    pub randomness: Account<'info, Randomness>,
    // system accounts
    pub system_program: Program<'info, System>,
}

pub(crate) fn _lock_crash(ctx: Context<LockCrash>) -> Result<()> {
    let lock = ctx.accounts.game.lock(ctx.bumps.lock, &ctx.accounts.randomness)?;
    ctx.accounts.lock.set_inner(lock);

    Ok(())
}

#[derive(Accounts)]
#[instruction(
    randomness_sig: [u8; 64],
    bet_sig: Option<[u8; 64]>,
    point: Option<Fraction>,
)]
pub struct SettleCrash<'info> {
    pub player: Signer<'info>,
    // jogo accounts
    pub admin: Account<'info, Admin>,
    #[account(seeds = [b"authority", admin.key().as_ref()], bump = admin.auth_bump[0])]
    pub admin_authority: SystemAccount<'info>,
    #[account(mut, has_one = admin, has_one = supply_token_account)]
    pub vault: Account<'info, Vault>,
    #[account(has_one = vault)]
    pub game: Account<'info, CrashGame>,
    #[account(seeds = [game.key().as_ref(), lock.round.to_le_bytes().as_ref()], bump = lock.bump)]
    pub lock: Account<'info, CrashLock>,
    #[account(
        mut,
        close = player,
        seeds = [lock.key().as_ref(), player.key().as_ref()],
        bump = bet.bump,
    )]
    pub bet: Account<'info, CrashBet>,
    // token accounts
    #[account(mut)]
    pub supply_token_account: Account<'info, TokenAccount>,
    #[account(mut)]
    pub player_token_account: Account<'info, TokenAccount>,
    pub token_program: Program<'info, Token>,
    // system program
    /// CHECK: this is a instructions sysvar account
    #[account(address = Instructions::id())]
    pub instructions: AccountInfo<'info>,
    pub system_program: Program<'info, System>,
}

pub(crate) fn _settle_crash(
    ctx: Context<SettleCrash>,
    randomness_sig: [u8; 64],
    bet_sig: Option<[u8; 64]>,
    point: Option<Fraction>,
) -> Result<()> {
    let instruction = load_instruction_at_checked(0, &ctx.accounts.instructions)?;
    // verify randomness signature
    verify_ed25519_ix(
        &instruction,
        &ctx.accounts.game.operator,
        &ctx.accounts.lock.randomness,
        &randomness_sig,
    )?;
    if let Some(point) = point {
        let instruction = load_instruction_at_checked(1, &ctx.accounts.instructions)?;
        // verify bet signature
        let bet_sig = bet_sig.ok_or::<Error>(JogoError::NoBetSignature.into())?;
        verify_ed25519_ix(
            &instruction,
            &ctx.accounts.game.operator,
            &CrashBet::message(&ctx.accounts.bet.key(), point),
            &bet_sig,
        )?;
    };

    let crash_point = ctx.accounts.game.crash_point(&randomness_sig)?;
    let winning = ctx.accounts.bet.settle(point, crash_point);
    ctx.accounts.vault.settle(ctx.accounts.bet.stake, ctx.accounts.bet.reserve, winning);
    
    if winning > 0 {
        let signer_seeds = &[
            &[
                b"authority".as_slice(),
                ctx.accounts.vault.admin.as_ref(),
                &ctx.accounts.admin.auth_bump,
            ][..],
        ];
        let cpi_ctx = CpiContext::new_with_signer(
            ctx.accounts.token_program.to_account_info(),
            Transfer {
                from: ctx.accounts.supply_token_account.to_account_info(),
                to: ctx.accounts.player_token_account.to_account_info(),
                authority: ctx.accounts.admin_authority.to_account_info(),
            },
            signer_seeds,
        );
        transfer(cpi_ctx, winning)
    } else {
        Ok(())
    }
}
