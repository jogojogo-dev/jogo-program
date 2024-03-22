use anchor_lang::prelude::*;
use solana_program::instruction::Instruction;

#[derive(AnchorDeserialize, AnchorSerialize)]
pub struct CreatorsParam {
    pub address: Pubkey,
    pub share: u8,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug)]
pub enum Tag {
    None,
    Defi,
    Game,
    Nfts,
}

#[derive(AnchorDeserialize, AnchorSerialize)]
pub struct CreateXnftParams {
    pub creators: Vec<CreatorsParam>,
    pub curator: Option<Pubkey>,
    pub install_authority: Option<Pubkey>,
    pub install_price: u64,
    pub install_vault: Pubkey,
    pub seller_fee_basis_points: u16,
    pub supply: Option<u64>,
    pub symbol: String,
    pub tag: Tag,
    pub uri: String,
}

#[derive(Accounts)]
pub struct CreateAppXnft<'info> {
    pub master_mint: AccountInfo<'info>,
    pub master_token: AccountInfo<'info>,
    pub master_metadata: AccountInfo<'info>,
    pub xnft: AccountInfo<'info>,
    pub payer: AccountInfo<'info>,
    pub publisher: AccountInfo<'info>,
    pub system_program: AccountInfo<'info>,
    pub token_program: AccountInfo<'info>,
    pub associated_token_program: AccountInfo<'info>,
    pub metadata_program: AccountInfo<'info>,
    pub rent: AccountInfo<'info>,
}

pub fn create_app_xnft<'info>(
    ctx: CpiContext<'_, '_, '_, 'info, CreateAppXnft<'info>>,
    name: String,
    params: CreateXnftParams,
) -> Result<()> {
    let mut data = vec![163, 181, 42, 46, 219, 211, 7, 133];
    name.serialize(&mut data)?;
    params.serialize(&mut data)?;

    let ix = Instruction {
        program_id: Xnft::id(),
        accounts: ctx.to_account_metas(None),
        data,
    };
    solana_program::program::invoke_signed(
        &ix,
        &ctx.to_account_infos(),
        ctx.signer_seeds,
    ).map_err(Into::into)
}

#[derive(Accounts)]
pub struct CreateInstall<'info> {
    pub xnft: AccountInfo<'info>,
    pub install_vault: AccountInfo<'info>,
    pub install: AccountInfo<'info>,
    pub target: AccountInfo<'info>,
    pub authority: AccountInfo<'info>,
    pub system_program: AccountInfo<'info>,
}

pub fn create_install<'info>(
    ctx: CpiContext<'_, '_, '_, 'info, CreateInstall<'info>>,
) -> Result<()> {
    let ix = Instruction {
        program_id: Xnft::id(),
        accounts: ctx.to_account_metas(None),
        data: vec![41, 58, 153, 9, 228, 59, 218, 207],
    };
    solana_program::program::invoke_signed(
        &ix,
        &ctx.to_account_infos(),
        ctx.signer_seeds,
    ).map_err(Into::into)
}

#[derive(Accounts)]
pub struct TransferXnft<'info> {
    pub xnft: AccountInfo<'info>,
    pub source: AccountInfo<'info>,
    pub destination: AccountInfo<'info>,
    pub master_mint: AccountInfo<'info>,
    pub recipient: AccountInfo<'info>,
    pub authority: AccountInfo<'info>,
    pub system_program: AccountInfo<'info>,
    pub token_program: AccountInfo<'info>,
    pub associated_token_program: AccountInfo<'info>,
}

pub fn transfer_xnft<'info>(
    ctx: CpiContext<'_, '_, '_, 'info, TransferXnft<'info>>,
) -> Result<()> {
    let ix = Instruction {
        program_id: Xnft::id(),
        accounts: ctx.to_account_metas(None),
        data: vec![163, 52, 200, 231, 140, 3, 69, 186],
    };
    solana_program::program::invoke_signed(
        &ix,
        &ctx.to_account_infos(),
        ctx.signer_seeds,
    ).map_err(Into::into)
}

#[derive(Clone)]
pub struct Xnft;

impl Id for Xnft {
    fn id() -> Pubkey {
        solana_program::pubkey!("xnft5aaToUM4UFETUQfj7NUDUBdvYHTVhNFThEYTm55")
    }
}