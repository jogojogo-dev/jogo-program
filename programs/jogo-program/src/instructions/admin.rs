use anchor_lang::prelude::*;

use crate::state::Admin;

#[derive(Accounts)]
pub struct InitAdmin<'info> {
    #[account(mut)]
    pub owner: Signer<'info>,
    // jogo accounts
    #[account(init, payer = owner, space = Admin::SIZE)]
    pub admin: Account<'info, Admin>,
    #[account(seeds = [b"authority", admin.key().as_ref()], bump)]
    pub admin_authority: SystemAccount<'info>,
    // system program
    pub system_program: Program<'info, System>,
}

pub(crate) fn _init_admin(ctx: Context<InitAdmin>) -> Result<()> {
    let admin = Admin {
        owner: ctx.accounts.owner.key(),
        auth_bump: [ctx.bumps.admin_authority],
    };
    ctx.accounts.admin.set_inner(admin);

    Ok(())
}
