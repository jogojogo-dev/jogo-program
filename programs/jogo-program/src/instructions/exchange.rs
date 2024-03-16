use anchor_lang::prelude::*;
use anchor_spl::{token, token_interface, associated_token::AssociatedToken};
use anchor_spl::token::{Burn, burn, mint_to, MintTo, Transfer, transfer};

use crate::state::{Admin, Exchange};

#[derive(Accounts)]
#[instruction(operator: Pubkey)]
pub struct InitExchange<'info> {
    #[account(mut)]
    pub owner: Signer<'info>,
    // jogo accounts
    #[account(has_one = owner)]
    pub admin: Account<'info, Admin>,
    #[account(seeds = [b"authority", admin.key().as_ref()], bump = admin.auth_bump[0])]
    pub admin_authority: SystemAccount<'info>,
    #[account(init, payer = owner, space = 8 + Exchange::SIZE)]
    pub exchange: Box<Account<'info, Exchange>>,
    // token accounts
    pub currency_mint: Box<Account<'info, token::Mint>>,
    #[account(
        init,
        payer = owner,
        token::mint = currency_mint,
        token::authority = admin_authority,
    )]
    pub exchange_currency_account: Box<Account<'info, token::TokenAccount>>,
    #[account(
        init,
        payer = owner,
        mint::decimals = currency_mint.decimals,
        mint::authority = admin_authority,
        mint::token_program = token_2022_program,
    )]
    pub chip_mint: Box<InterfaceAccount<'info, token_interface::Mint>>,
    pub token_program: Program<'info, token::Token>,
    pub token_2022_program: Program<'info, token_interface::Token2022>,
    // system program
    pub system_program: Program<'info, System>,
}

pub(crate) fn _init_exchange(ctx: Context<InitExchange>, operator: Pubkey) -> Result<()> {
    let exchange = Exchange {
        admin: ctx.accounts.admin.key(),
        operator,
        exchange_currency_account: ctx.accounts.exchange_currency_account.key(),
        chip_mint: ctx.accounts.chip_mint.key(),
    };
    ctx.accounts.exchange.set_inner(exchange);

    Ok(())
}

#[derive(Accounts)]
#[instruction(amount: u64)]
pub struct SwapIn<'info> {
    #[account(mut)]
    pub user: Signer<'info>,
    // jogo accounts
    pub admin: Account<'info, Admin>,
    #[account(seeds = [b"authority", admin.key().as_ref()], bump = admin.auth_bump[0])]
    pub admin_authority: SystemAccount<'info>,
    #[account(has_one = admin, has_one = exchange_currency_account, has_one = chip_mint)]
    pub exchange: Account<'info, Exchange>,
    // token accounts
    #[account(mut)]
    pub chip_mint: InterfaceAccount<'info, token_interface::Mint>,
    #[account(mut)]
    pub exchange_currency_account: Account<'info, token::TokenAccount>,
    #[account(mut)]
    pub user_currency_account: Account<'info, token::TokenAccount>,
    #[account(
        init_if_needed,
        payer = user,
        associated_token::mint = chip_mint,
        associated_token::authority = user,
        associated_token::token_program = token_2022_program,
    )]
    pub user_chip_account: InterfaceAccount<'info, token_interface::TokenAccount>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub token_program: Program<'info, token::Token>,
    pub token_2022_program: Program<'info, token_interface::Token2022>,
    // system program
    pub system_program: Program<'info, System>,
}

pub(crate) fn _swap_in(ctx: Context<SwapIn>, amount: u64) -> Result<()> {
    let cpi_ctx = CpiContext::new(
        ctx.accounts.token_program.to_account_info(),
        Transfer {
            from: ctx.accounts.user_currency_account.to_account_info(),
            to: ctx.accounts.exchange_currency_account.to_account_info(),
            authority: ctx.accounts.user.to_account_info(),
        },
    );
    transfer(cpi_ctx, amount)?;

    let signer_seeds = &[
        &[
            b"authority".as_slice(),
            ctx.accounts.exchange.admin.as_ref(),
            &ctx.accounts.admin.auth_bump,
        ][..],
    ];
    let cpi_ctx = CpiContext::new_with_signer(
        ctx.accounts.token_2022_program.to_account_info(),
        MintTo {
            mint: ctx.accounts.chip_mint.to_account_info(),
            to: ctx.accounts.user_chip_account.to_account_info(),
            authority: ctx.accounts.admin_authority.to_account_info(),
        },
        signer_seeds,
    );
    mint_to(cpi_ctx, amount)
}

