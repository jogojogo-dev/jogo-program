use anchor_lang::prelude::*;
use anchor_spl::token_interface::{
    Mint, TokenAccount, TokenInterface,
    transfer_checked, TransferChecked,
};

use crate::{
    state::{Admin, Club, Credential},
    error::GameError,
};

#[derive(Accounts)]
#[instruction(identifier: [u8; 32])]
pub struct InitClub<'info> {
    #[account(mut)]
    pub owner: Signer<'info>,
    // program accounts
    pub admin: Account<'info, Admin>,
    #[account(
        init,
        payer = owner,
        space = 8 + Club::SIZE,
        seeds = [
            b"club",
            admin.key().as_ref(),
            owner.key().as_ref(),
            supply_token_mint.key().as_ref(),
            &identifier,
        ],
        bump,
    )]
    pub club: Account<'info, Club>,
    #[account(seeds = [b"authority", club.key().as_ref()], bump)]
    pub club_authority: SystemAccount<'info>,
    // token accounts
    pub supply_token_mint: InterfaceAccount<'info, Mint>,
    #[account(
        init,
        payer = owner,
        token::mint = supply_token_mint,
        token::authority = club_authority,
    )]
    pub supply_token_account: InterfaceAccount<'info, TokenAccount>,
    pub token_program: Interface<'info, TokenInterface>,
    // system program
    pub system_program: Program<'info, System>,
}

#[event]
pub struct InitClubEvent {
    pub owner: Pubkey,
    pub club: Pubkey,
    pub token_mint: Pubkey,
}

