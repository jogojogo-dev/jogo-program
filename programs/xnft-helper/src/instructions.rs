use anchor_lang::{prelude::*, system_program::{Transfer as TransferSol, transfer as transfer_sol}};

use crate::xnft::{
    create_app_xnft, create_install, CreateAppXnft, CreateInstall,
    transfer_xnft, TransferXnft, Xnft,
};

#[derive(Accounts)]
pub struct CreateJogoXnft<'info> {
    #[account(mut)]
    pub user: Signer<'info>,
    #[account(mut)]
    pub operator: Signer<'info>,
    #[account(mut)]
    pub target: Signer<'info>,
    // xnft accounts
    /// CHECK: this will be verified in xnft
    #[account(mut)]
    pub master_metadata: UncheckedAccount<'info>,
    /// CHECK: this will be verified in xnft
    pub metadata_program: UncheckedAccount<'info>,
    /// CHECK: this will be verified in xnft
    #[account(mut)]
    pub xnft: UncheckedAccount<'info>,
    pub xnft_program: Program<'info, Xnft>,
    // token accounts
    /// CHECK: this will be verified in xnft
    #[account(mut)]
    pub master_mint: UncheckedAccount<'info>,
    /// CHECK: this will be verified in xnft
    #[account(mut)]
    pub admin_master_token: UncheckedAccount<'info>,
    /// CHECK: this will be verified in xnft
    #[account(mut)]
    pub user_master_token: UncheckedAccount<'info>,
    /// CHECK: this will be verified in xnft
    pub token_program: UncheckedAccount<'info>,
    /// CHECK: this will be verified in xnft
    pub associated_token_program: UncheckedAccount<'info>,
    // system accounts
    /// CHECK: this will be verified in xnft
    pub system_program: UncheckedAccount<'info>,
    /// CHECK: this will be verified in xnft
    pub rent: UncheckedAccount<'info>,
}

pub(crate) fn _create_jogo_xnft(ctx: Context<CreateJogoXnft>) -> Result<()> {
    // step 1: Create the xnft
    let cpi_ctx = CpiContext::new_with_signer(
        ctx.accounts.xnft_program.to_account_info(),
        CreateAppXnft {
            master_mint: ctx.accounts.master_mint.to_account_info(),
            master_token: ctx.accounts.admin_master_token.to_account_info(),
            master_metadata: ctx.accounts.master_metadata.to_account_info(),
            xnft: ctx.accounts.xnft.to_account_info(),
            payer: ctx.accounts.user.to_account_info(),
            publisher: ctx.accounts.operator.to_account_info(),
            system_program: ctx.accounts.system_program.to_account_info(),
            token_program: ctx.accounts.token_program.to_account_info(),
            associated_token_program: ctx.accounts.associated_token_program.to_account_info(),
            metadata_program: ctx.accounts.metadata_program.to_account_info(),
            rent: ctx.accounts.rent.to_account_info(),
        },
        signer_seeds,
    );
    let (name, params) = ctx.accounts.xnft_state.new_create_xnft_params(
        ctx.accounts.admin_authority.key(),
        ctx.accounts.user.key(),
    );
    create_app_xnft(cpi_ctx, name, params)?;

    // step 2: Transfer xnft to user
    let cpi_ctx = CpiContext::new_with_signer(
        ctx.accounts.xnft_program.to_account_info(),
        TransferXnft {
            xnft: ctx.accounts.xnft.to_account_info(),
            source: ctx.accounts.admin_master_token.to_account_info(),
            destination: ctx.accounts.user_master_token.to_account_info(),
            master_mint: ctx.accounts.master_mint.to_account_info(),
            recipient: ctx.accounts.user.to_account_info(),
            authority: ctx.accounts.admin_authority.to_account_info(),
            system_program: ctx.accounts.system_program.to_account_info(),
            token_program: ctx.accounts.token_program.to_account_info(),
            associated_token_program: ctx.accounts.associated_token_program.to_account_info(),
        },
        signer_seeds,
    );
    transfer_xnft(cpi_ctx)?;

    ctx.accounts.xnft_state.mint();

    Ok(())
}

#[derive(Accounts)]
#[instruction(id: u64)]
pub struct CreateJogoXnftInstall<'info> {
    // jogo accounts
    #[account(mut)]
    pub user: Signer<'info>,
    pub operator: Signer<'info>,
    pub admin: Account<'info, Admin>,
    #[account(
    mut,
    seeds = [b"authority", admin.key().as_ref()],
    bump = admin.auth_bump[0],
    )]
    pub admin_authority: SystemAccount<'info>,
    #[account(has_one = admin, has_one = operator, has_one = install_vault)]
    pub xnft_state: Account<'info, XnftState>,
    #[account(
    mut,
    seeds = [b"target", admin.key().as_ref(), id.to_le_bytes().as_ref()],
    bump,
    )]
    pub target: SystemAccount<'info>,
    // xnft accounts
    /// CHECK: this will be verified in xnft
    #[account(mut)]
    pub xnft: UncheckedAccount<'info>,
    /// CHECK: this will be verified in xnft
    #[account(mut)]
    pub install_vault: UncheckedAccount<'info>,
    /// CHECK: this will be verified in xnft
    #[account(mut)]
    pub install: UncheckedAccount<'info>,
    pub xnft_program: Program<'info, Xnft>,
    // system accounts
    /// CHECK: this will be verified in xnft
    pub system_program: UncheckedAccount<'info>,
}

pub(crate) fn _create_jogo_xnft_install(ctx: Context<CreateJogoXnftInstall>, id: u64) -> Result<()> {
    // step 1: Transfer SOL to install vault
    let cpi_ctx = CpiContext::new(
        ctx.accounts.system_program.to_account_info(),
        TransferSol {
            from: ctx.accounts.user.to_account_info(),
            to: ctx.accounts.target.to_account_info(),
        },
    );
    transfer_sol(cpi_ctx, ctx.accounts.user.lamports())?;

    // step 2: Create install
    let target_bump = [ctx.bumps.target];
    let id_bytes = id.to_le_bytes();
    let signer_seeds = &[
        &[
            b"authority".as_slice(),
            ctx.accounts.xnft_state.admin.as_ref(),
            &ctx.accounts.admin.auth_bump,
        ][..],
        &[
            b"target".as_slice(),
            ctx.accounts.xnft_state.admin.as_ref(),
            &id_bytes,
            &target_bump,
        ][..],
    ];
    let cpi_ctx = CpiContext::new_with_signer(
        ctx.accounts.xnft_program.to_account_info(),
        CreateInstall {
            xnft: ctx.accounts.xnft.to_account_info(),
            install_vault: ctx.accounts.install_vault.to_account_info(),
            install: ctx.accounts.install.to_account_info(),
            target: ctx.accounts.target.to_account_info(),
            authority: ctx.accounts.operator.to_account_info(),
            system_program: ctx.accounts.system_program.to_account_info(),
        },
        signer_seeds,
    );
    create_install(cpi_ctx)?;

    Ok(())
}

