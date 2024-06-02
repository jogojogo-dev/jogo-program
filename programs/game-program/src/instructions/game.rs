use anchor_lang::prelude::*;
use anchor_spl::token_interface::{
    Mint, TokenAccount, TokenInterface,
    transfer_checked, TransferChecked,
};

use crate::state::{Admin, Game, PlayerState};

#[derive(Accounts)]
pub struct InitGame<'info> {
    #[account(mut)]
    pub owner: Signer<'info>,
    // program accounts
    #[account()]
    pub admin: Account<'info, Admin>,
    #[account(
        init,
        payer = owner,
        space = 8 + Game::SIZE,
        seeds = [b"game", admin.key().as_ref(), owner.key().as_ref()],
        bump,
    )]
    pub game: Account<'info, Game>,
    #[account(seeds = [b"authority", game.key().as_ref()], bump)]
    pub authority: SystemAccount<'info>,
    // token accounts
    pub supply_token_mint: InterfaceAccount<'info, Mint>,
    #[account(
        init,
        payer = owner,
        token::mint = supply_token_mint,
        token::authority = authority,
    )]
    pub supply_token_account: InterfaceAccount<'info, TokenAccount>,
    pub token_program: Interface<'info, TokenInterface>,
    // system program
    pub system_program: Program<'info, System>,
}

#[event]
pub struct InitGameEvent {
    pub owner: Pubkey,
    pub game: Pubkey,
    pub token_mint: Pubkey,
}

pub(crate) fn _init_game(ctx: Context<InitGame>) -> Result<()> {
    ctx.accounts.game.bump = [ctx.bumps.game];
    ctx.accounts.game.auth_bump = [ctx.bumps.authority];
    ctx.accounts.game.admin = ctx.accounts.admin.key();
    ctx.accounts.game.owner = ctx.accounts.owner.key();
    ctx.accounts.game.supply_token_account = ctx.accounts.supply_token_account.key();

    emit!(InitGameEvent {
        owner: ctx.accounts.owner.key(),
        game: ctx.accounts.game.key(),
        token_mint: ctx.accounts.supply_token_mint.key(),
    });

    Ok(())
}

#[derive(Accounts)]
#[instruction(amount: u64)]
pub struct Deposit<'info> {
    pub owner: Signer<'info>,
    // program accounts
    #[account(mut, has_one = owner, has_one = supply_token_account)]
    pub game: Account<'info, Game>,
    // token accounts
    pub token_mint: InterfaceAccount<'info, Mint>,
    #[account(mut)]
    pub owner_token_account: InterfaceAccount<'info, TokenAccount>,
    #[account(mut)]
    pub supply_token_account: InterfaceAccount<'info, TokenAccount>,
    pub token_program: Interface<'info, TokenInterface>,
}

#[event]
pub struct DepositEvent {
    pub game: Pubkey,
    pub owner: Pubkey,
    pub token_mint: Pubkey,
    pub amount: u64,
}

pub(crate) fn _deposit(ctx: Context<Deposit>, amount: u64) -> Result<()> {
    let cpi_ctx = CpiContext::new(
        ctx.accounts.token_program.to_account_info(),
        TransferChecked {
            from: ctx.accounts.owner_token_account.to_account_info(),
            mint: ctx.accounts.token_mint.to_account_info(),
            to: ctx.accounts.supply_token_account.to_account_info(),
            authority: ctx.accounts.owner.to_account_info(),
        },
    );
    transfer_checked(cpi_ctx, amount, ctx.accounts.token_mint.decimals)?;

    ctx.accounts.game.deposit(amount)?;

    emit!(DepositEvent {
        game: ctx.accounts.game.key(),
        owner: ctx.accounts.owner.key(),
        token_mint: ctx.accounts.token_mint.key(),
        amount,
    });

    Ok(())
}

#[derive(Accounts)]
#[instruction(amount: u64)]
pub struct Withdraw<'info> {
    pub owner: Signer<'info>,
    // program accounts
    #[account(mut, has_one = owner, has_one = supply_token_account)]
    pub game: Account<'info, Game>,
    #[account(seeds = [b"authority", game.key().as_ref()], bump = game.auth_bump[0])]
    pub authority: SystemAccount<'info>,
    // token accounts
    pub token_mint: InterfaceAccount<'info, Mint>,
    #[account(mut)]
    pub owner_token_account: InterfaceAccount<'info, TokenAccount>,
    #[account(mut)]
    pub supply_token_account: InterfaceAccount<'info, TokenAccount>,
    pub token_program: Interface<'info, TokenInterface>,
}

#[event]
pub struct WithdrawEvent {
    pub game: Pubkey,
    pub owner: Pubkey,
    pub token_mint: Pubkey,
    pub amount: u64,
}

pub(crate) fn _withdraw(ctx: Context<Withdraw>, amount: u64) -> Result<()> {
    ctx.accounts.game.withdraw(amount)?;

    let game = ctx.accounts.game.key();
    let signer_seeds = &[
        &[
            b"authority".as_slice(),
            game.as_ref(),
            &ctx.accounts.game.auth_bump,
        ][..],
    ];
    let cpi_ctx = CpiContext::new_with_signer(
        ctx.accounts.token_program.to_account_info(),
        TransferChecked {
            from: ctx.accounts.supply_token_account.to_account_info(),
            mint: ctx.accounts.token_mint.to_account_info(),
            to: ctx.accounts.owner_token_account.to_account_info(),
            authority: ctx.accounts.authority.to_account_info(),
        },
        signer_seeds,
    );
    transfer_checked(cpi_ctx, amount, ctx.accounts.token_mint.decimals)?;

    emit!(WithdrawEvent {
        game: ctx.accounts.game.key(),
        owner: ctx.accounts.owner.key(),
        token_mint: ctx.accounts.token_mint.key(),
        amount,
    });

    Ok(())
}

