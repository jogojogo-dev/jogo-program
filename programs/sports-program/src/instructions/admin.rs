use anchor_lang::prelude::*;

use crate::state::Admin;

#[derive(Accounts)]
pub struct InitAdmin<'info> {
    #[account(mut)]
    pub owner: Signer<'info>,
    pub fee_receiver: SystemAccount<'info>,
    // program accounts
    #[account(init, payer = owner, space = 8 + Admin::SIZE)]
    pub admin: Account<'info, Admin>,
    // system program
    pub system_program: Program<'info, System>,
}

#[event]
pub struct InitAdminEvent {
    pub owner: Pubkey,
    pub fee_receiver: Pubkey,
    pub admin: Pubkey,
}

pub(crate) fn _init_admin(ctx: Context<InitAdmin>) -> Result<()> {
    ctx.accounts.admin.set_inner(
        Admin::new(
            ctx.accounts.owner.key(),
            ctx.accounts.fee_receiver.key(),
        )
    );

    emit!(InitAdminEvent {
        owner: ctx.accounts.owner.key(),
        fee_receiver: ctx.accounts.fee_receiver.key(),
        admin: ctx.accounts.admin.key(),
    });

    Ok(())
}

#[derive(Accounts)]
pub struct ChangeFeeReceiver<'info> {
    pub owner: Signer<'info>,
    pub fee_receiver: SystemAccount<'info>,
    // program accounts
    #[account(mut, has_one = owner)]
    pub admin: Account<'info, Admin>,
}

#[event]
pub struct ChangeFeeReceiverEvent {
    pub admin: Pubkey,
    pub old_fee_receiver: Pubkey,
    pub new_fee_receiver: Pubkey,
}

pub(crate) fn _change_fee_receiver(ctx: Context<ChangeFeeReceiver>) -> Result<()> {
    let old_fee_receiver = ctx.accounts.admin.fee_receiver;
    ctx.accounts.admin.fee_receiver = ctx.accounts.fee_receiver.key();

    emit!(ChangeFeeReceiverEvent {
        admin: ctx.accounts.admin.key(),
        old_fee_receiver,
        new_fee_receiver: ctx.accounts.fee_receiver.key(),
    });

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

#[event]
pub struct AssignOperatorEvent {
    pub admin: Pubkey,
    pub operator: Pubkey,
}

pub(crate) fn _assign_operator(ctx: Context<AssignOperator>) -> Result<()> {
    ctx.accounts.admin.assign_operator(ctx.accounts.operator.key());

    emit!(AssignOperatorEvent {
        admin: ctx.accounts.admin.key(),
        operator: ctx.accounts.operator.key(),
    });

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

#[event]
pub struct RemoveOperatorEvent {
    pub admin: Pubkey,
    pub operator: Pubkey,
}

pub(crate) fn _remove_operator(ctx: Context<RemoveOperator>) -> Result<()> {
    ctx.accounts.admin.remove_operator(ctx.accounts.operator.key);

    emit!(RemoveOperatorEvent {
        admin: ctx.accounts.admin.key(),
        operator: ctx.accounts.operator.key(),
    });

    Ok(())
}
