use anchor_lang::{prelude::*, system_program::{Transfer as Send, transfer as send}};
use anchor_spl::{
    associated_token::AssociatedToken,
    token_interface::{
        Mint, TokenAccount, TokenInterface, TransferChecked, CloseAccount,
        transfer_checked, close_account,
    },
};

use crate::{
    state::{Admin, Club, Credential},
    error::SportsError,
};

const CLUB_CREATION_FEE: u64 = 5_000_000;
const SETTLE_FEE_POINT: u128 = 50;
const BASIS_DIVISOR: u128 = 10000;

#[derive(Accounts)]
#[instruction(identifier: [u8; 32])]
pub struct InitClub<'info> {
    #[account(mut)]
    pub owner: Signer<'info>,
    #[account(mut)]
    pub fee_receiver: SystemAccount<'info>,
    // program accounts
    #[account(has_one = fee_receiver)]
    pub admin: Box<Account<'info, Admin>>,
    #[account(
        init_if_needed,
        payer = owner,
        space = 8 + Club::SIZE,
        seeds = [
            b"club",
            admin.key().as_ref(),
            owner.key().as_ref(),
            token_mint.key().as_ref(),
            &identifier,
        ],
        bump,
    )]
    pub club: Box<Account<'info, Club>>,
    #[account(seeds = [b"authority", club.key().as_ref()], bump)]
    pub club_authority: SystemAccount<'info>,
    // token accounts
    pub token_mint: Box<InterfaceAccount<'info, Mint>>,
    #[account(
        init_if_needed,
        payer = owner,
        associated_token::mint = token_mint,
        associated_token::authority = club_authority,
    )]
    pub supply_token_account: Box<InterfaceAccount<'info, TokenAccount>>,
    pub associated_token_program: Program<'info, AssociatedToken>,
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
    if ctx.accounts.club.initialized {
        return Ok(());
    }

    ctx.accounts.club.set_inner(
        Club::new(
            ctx.accounts.admin.key(),
            ctx.accounts.owner.key(),
            ctx.accounts.token_mint.key(),
            identifier,
        )
    );

    let cpi_ctx = CpiContext::new(
        ctx.accounts.system_program.to_account_info(),
        Send {
            from: ctx.accounts.owner.to_account_info(),
            to: ctx.accounts.fee_receiver.to_account_info(),
        },
    );
    send(cpi_ctx, CLUB_CREATION_FEE)?;

    emit!(InitClubEvent {
        owner: ctx.accounts.owner.key(),
        club: ctx.accounts.club.key(),
        token_mint: ctx.accounts.token_mint.key(),
    });

    Ok(())
}

#[derive(Accounts)]
pub struct CloseClub<'info> {
    #[account(mut)]
    pub owner: Signer<'info>,
    // program accounts
    #[account(mut, close = owner, has_one = token_mint)]
    pub club: Account<'info, Club>,
    #[account(seeds = [b"authority", club.key().as_ref()], bump)]
    pub club_authority: SystemAccount<'info>,
    // token accounts
    pub token_mint: InterfaceAccount<'info, Mint>,
    #[account(mut)]
    pub owner_token_account: InterfaceAccount<'info, TokenAccount>,
    #[account(
        mut,
        associated_token::mint = token_mint,
        associated_token::authority = club_authority,
    )]
    pub supply_token_account: InterfaceAccount<'info, TokenAccount>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub token_program: Interface<'info, TokenInterface>,
}

#[event]
pub struct CloseClubEvent {
    pub owner: Pubkey,
    pub club: Pubkey,
}

