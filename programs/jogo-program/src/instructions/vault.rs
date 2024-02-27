use anchor_lang::prelude::*;
use anchor_spl::{token::*, associated_token::AssociatedToken};

use crate::state::{Admin, Vault};

#[derive(Accounts)]
pub struct InitVault<'info> {
    #[account(mut)]
    pub owner: Signer<'info>,
    // jogo accounts
    #[account(has_one = owner)]
    pub admin: Account<'info, Admin>,
    #[account(seeds = [b"authority", admin.key().as_ref()], bump = admin.auth_bump)]
    pub admin_authority: UncheckedAccount<'info>,
    #[account(init, payer = owner, space = Vault::SIZE)]
    pub vault: Account<'info, Vault>,
    // token accounts
    pub supply_token_mint: Account<'info, Mint>,
    #[account(
        init,
        payer = owner,
        token::mint = supply_token_mint,
        token::authority = admin_authority,
    )]
    pub supply_token_account: Account<'info, TokenAccount>,
    #[account(
        init,
        payer = owner,
        mint::decimals = supply_token_mint.decimals,
        mint::authority = admin_authority,
    )]
    pub lp_token_mint: Account<'info, Mint>,
    pub token_program: Program<'info, Token>,
    // system program
    pub system_program: Program<'info, System>,
}

pub(crate) fn init_vault(ctx: Context<InitVault>) -> Result<()> {
    let vault = Vault {
        admin: ctx.accounts.admin.key(),
        lp_token_mint: ctx.accounts.lp_token_mint.key(),
        supply_token_account: ctx.accounts.supply_token_account.key(),
        liquidity: 0,
        stake: 0,
        reserve: 0,
        minted_lp: 0,
    };
    ctx.accounts.vault.set_inner(vault);

    Ok(())
}


#[derive(Accounts)]
#[instruction(amount: u64)]
pub struct Deposit<'info> {
    #[account(mut)]
    pub user: Signer<'info>,
    // jogo accounts
    pub admin: Account<'info, Admin>,
    #[account(seeds = [b"authority", admin.key().as_ref()], bump = admin.auth_bump)]
    pub admin_authority: UncheckedAccount<'info>,
    #[account(mut, has_one = admin, has_one = lp_token_mint, has_one = supply_token_account)]
    pub vault: Account<'info, Vault>,
    // token accounts
    #[account(mut)]
    pub lp_token_mint: Account<'info, Mint>,
    #[account(mut)]
    pub supply_token_account: Account<'info, TokenAccount>,
    #[account(mut)]
    pub user_token_account: Account<'info, TokenAccount>,
    #[account(
    init_if_needed,
    payer = user,
    associated_token::mint = lp_token_mint,
    associated_token::authority = user,
    )]
    pub user_lp_token_account: Account<'info, TokenAccount>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub token_program: Program<'info, Token>,
    // system program
    pub system_program: Program<'info, System>,
}

pub(crate) fn deposit(ctx: Context<Deposit>, amount: u64) -> Result<()> {
    let cpi_ctx = CpiContext::new(
        ctx.accounts.token_program.to_account_info(),
        Transfer {
            from: ctx.accounts.user_token_account.to_account_info(),
            to: ctx.accounts.supply_token_account.to_account_info(),
            authority: ctx.accounts.user.to_account_info(),
        },
    );
    transfer(cpi_ctx, amount)?;

    let minted_lp = ctx.accounts.vault.deposit(amount)?;
    let cpi_ctx = CpiContext::new_with_signer(
        ctx.accounts.token_program.to_account_info(),
        MintTo {
            mint: ctx.accounts.lp_token_mint.to_account_info(),
            to: ctx.accounts.user_lp_token_account.to_account_info(),
            authority: ctx.accounts.admin_authority.to_account_info(),
        },
        &[
            &[
                b"authority",
                ctx.accounts.vault.admin.as_ref(),
                &[ctx.accounts.admin.auth_bump],
            ],
        ],
    );
    mint_to(cpi_ctx, minted_lp)
}


#[derive(Accounts)]
#[instruction(amount: u64)]
pub struct Withdraw<'info> {
    #[account(mut)]
    pub user: Signer<'info>,
    // jogo accounts
    pub admin: Account<'info, Admin>,
    #[account(seeds = [b"authority", vault.admin.as_ref()], bump = admin.auth_bump)]
    pub admin_authority: UncheckedAccount<'info>,
    #[account(mut, has_one = admin, has_one = lp_token_mint, has_one = supply_token_account)]
    pub vault: Account<'info, Vault>,
    // token accounts
    #[account(mut)]
    pub lp_token_mint: Account<'info, Mint>,
    #[account(mut)]
    pub supply_token_account: Account<'info, TokenAccount>,
    #[account(mut)]
    pub user_token_account: Account<'info, TokenAccount>,
    #[account(mut)]
    pub user_lp_token_account: Account<'info, TokenAccount>,
    pub token_program: Program<'info, Token>,
    // system program
    pub system_program: Program<'info, System>,
}

pub(crate) fn withdraw(ctx: Context<Withdraw>, amount: u64) -> Result<()> {
    let cpi_ctx = CpiContext::new(
        ctx.accounts.token_program.to_account_info(),
        Burn {
            mint: ctx.accounts.lp_token_mint.to_account_info(),
            from: ctx.accounts.user_lp_token_account.to_account_info(),
            authority: ctx.accounts.user.to_account_info(),
        },
    );
    burn(cpi_ctx, amount)?;

    let withdrawal = ctx.accounts.vault.withdraw(amount)?;
    let cpi_ctx = CpiContext::new_with_signer(
        ctx.accounts.token_program.to_account_info(),
        Transfer {
            from: ctx.accounts.supply_token_account.to_account_info(),
            to: ctx.accounts.user_token_account.to_account_info(),
            authority: ctx.accounts.admin_authority.to_account_info(),
        },
        &[
            &[
                b"authority",
                ctx.accounts.vault.admin.as_ref(),
                &[ctx.accounts.admin.auth_bump],
            ],
        ],
    );
    transfer(cpi_ctx, withdrawal)
}
