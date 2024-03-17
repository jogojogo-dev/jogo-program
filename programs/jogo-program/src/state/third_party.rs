use anchor_lang::prelude::*;
use solana_program::pubkey::Pubkey;

use crate::error::JogoError;

#[account]
pub struct ThirdPartyGame {
    pub vault: Pubkey,
    pub operator: Pubkey,
}

impl ThirdPartyGame {
    pub const SIZE: usize = std::mem::size_of::<Self>();
}

#[account]
pub struct ThirdPartyPlayerState {
    pub bump: u8,
    pub nonce: u64,
}

impl ThirdPartyPlayerState {
    pub const SIZE: usize = std::mem::size_of::<Self>();

    pub(crate) fn unpack_result(
        &self,
        info: &Pubkey,
        msg: &[u8],
    ) -> Result<(bool, u64)> {
        if msg.len() != 49 {
            return Err(JogoError::InvalidThirdPartyPlayerMessage.into());
        }
        let msg_info = Pubkey::try_from(&msg[..32]).unwrap();
        if &msg_info != info {
            return Err(JogoError::InvalidThirdPartyPlayerMessage.into());
        }
        let nonce = u64::from_le_bytes(msg[32..40].as_ref().try_into().unwrap());
        if nonce != self.nonce {
            return Err(JogoError::InvalidThirdPartyPlayerNonce.into());
        }
        let win = match msg[40] {
            0 => false,
            1 => true,
            _ => return Err(JogoError::InvalidThirdPartyPlayerMessage.into()),
        };
        let amount = u64::from_le_bytes(msg[41..].as_ref().try_into().unwrap());

        Ok((win, amount))
    }

    pub(crate) fn increase(&mut self) {
        self.nonce += 1;
    }
}