pub(crate) fn _close_club(ctx: Context<CloseClub>) -> Result<()> {
    require!(ctx.accounts.club.can_close(), SportsError::CannotCloseClub);

    if ctx.accounts.supply_token_account.amount > 0 {
        let club_key = ctx.accounts.club.key();
        let bumps = [ctx.bumps.club_authority];
        let signer_seeds = &[
            &[
                b"authority".as_slice(),
                club_key.as_ref(),
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
        transfer_checked(
            cpi_ctx,
            ctx.accounts.supply_token_account.amount,
            ctx.accounts.token_mint.decimals,
        )?;

        let cpi_ctx = CpiContext::new_with_signer(
            ctx.accounts.token_program.to_account_info(),
            CloseAccount {
                account: ctx.accounts.supply_token_account.to_account_info(),
                destination: ctx.accounts.owner.to_account_info(),
                authority: ctx.accounts.club_authority.to_account_info(),
            },
            signer_seeds,
        );
        close_account(cpi_ctx)?;
    }

    emit!(CloseClubEvent {
        owner: ctx.accounts.owner.key(),
        club: ctx.accounts.club.key(),
    });

    Ok(())
}

#[derive(Accounts)]
#[instruction(amount: u64)]
pub struct Deposit<'info> {
    pub owner: Signer<'info>,
    // program accounts
    #[account(mut, has_one = owner, has_one = token_mint)]
    pub club: Account<'info, Club>,
    #[account(seeds = [b"authority", club.key().as_ref()], bump)]
    pub club_authority: SystemAccount<'info>,
    // token accounts
    pub token_mint: InterfaceAccount<'info, Mint>,
    #[account(mut)]
    pub owner_token_account: InterfaceAccount<'info, TokenAccount>,
    #[account(
        mut,
        associated_token::mint = token_mint,
        associated_token::authority = club_authority,
    )]
    pub supply_token_account: InterfaceAccount<'info, TokenAccount>,
    pub associated_token_program: Program<'info, AssociatedToken>,
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
    #[account(mut, has_one = owner, has_one = token_mint)]
    pub club: Account<'info, Club>,
    #[account(seeds = [b"authority", club.key().as_ref()], bump)]
    pub club_authority: SystemAccount<'info>,
    // token accounts
    pub token_mint: InterfaceAccount<'info, Mint>,
    #[account(mut)]
    pub owner_token_account: InterfaceAccount<'info, TokenAccount>,
    #[account(
        mut,
        associated_token::mint = token_mint,
        associated_token::authority = club_authority,
    )]
    pub supply_token_account: InterfaceAccount<'info, TokenAccount>,
    pub associated_token_program: Program<'info, AssociatedToken>,
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
    #[account(mut, has_one = admin, has_one = token_mint)]
    pub club: Account<'info, Club>,
    #[account(seeds = [b"authority", club.key().as_ref()], bump)]
    pub club_authority: SystemAccount<'info>,
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
    #[account(
        mut,
        associated_token::mint = token_mint,
        associated_token::authority = club_authority,
    )]
    pub supply_token_account: InterfaceAccount<'info, TokenAccount>,
    pub associated_token_program: Program<'info, AssociatedToken>,
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
        SportsError::InvalidOperator,
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
pub struct CancelBet<'info> {
    pub player: Signer<'info>,
    #[account(mut)]
    pub operator: Signer<'info>,
    // program accounts
    pub admin: Account<'info, Admin>,
    #[account(mut, has_one = admin, has_one = token_mint)]
    pub club: Account<'info, Club>,
    #[account(seeds = [b"authority", club.key().as_ref()], bump)]
    pub club_authority: SystemAccount<'info>,
    #[account(mut, close = operator, has_one = club, has_one = player)]
    pub credential: Account<'info, Credential>,
    // token accounts
    pub token_mint: InterfaceAccount<'info, Mint>,
    #[account(mut)]
    pub player_token_account: InterfaceAccount<'info, TokenAccount>,
    #[account(
        mut,
        associated_token::mint = token_mint,
        associated_token::authority = club_authority,
    )]
    pub supply_token_account: InterfaceAccount<'info, TokenAccount>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub token_program: Interface<'info, TokenInterface>,
}

#[event]
pub struct CancelBetEvent {
    pub club: Pubkey,
    pub credential: Pubkey,
    pub player: Pubkey,
}

pub(crate) fn _cancel_bet(ctx: Context<CancelBet>) -> Result<()> {
    ctx.accounts.club.cancel_bet(&ctx.accounts.credential)?;

    if ctx.accounts.credential.stake > 0 {
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
        transfer_checked(cpi_ctx, ctx.accounts.credential.stake, ctx.accounts.token_mint.decimals)?;
    }

    emit!(CancelBetEvent {
        club: ctx.accounts.club.key(),
        credential: ctx.accounts.credential.key(),
        player: ctx.accounts.player.key(),
    });

    Ok(())
}

#[derive(Accounts)]
#[instruction(direction: u8)]
pub struct Settle<'info> {
    pub player: Signer<'info>,
    #[account(mut)]
    pub operator: Signer<'info>,
    pub fee_receiver: SystemAccount<'info>,
    // program accounts
    #[account(has_one = fee_receiver)]
    pub admin: Account<'info, Admin>,
    #[account(mut, has_one = admin, has_one = token_mint)]
    pub club: Account<'info, Club>,
    #[account(seeds = [b"authority", club.key().as_ref()], bump)]
    pub club_authority: SystemAccount<'info>,
    #[account(mut, close = operator, has_one = club, has_one = player)]
    pub credential: Account<'info, Credential>,
    // token accounts
    pub token_mint: InterfaceAccount<'info, Mint>,
    #[account(mut)]
    pub player_token_account: InterfaceAccount<'info, TokenAccount>,
    #[account(
        mut,
        associated_token::mint = token_mint,
        associated_token::authority = club_authority,
    )]
    pub supply_token_account: InterfaceAccount<'info, TokenAccount>,
    #[account(
        init_if_needed,
        payer = operator,
        associated_token::mint = token_mint,
        associated_token::authority = fee_receiver,
    )]
    pub fee_token_account: InterfaceAccount<'info, TokenAccount>,
    pub associated_token_program: Program<'info, AssociatedToken>,
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
        SportsError::InvalidOperator,
    );

    // update club
    let win = direction == ctx.accounts.credential.direction;
    ctx.accounts.club.settle(win, &ctx.accounts.credential)?;

    if win {
        let prize = ctx.accounts.credential.lock;
        let fee = (prize as u128 * SETTLE_FEE_POINT / BASIS_DIVISOR) as u64;
        let receiving = prize - fee;

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
                to: ctx.accounts.fee_token_account.to_account_info(),
                authority: ctx.accounts.club_authority.to_account_info(),
            },
            signer_seeds,
        );
        transfer_checked(cpi_ctx, fee, ctx.accounts.token_mint.decimals)?;

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
        transfer_checked(cpi_ctx, receiving, ctx.accounts.token_mint.decimals)?;
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