pub(crate) fn _init_club(ctx: Context<InitClub>, identifier: [u8; 32]) -> Result<()> {
    ctx.accounts.club.admin = ctx.accounts.admin.key();
    ctx.accounts.club.owner = ctx.accounts.owner.key();
    ctx.accounts.club.supply_token_account = ctx.accounts.supply_token_account.key();
    ctx.accounts.club.identifier = identifier;

    emit!(InitClubEvent {
        owner: ctx.accounts.owner.key(),
        club: ctx.accounts.club.key(),
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
    pub club: Account<'info, Club>,
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
    pub club: Pubkey,
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

    ctx.accounts.club.deposit(amount)?;

    emit!(DepositEvent {
        club: ctx.accounts.club.key(),
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
    pub club: Account<'info, Club>,
    #[account(seeds = [b"authority", club.key().as_ref()], bump)]
    pub club_authority: SystemAccount<'info>,
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
    pub club: Pubkey,
    pub owner: Pubkey,
    pub token_mint: Pubkey,
    pub amount: u64,
}

pub(crate) fn _withdraw(ctx: Context<Withdraw>, amount: u64) -> Result<()> {
    ctx.accounts.club.withdraw(amount)?;

    let club = ctx.accounts.club.key();
    let bumps = [ctx.bumps.club_authority];
    let signer_seeds = &[
        &[
            b"authority".as_slice(),
            club.as_ref(),
            &bumps,
        ][..],
    ];
    let cpi_ctx = CpiContext::new_with_signer(
        ctx.accounts.token_program.to_account_info(),
        TransferChecked {
            from: ctx.accounts.supply_token_account.to_account_info(),
            mint: ctx.accounts.token_mint.to_account_info(),
            to: ctx.accounts.owner_token_account.to_account_info(),
            authority: ctx.accounts.club_authority.to_account_info(),
        },
        signer_seeds,
    );
    transfer_checked(cpi_ctx, amount, ctx.accounts.token_mint.decimals)?;

    emit!(WithdrawEvent {
        club: ctx.accounts.club.key(),
        owner: ctx.accounts.owner.key(),
        token_mint: ctx.accounts.token_mint.key(),
        amount,
    });

    Ok(())
}

#[derive(Accounts)]
#[instruction(identifier: [u8; 32], direction: u8, stake: u64, lock: u64)]
pub struct Bet<'info> {
    pub player: Signer<'info>,
    #[account(mut)]
    pub operator: Signer<'info>,
    // program accounts
    pub admin: Account<'info, Admin>,
    #[account(mut, has_one = admin, has_one = supply_token_account)]
    pub club: Account<'info, Club>,
    #[account(
        init,
        payer = operator,
        space = 8 + Credential::SIZE,
        seeds = [
            b"credential",
            club.key().as_ref(),
            player.key().as_ref(),
            &identifier,
        ],
        bump,
    )]
    pub credential: Account<'info, Credential>,
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
    pub club: Pubkey,
    pub credential: Pubkey,
    pub player: Pubkey,
    pub direction: u8,
    pub stake: u64,
    pub lock: u64,
}

pub(crate) fn _bet(
    ctx: Context<Bet>,
    identifier: [u8; 32],
    direction: u8,
    stake: u64,
    lock: u64,
) -> Result<()> {
    require!(
        ctx.accounts.admin.is_operator(ctx.accounts.operator.key),
        GameError::InvalidOperator,
    );

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

    // update club
    ctx.accounts.club.bet(direction, stake, lock)?;

    // initialize credential
    ctx.accounts.credential.club = ctx.accounts.club.key();
    ctx.accounts.credential.player = ctx.accounts.player.key();
    ctx.accounts.credential.identifier = identifier;
    ctx.accounts.credential.direction = direction;
    ctx.accounts.credential.stake = stake;
    ctx.accounts.credential.lock = lock;

    emit!(BetEvent {
        club: ctx.accounts.club.key(),
        credential: ctx.accounts.credential.key(),
        player: ctx.accounts.player.key(),
        direction,
        stake,
        lock,
    });

    Ok(())
}

#[derive(Accounts)]
#[instruction(direction: u8)]
pub struct Settle<'info> {
    pub player: Signer<'info>,
    pub operator: Signer<'info>,
    // program accounts
    pub admin: Account<'info, Admin>,
    #[account(mut, has_one = admin, has_one = supply_token_account)]
    pub club: Account<'info, Club>,
    #[account(seeds = [b"authority", club.key().as_ref()], bump)]
    pub club_authority: SystemAccount<'info>,
    #[account(mut, close = operator, has_one = club, has_one = player)]
    pub credential: Account<'info, Credential>,
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
    pub club: Pubkey,
    pub credential: Pubkey,
    pub player: Pubkey,
    pub bet_direction: u8,
    pub final_direction: u8,
}

pub(crate) fn _settle(ctx: Context<Settle>, direction: u8) -> Result<()> {
    require!(
        ctx.accounts.admin.is_operator(ctx.accounts.operator.key),
        GameError::InvalidOperator,
    );

    // update club
    let win = direction == ctx.accounts.credential.direction;
    let locking = ctx.accounts.credential.lock;
    ctx.accounts.club.settle(win, ctx.accounts.credential.direction, locking)?;

    if win {
        let club = ctx.accounts.club.key();
        let bumps = [ctx.bumps.club_authority];
        let signer_seeds = &[
            &[
                b"authority".as_slice(),
                club.as_ref(),
                &bumps,
            ][..],
        ];
        let cpi_ctx = CpiContext::new_with_signer(
            ctx.accounts.token_program.to_account_info(),
            TransferChecked {
                from: ctx.accounts.supply_token_account.to_account_info(),
                mint: ctx.accounts.token_mint.to_account_info(),
                to: ctx.accounts.player_token_account.to_account_info(),
                authority: ctx.accounts.club_authority.to_account_info(),
            },
            signer_seeds,
        );
        transfer_checked(cpi_ctx, locking, ctx.accounts.token_mint.decimals)?;
    }

    emit!(SettleEvent {
        club: ctx.accounts.club.key(),
        credential: ctx.accounts.credential.key(),
        player: ctx.accounts.player.key(),
        bet_direction: ctx.accounts.credential.direction,
        final_direction: direction,
    });

    Ok(())
}