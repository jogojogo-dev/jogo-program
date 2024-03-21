use anchor_lang::prelude::*;

use crate::{
    state::{Admin, XnftState},
    xnft::{CreateAppXnft, Xnft, create_app_xnft}
};

// #[derive(Accounts)]
// pub struct InitXnftState<'info> {
//     pub owner: Signer<'info>,
//     pub
// }

#[derive(Accounts)]
pub struct CreateJogoXnft<'info> {
    #[account(mut)]
    pub user: Signer<'info>,
    // jogo accounts
    pub admin: Account<'info, Admin>,
    #[account(seeds = [b"authority", admin.key().as_ref()], bump = admin.auth_bump[0])]
    pub admin_authority: SystemAccount<'info>,
    #[account(mut, has_one = admin)]
    pub xnft_state: Account<'info, XnftState>,
    // xnft accounts
    /// CHECK: this will be verified in xnft
    pub master_mint: UncheckedAccount<'info>,
    /// CHECK: this will be verified in xnft
    pub master_token: UncheckedAccount<'info>,
    /// CHECK: this will be verified in xnft
    pub master_metadata: UncheckedAccount<'info>,
    /// CHECK: this will be verified in xnft
    pub xnft: UncheckedAccount<'info>,
    /// CHECK: this will be verified in xnft
    pub system_program: UncheckedAccount<'info>,
    /// CHECK: this will be verified in xnft
    pub token_program: UncheckedAccount<'info>,
    /// CHECK: this will be verified in xnft
    pub associated_token_program: UncheckedAccount<'info>,
    /// CHECK: this will be verified in xnft
    pub metadata_program: UncheckedAccount<'info>,
    /// CHECK: this will be verified in xnft
    pub rent: UncheckedAccount<'info>,
    // xnft program
    pub xnft_program: Program<'info, Xnft>,
}

pub(crate) fn _create_jogo_xnft(ctx: Context<CreateJogoXnft>) -> Result<()> {
    // Create the XNFT
    let signer_seeds = &[
        &[
            b"authority".as_slice(),
            ctx.accounts.xnft_state.admin.as_ref(),
            &ctx.accounts.admin.auth_bump,
        ][..],
    ];
    let cpi_ctx = CpiContext::new_with_signer(
        ctx.accounts.xnft_program.to_account_info(),
        CreateAppXnft {
            master_mint: ctx.accounts.master_mint.to_account_info(),
            master_token: ctx.accounts.master_token.to_account_info(),
            master_metadata: ctx.accounts.master_metadata.to_account_info(),
            xnft: ctx.accounts.xnft.to_account_info(),
            payer: ctx.accounts.user.to_account_info(),
            publisher: ctx.accounts.admin_authority.to_account_info(),
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

    ctx.accounts.xnft_state.mint();

    Ok(())
}

#[derive(Accounts)]
#[instruction(id: u64)]
pub struct CreateJogoInstall<'info> {
    // jogo accounts
    #[account(mut)]
    pub operator: Signer<'info>,
    pub admin: Account<'info, Admin>,
    #[account(
        mut,
        seeds = [b"authority", admin.key().as_ref()],
        bump = admin.auth_bump[0],
    )]
    pub admin_authority: SystemAccount<'info>,
    #[account(has_one = admin, has_one = operator)]
    pub xnft_state: Account<'info, XnftState>,
    // xnft accounts
    pub xnft: UncheckedAccount<'info>,
    pub install_vault: UncheckedAccount<'info>,
    pub install: AccountInfo<'info>,
    pub target: AccountInfo<'info>,
    pub authority: AccountInfo<'info>,
    pub system_program: AccountInfo<'info>,
}

