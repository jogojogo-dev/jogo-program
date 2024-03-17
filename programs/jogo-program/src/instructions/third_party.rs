use anchor_lang::prelude::*;
use anchor_spl::token_2022::{transfer_checked, TransferChecked};
use anchor_spl::token_interface::{Mint, TokenAccount, Token2022};
use solana_program::sysvar::{SysvarId, instructions::*};

use crate::{
    math::deserialize_ed25519_instruction,
    state::{Admin, Vault, ThirdPartyGame, ThirdPartyPlayerState},
};

#[derive(Accounts)]
#[instruction(operator: Pubkey)]
pub struct InitThirtyPartyGame<'info> {
    #[account(mut)]
    pub owner: Signer<'info>,
    // jogo accounts
    #[account(has_one = owner)]
    pub admin: Account<'info, Admin>,
    #[account(has_one = admin)]
    pub vault: Account<'info, Vault>,
    #[account(init, payer = owner, space = 8 + ThirdPartyGame::SIZE)]
    pub third_party: Account<'info, ThirdPartyGame>,
    // system accounts
    pub system_program: Program<'info, System>,
}

pub(crate) fn _init_third_party_game(
    ctx: Context<InitThirtyPartyGame>,
    operator: Pubkey,
) -> Result<()> {
    let game = ThirdPartyGame {
        vault: ctx.accounts.vault.key(),
        operator,
    };
    ctx.accounts.third_party.set_inner(game);

    Ok(())
}

#[derive(Accounts)]
pub struct InitThirdPartyPlayerState<'info> {
    #[account(mut)]
    pub player: Signer<'info>,
    // jogo accounts
    pub vault: Account<'info, Vault>,
    #[account(mut, has_one = vault)]
    pub game: Account<'info, ThirdPartyGame>,
    #[account(
        init_if_needed,
        payer = player,
        space = 8 + ThirdPartyPlayerState::SIZE,
        seeds = [game.key().as_ref(), player.key().as_ref()],
        bump,
    )]
    pub player_state: Account<'info, ThirdPartyPlayerState>,
    // system program
    pub system_program: Program<'info, System>,
}

pub(crate) fn _init_third_party_player_state(ctx: Context<InitThirdPartyPlayerState>) -> Result<()> {
    ctx.accounts.player_state.bump = ctx.bumps.player_state;

    Ok(())
}

#[derive(Accounts)]
pub struct SettleThirdPlayerState<'info> {
    pub player: Signer<'info>,
    // jogo accounts
    pub admin: Account<'info, Admin>,
    #[account(seeds = [b"authority", admin.key().as_ref()], bump = admin.auth_bump[0])]
    pub admin_authority: SystemAccount<'info>,
    #[account(mut, has_one = admin, has_one = vault_chip_account)]
    pub vault: Account<'info, Vault>,
    #[account(mut, has_one = vault)]
    pub game: Account<'info, ThirdPartyGame>,
    #[account(
        mut,
        seeds = [game.key().as_ref(), player.key().as_ref()],
        bump = player_state.bump,
    )]
    pub player_state: Account<'info, ThirdPartyPlayerState>,
    // token accounts
    pub chip_mint: InterfaceAccount<'info, Mint>,
    #[account(mut)]
    pub vault_chip_account: InterfaceAccount<'info, TokenAccount>,
    #[account(mut)]
    pub player_chip_account: InterfaceAccount<'info, TokenAccount>,
    pub token_program: Program<'info, Token2022>,
    // system accounts
    /// CHECK: this is an instructions sysvar account
    #[account(address = Instructions::id())]
    pub instructions: UncheckedAccount<'info>,
}

pub(crate) fn _settle_third_party_player_state(ctx: Context<SettleThirdPlayerState>) -> Result<()> {
    let index = load_current_index_checked(&ctx.accounts.instructions)? as usize;
    let instruction = load_instruction_at_checked(index - 1, &ctx.accounts.instructions)?;
    let instruction_data = deserialize_ed25519_instruction(&instruction)?;
    instruction_data.verify_signer(&ctx.accounts.game.operator)?;
    let (win, amount) = ctx.accounts.player_state.unpack_result(
        &ctx.accounts.player_state.key(),
        instruction_data.msg,
    )?;
    msg!("SettleThirdPartyPlayerState: Verified Ed25519 instruction");
    ctx.accounts.player_state.increase();
    ctx.accounts.vault.settle_third_party(win, amount)?;

    if amount == 0 {
        return Ok(());
    }
    if win {
        msg!("SettleThirdPartyPlayerState: Player win: {}", amount);
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
                to: ctx.accounts.player_chip_account.to_account_info(),
                authority: ctx.accounts.admin_authority.to_account_info(),
            },
            signer_seeds,
        );
        transfer_checked(cpi_ctx, amount, ctx.accounts.chip_mint.decimals)
    } else {
        msg!("SettleThirdPartyPlayerState: Player lose: {}", amount);
        let cpi_ctx = CpiContext::new(
            ctx.accounts.token_program.to_account_info(),
            TransferChecked {
                from: ctx.accounts.player_chip_account.to_account_info(),
                mint: ctx.accounts.chip_mint.to_account_info(),
                to: ctx.accounts.vault_chip_account.to_account_info(),
                authority: ctx.accounts.player.to_account_info(),
            },
        );
        transfer_checked(cpi_ctx, amount, ctx.accounts.chip_mint.decimals)
    }
}
