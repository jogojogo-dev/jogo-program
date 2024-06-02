use anchor_lang::prelude::*;

use crate::state::Admin;

#[derive(Accounts)]
pub struct InitAdmin<'info> {
    #[account(mut)]
    pub owner: Signer<'info>,
    // program accounts
    #[account(init, payer = owner, space = 8 + Admin::SIZE)]
    pub admin: Account<'info, Admin>,
    // system program
    pub system_program: Program<'info, System>,
}

pub(crate) fn _init_admin(ctx: Context<InitAdmin>) -> Result<()> {
    let admin = Admin {
        owner: ctx.accounts.owner.key(),
        operators: vec![],
    };
    ctx.accounts.admin.set_inner(admin);

    Ok(())
}

#[derive(Accounts)]
pub struct AssignOperator<'info> {
    pub owner: Signer<'info>,
    pub operator: SystemAccount<'info>,
    // program accounts
    #[account(mut, has_one = owner)]
    pub admin: Account<'info, Admin>,
}

pub(crate) fn _assign_operator(ctx: Context<AssignOperator>) -> Result<()> {
    ctx.accounts.admin.assign_operator(ctx.accounts.operator.key());

    Ok(())
}

#[derive(Accounts)]
pub struct RemoveOperator<'info> {
    pub owner: Signer<'info>,
    pub operator: SystemAccount<'info>,
    // program accounts
    #[account(mut, has_one = owner)]
    pub admin: Account<'info, Admin>,
}

pub(crate) fn _remove_operator(ctx: Context<RemoveOperator>) -> Result<()> {
    ctx.accounts.admin.remove_operator(ctx.accounts.operator.key);

    Ok(())
}