#[derive(Accounts)]
#[instruction(amount: u64)]
pub struct SwapOut<'info> {
    pub user: Signer<'info>,
    // jogo accounts
    pub admin: Account<'info, Admin>,
    #[account(seeds = [b"authority", admin.key().as_ref()], bump = admin.auth_bump[0])]
    pub admin_authority: SystemAccount<'info>,
    #[account(has_one = admin, has_one = exchange_currency_account, has_one = chip_mint)]
    pub exchange: Account<'info, Exchange>,
    // token accounts
    #[account(mut)]
    pub chip_mint: InterfaceAccount<'info, token_interface::Mint>,
    #[account(mut)]
    pub exchange_currency_account: Account<'info, token::TokenAccount>,
    #[account(mut)]
    pub user_currency_account: Account<'info, token::TokenAccount>,
    #[account(mut)]
    pub user_chip_account: InterfaceAccount<'info, token_interface::TokenAccount>,
    pub token_program: Program<'info, token::Token>,
    pub token_2022_program: Program<'info, token_interface::Token2022>,
    // system program
    pub system_program: Program<'info, System>,
}

pub(crate) fn _swap_out(ctx: Context<SwapOut>, amount: u64) -> Result<()> {
    let cpi_ctx = CpiContext::new(
        ctx.accounts.token_2022_program.to_account_info(),
        Burn {
            mint: ctx.accounts.chip_mint.to_account_info(),
            from: ctx.accounts.user_chip_account.to_account_info(),
            authority: ctx.accounts.user.to_account_info(),
        },
    );
    burn(cpi_ctx, amount)?;

    let signer_seeds = &[
        &[
            b"authority".as_slice(),
            ctx.accounts.exchange.admin.as_ref(),
            &ctx.accounts.admin.auth_bump,
        ][..],
    ];
    let cpi_ctx = CpiContext::new_with_signer(
        ctx.accounts.token_program.to_account_info(),
        Transfer {
            from: ctx.accounts.exchange_currency_account.to_account_info(),
            to: ctx.accounts.user_currency_account.to_account_info(),
            authority: ctx.accounts.admin_authority.to_account_info(),
        },
        signer_seeds,
    );
    transfer(cpi_ctx, amount)
}

#[derive(Accounts)]
#[instruction(amount: u64)]
pub struct MintChip<'info> {
    #[account(mut)]
    pub operator: Signer<'info>,
    pub user: SystemAccount<'info>,
    // jogo accounts
    pub admin: Account<'info, Admin>,
    #[account(seeds = [b"authority", admin.key().as_ref()], bump = admin.auth_bump[0])]
    pub admin_authority: SystemAccount<'info>,
    #[account(has_one = admin, has_one = operator, has_one = chip_mint)]
    pub exchange: Account<'info, Exchange>,
    // token accounts
    #[account(mut)]
    pub chip_mint: InterfaceAccount<'info, token_interface::Mint>,
    #[account(
        init_if_needed,
        payer = operator,
        associated_token::mint = chip_mint,
        associated_token::authority = user,
    )]
    pub user_chip_account: InterfaceAccount<'info, token_interface::TokenAccount>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub token_program: Program<'info, token_interface::Token2022>,
    // system program
    pub system_program: Program<'info, System>,
}

pub(crate) fn _mint_chip(ctx: Context<MintChip>, amount: u64) -> Result<()> {
    let signer_seeds = &[
        &[
            b"authority".as_slice(),
            ctx.accounts.exchange.admin.as_ref(),
            &ctx.accounts.admin.auth_bump,
        ][..],
    ];
    let cpi_ctx = CpiContext::new_with_signer(
        ctx.accounts.token_program.to_account_info(),
        MintTo {
            mint: ctx.accounts.chip_mint.to_account_info(),
            to: ctx.accounts.user_chip_account.to_account_info(),
            authority: ctx.accounts.admin_authority.to_account_info(),
        },
        signer_seeds,
    );
    mint_to(cpi_ctx, amount)
}
