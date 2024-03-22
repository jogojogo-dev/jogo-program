use anchor_lang::prelude::*;

use xnft_helper::xnft::{CreatorsParam, Tag, CreateXnftParams};

#[account]
pub struct XnftState {
    pub operator: Pubkey,
    pub install_vault: Pubkey,
    pub install_price: u64,
    pub seller_fee_basis_points: u16,
    pub next_index: u64,
}

impl XnftState {
    pub const SIZE: usize = std::mem::size_of::<Self>();

    pub(crate) fn name(&self) -> String {
        format!("JogoJogo XNFT #{}", self.next_index)
    }

    pub(crate) fn symbol(&self) -> String {
        "XJOGO".to_string()
    }

    pub(crate) fn uri(&self) -> String {
        format!("https://jogojogo.io/xnft/{}", self.next_index)
    }

    pub(crate) fn mint(&mut self) {
        self.next_index += 1;
    }

    pub(crate) fn new_create_xnft_params(
        &self,
        admin_authority: Pubkey,
        user: Pubkey,
    ) -> (String, CreateXnftParams) {
        (
            self.name(),
            CreateXnftParams {
                creators: vec![CreatorsParam { address: user, share: 100 }],
                curator: None,
                install_authority: Some(admin_authority),
                install_price: self.install_price,
                install_vault: self.install_vault,
                seller_fee_basis_points: self.seller_fee_basis_points,
                supply: None,
                symbol: self.symbol(),
                tag: Tag::Game,
                uri: self.uri(),
            },
        )
    }
}
