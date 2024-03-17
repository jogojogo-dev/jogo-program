use anchor_lang::prelude::*;
use anchor_spl::token_interface::*;

use crate::state::{Admin, Vault};

#[derive(Accounts)]
pub struct InitVault<'info> {
    #[account(mut)]
    pub owner: Signer<'info>,
    // jogo accounts
    #[account(has_one = owner)]
    pub admin: Account<'info, Admin>,
    #[account(seeds = [b"authority", admin.key().as_ref()], bump = admin.auth_bump[0])]
    pub admin_authority: SystemAccount<'info>,
    #[account(init, payer = owner, space = 8 + Vault::SIZE)]
    pub vault: Box<Account<'info, Vault>>,
    // token accounts
    pub chip_mint: InterfaceAccount<'info, Mint>,
    #[account(
        init,
        payer = owner,
        token::mint = chip_mint,
        token::authority = admin_authority,
    )]
    pub vault_chip_account: Box<InterfaceAccount<'info, TokenAccount>>,
    #[account(
        init,
        payer = owner,
        mint::decimals = chip_mint.decimals,
        mint::authority = admin_authority,
    )]
    pub lp_token_mint: Box<InterfaceAccount<'info, Mint>>,
    pub token_program: Program<'info, Token2022>,
    // system program
    pub system_program: Program<'info, System>,
}

pub(crate) fn _init_vault(ctx: Context<InitVault>) -> Result<()> {
    let vault = Vault {
        admin: ctx.accounts.admin.key(),
        lp_token_mint: ctx.accounts.lp_token_mint.key(),
        vault_chip_account: ctx.accounts.vault_chip_account.key(),
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
    pub user: Signer<'info>,
    // jogo accounts
    pub admin: Account<'info, Admin>,
    #[account(seeds = [b"authority", admin.key().as_ref()], bump = admin.auth_bump[0])]
    pub admin_authority: SystemAccount<'info>,
    #[account(mut, has_one = admin, has_one = lp_token_mint, has_one = vault_chip_account)]
    pub vault: Account<'info, Vault>,
    // token accounts
    pub chip_mint: InterfaceAccount<'info, Mint>,
    #[account(mut)]
    pub lp_token_mint: InterfaceAccount<'info, Mint>,
    #[account(mut)]
    pub vault_chip_account: InterfaceAccount<'info, TokenAccount>,
    #[account(mut)]
    pub user_chip_account: InterfaceAccount<'info, TokenAccount>,
    #[account(mut)]
    pub user_lp_token_account: InterfaceAccount<'info, TokenAccount>,
    pub token_program: Program<'info, Token2022>,
}

pub(crate) fn _deposit(ctx: Context<Deposit>, amount: u64) -> Result<()> {
    let cpi_ctx = CpiContext::new(
        ctx.accounts.token_program.to_account_info(),
        TransferChecked {
            from: ctx.accounts.user_chip_account.to_account_info(),
            mint: ctx.accounts.chip_mint.to_account_info(),
            to: ctx.accounts.vault_chip_account.to_account_info(),
            authority: ctx.accounts.user.to_account_info(),
        },
    );
    transfer_checked(cpi_ctx, amount, ctx.accounts.chip_mint.decimals)?;

    let minted_lp = ctx.accounts.vault.deposit(amount)?;
    let signer_seeds = &[
        &[
            b"authority".as_slice(),
            ctx.accounts.vault.admin.as_ref(),
            &ctx.accounts.admin.auth_bump,
        ][..],
    ];
    let cpi_ctx = CpiContext::new_with_signer(
        ctx.accounts.token_program.to_account_info(),
        MintTo {
            mint: ctx.accounts.lp_token_mint.to_account_info(),
            to: ctx.accounts.user_lp_token_account.to_account_info(),
            authority: ctx.accounts.admin_authority.to_account_info(),
        },
        signer_seeds,
    );
    mint_to(cpi_ctx, minted_lp)
}

#[derive(Accounts)]
#[instruction(amount: u64)]
pub struct Withdraw<'info> {
    pub user: Signer<'info>,
    // jogo accounts
    pub admin: Account<'info, Admin>,
    #[account(seeds = [b"authority", admin.key().as_ref()], bump = admin.auth_bump[0])]
    pub admin_authority: SystemAccount<'info>,
    #[account(mut, has_one = admin, has_one = lp_token_mint, has_one = vault_chip_account)]
    pub vault: Account<'info, Vault>,
    // token accounts
    pub chip_mint: InterfaceAccount<'info, Mint>,
    #[account(mut)]
    pub lp_token_mint: InterfaceAccount<'info, Mint>,
    #[account(mut)]
    pub vault_chip_account: InterfaceAccount<'info, TokenAccount>,
    #[account(mut)]
    pub user_chip_account: InterfaceAccount<'info, TokenAccount>,
    #[account(mut)]
    pub user_lp_token_account: InterfaceAccount<'info, TokenAccount>,
    pub token_program: Program<'info, Token2022>,
}

pub(crate) fn _withdraw(ctx: Context<Withdraw>, amount: u64) -> Result<()> {
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
    let signer_seeds = &[
        &[
            b"authority".as_slice(),
            ctx.accounts.vault.admin.as_ref(),
            &ctx.accounts.admin.auth_bump,
        ][..],
    ];
    let cpi_ctx = CpiContext::new_with_signer(
        ctx.accounts.token_program.to_account_info(),
        TransferChecked {
            from: ctx.accounts.vault_chip_account.to_account_info(),
            mint: ctx.accounts.chip_mint.to_account_info(),
            to: ctx.accounts.user_chip_account.to_account_info(),
            authority: ctx.accounts.admin_authority.to_account_info(),
        },
        signer_seeds,
    );
    transfer_checked(cpi_ctx, withdrawal, ctx.accounts.chip_mint.decimals)
}
