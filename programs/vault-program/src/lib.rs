use anchor_lang::{prelude::*, system_program::{Transfer, transfer}};

declare_id!("9Douj7WFcMwJM4di1Vhth2P41Wpnt99Y88MzECtdK5Bw");

#[error_code]
pub enum CustomError {
    #[msg("Overflow")]
    Overflow,
}

#[account]
pub struct Global {
    pub amount: u64,
}

impl Global {
    pub const SIZE: usize = std::mem::size_of::<Self>();
}

#[account]
pub struct Credential {
    pub amount: u64,
}

impl Credential {
    pub const SIZE: usize = std::mem::size_of::<Self>();
}

#[program]
pub mod vault_program {
    use super::*;

    pub fn init_global(ctx: Context<InitGlobal>) -> Result<()> {
        ctx.accounts.global.amount = 0;

        Ok(())
    }

    pub fn deposit_or_withdraw(
        ctx: Context<DepositOrWithdraw>,
        is_deposit: bool,
        amount: u64,
    ) -> Result<()> {
        if is_deposit {
            let cpi_ctx = CpiContext::new(
                ctx.accounts.system_program.to_account_info(),
                Transfer {
                    from: ctx.accounts.user.to_account_info(),
                    to: ctx.accounts.global.to_account_info(),
                },
            );
            transfer(cpi_ctx, amount)?;

            ctx.accounts.global.amount = ctx.accounts.global.amount
                .checked_add(amount)
                .ok_or(CustomError::Overflow)?;
            ctx.accounts.credential.amount = ctx.accounts.credential.amount
                .checked_add(amount)
                .ok_or(CustomError::Overflow)?;
        } else {
            ctx.accounts.credential.amount = ctx.accounts.credential.amount
                .checked_sub(amount)
                .ok_or(CustomError::Overflow)?;
            ctx.accounts.global.amount = ctx.accounts.global.amount
                .checked_sub(amount)
                .ok_or(CustomError::Overflow)?;

            ctx.accounts.global.sub_lamports(amount)?;
            ctx.accounts.user.add_lamports(amount)?;
        }

        Ok(())
    }

    pub fn close_credential(ctx: Context<CloseCredential>) -> Result<()> {
        ctx.accounts.global.amount = ctx.accounts.global.amount
            .checked_sub(ctx.accounts.credential.amount)
            .ok_or(CustomError::Overflow)?;

        ctx.accounts.global.sub_lamports(ctx.accounts.credential.amount)?;
        ctx.accounts.user.add_lamports(ctx.accounts.credential.amount)?;

        Ok(())
    }
}

#[derive(Accounts)]
pub struct InitGlobal<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,
    #[account(
        init,
        payer = payer,
        space = Global::SIZE,
        seeds = [b"global"],
        bump,
    )]
    pub global: Account<'info, Global>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
#[instruction(is_deposit: bool, amount: u64)]
pub struct DepositOrWithdraw<'info> {
    #[account(mut)]
    pub user: Signer<'info>,
    #[account(mut)]
    pub global: Account<'info, Global>,
    #[account(
        init_if_needed,
        payer = user,
        space = Credential::SIZE,
        seeds = [b"credential", user.key().as_ref()],
        bump,
    )]
    pub credential: Account<'info, Credential>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct CloseCredential<'info> {
    #[account(mut)]
    pub user: Signer<'info>,
    #[account(mut)]
    pub global: Account<'info, Global>,
    #[account(
        mut,
        close = user,
        seeds = [b"credential", user.key().as_ref()],
        bump,
    )]
    pub credential: Account<'info, Credential>,
}