#[derive(Accounts)]
#[instruction(round: u64, stake: u64, lock: u64, reward: u64)]
pub struct Bet<'info> {
    pub player: Signer<'info>,
    #[account(mut)]
    pub operator: Signer<'info>,
    // program accounts
    #[account(mut, has_one = supply_token_account)]
    pub game: Account<'info, Game>,
    #[account(
        init_if_needed,
        payer = operator,
        space = 8 + PlayerState::SIZE,
        seeds = [b"player", game.key().as_ref(), player.key().as_ref()],
        bump,
    )]
    pub player_state: Account<'info, PlayerState>,
    // token accounts
    pub token_mint: InterfaceAccount<'info, Mint>,
    #[account(mut)]
    pub player_token_account: InterfaceAccount<'info, TokenAccount>,
    #[account(mut)]
    pub supply_token_account: InterfaceAccount<'info, TokenAccount>,
    pub token_program: Interface<'info, TokenInterface>,
    // system program
    pub system_program: Program<'info, System>,
}

#[event]
pub struct BetEvent {
    pub game: Pubkey,
    pub player: Pubkey,
    pub round: u64,
    pub stake: u64,
    pub lock: u64,
    pub reward: u64,
}

pub(crate) fn _bet(
    ctx: Context<Bet>,
    round: u64,
    stake: u64,
    lock: u64,
    reward: u64,
) -> Result<()> {
    let cpi_ctx = CpiContext::new(
        ctx.accounts.token_program.to_account_info(),
        TransferChecked {
            from: ctx.accounts.player_token_account.to_account_info(),
            mint: ctx.accounts.token_mint.to_account_info(),
            to: ctx.accounts.supply_token_account.to_account_info(),
            authority: ctx.accounts.player.to_account_info(),
        },
    );
    transfer_checked(cpi_ctx, stake, ctx.accounts.token_mint.decimals)?;

    ctx.accounts.player_state.game = ctx.accounts.game.key();
    ctx.accounts.player_state.owner = ctx.accounts.player.key();
    ctx.accounts.player_state.bet(round, stake, lock, reward)?;
    ctx.accounts.game.bet(stake, lock)?;

    emit!(BetEvent {
        game: ctx.accounts.game.key(),
        player: ctx.accounts.player.key(),
        round,
        stake,
        lock,
        reward,
    });

    Ok(())
}

#[derive(Accounts)]
#[instruction(round: u64, win: bool)]
pub struct Settle<'info> {
    pub player: Signer<'info>,
    #[account(mut)]
    pub operator: Signer<'info>,
    // program accounts
    #[account(mut, has_one = supply_token_account)]
    pub game: Account<'info, Game>,
    #[account(seeds = [b"authority", game.key().as_ref()], bump = game.auth_bump[0])]
    pub authority: SystemAccount<'info>,
    #[account(mut)]
    pub player_state: Account<'info, PlayerState>,
    // token accounts
    pub token_mint: InterfaceAccount<'info, Mint>,
    #[account(mut)]
    pub player_token_account: InterfaceAccount<'info, TokenAccount>,
    #[account(mut)]
    pub supply_token_account: InterfaceAccount<'info, TokenAccount>,
    pub token_program: Interface<'info, TokenInterface>,
    // system program
    pub system_program: Program<'info, System>,
}

#[event]
pub struct SettleEvent {
    pub game: Pubkey,
    pub player: Pubkey,
    pub round: u64,
    pub win: bool,
    pub stake: u64,
    pub lock: u64,
    pub reward: u64,
}

pub(crate) fn _settle(ctx: Context<Settle>, round: u64, win: bool) -> Result<()> {
    let stake = ctx.accounts.player_state.stake;
    let lock = ctx.accounts.player_state.lock;
    let reward = ctx.accounts.player_state.reward;
    ctx.accounts.game.settle(win, stake, lock)?;
    ctx.accounts.player_state.settle(round)?;

    let signer_seeds = &[
        &[
            b"authority".as_slice(),
            ctx.accounts.player_state.game.as_ref(),
            &ctx.accounts.game.auth_bump,
        ][..],
    ];
    let cpi_ctx = CpiContext::new_with_signer(
        ctx.accounts.token_program.to_account_info(),
        TransferChecked {
            from: ctx.accounts.supply_token_account.to_account_info(),
            mint: ctx.accounts.token_mint.to_account_info(),
            to: ctx.accounts.player_token_account.to_account_info(),
            authority: ctx.accounts.authority.to_account_info(),
        },
        signer_seeds,
    );
    transfer_checked(cpi_ctx, reward, ctx.accounts.token_mint.decimals)?;

    emit!(SettleEvent {
        game: ctx.accounts.game.key(),
        player: ctx.accounts.player.key(),
        win,
        round,
        stake,
        lock,
        reward,
    });

    Ok(())
}