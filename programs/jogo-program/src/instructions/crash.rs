use anchor_lang::prelude::*;
use anchor_spl::token::{Token, TokenAccount, Transfer, transfer};
use orao_solana_vrf::{state::Randomness, RANDOMNESS_ACCOUNT_SEED};
use solana_program::sysvar::{instructions::load_instruction_at_checked, SysvarId};
use solana_program::sysvar::instructions::load_current_index_checked;

use crate::math::{Fraction, deserialize_ed25519_instruction};
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
pub struct LockCrashGame<'info> {
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

pub(crate) fn _lock_crash_game(ctx: Context<LockCrashGame>) -> Result<()> {
    let lock = ctx.accounts.game.lock(ctx.bumps.lock, &ctx.accounts.randomness)?;
    ctx.accounts.lock.set_inner(lock);

    Ok(())
}

#[derive(Accounts)]
#[instruction(stake: u64, point: Option<u64>)]
pub struct CreateCrashBet<'info> {
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

pub(crate) fn _create_crash_bet(
    ctx: Context<CreateCrashBet>,
    stake: u64,
    point: Option<u64>,
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
pub struct SettleCrashGame<'info> {
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
    // TODO: remove system prpgram
    pub system_program: Program<'info, System>,
}

pub(crate) fn _settle_crash_game(ctx: Context<SettleCrashGame>) -> Result<()> {
    let index = load_current_index_checked(&ctx.accounts.instructions)? as usize;
    let instruction = load_instruction_at_checked(index - 2, &ctx.accounts.instructions)?;
    let instruction_data = deserialize_ed25519_instruction(&instruction)?;
    instruction_data.verify_signer(&ctx.accounts.game.operator)?;
    instruction_data.verify_message(&ctx.accounts.lock.randomness)?;
    msg!("SettleCrash: Verified Ed25519 instruction 0");
    let crash_point = ctx.accounts.game.crash_point(instruction_data.sig)?;
    
    let point = if let Ok(instruction) =
        load_instruction_at_checked(index - 1, &ctx.accounts.instructions) {
        let instruction_data = deserialize_ed25519_instruction(&instruction)?;
        instruction_data.verify_signer(&ctx.accounts.game.operator)?;
        let point = ctx.accounts.bet.unpack_point(
            &ctx.accounts.bet.key(),
            instruction_data.msg,
        )?;
        msg!("SettleCrash: Verified Ed25519 instruction 1");
        Some(point)
    } else {
        None
    };
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

#[derive(Accounts)]
pub struct CloseCrashLock<'info> {
    #[account(mut)]
    pub operator: Signer<'info>,
    // jogo accounts
    #[account(mut, has_one = operator)]
    pub game: Account<'info, CrashGame>,
    #[account(
        mut,
        close = operator,
        seeds = [game.key().as_ref(), lock.round.to_le_bytes().as_ref()],
        bump = lock.bump,
        constraint = lock.round + 1 < game.next_round,
    )]
    pub lock: Account<'info, CrashLock>,
}

pub(crate) fn _close_crash_lock(_ctx: Context<CloseCrashLock>) -> Result<()> {
    Ok(())
}
