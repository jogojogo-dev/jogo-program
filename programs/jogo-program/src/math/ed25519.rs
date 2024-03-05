use anchor_lang::prelude::*;
use solana_program::{instruction::Instruction, ed25519_program::ID as ED25519_ID};
use crate::error::JogoError;

pub struct ED25519Data<'a> {
    pub signer: Pubkey,
    pub sig: &'a[u8],
    pub msg: &'a[u8],
}

impl<'a> ED25519Data<'a> {
    pub(crate) fn verify_signer(&self, signer: &Pubkey) -> Result<()> {
        if &self.signer == signer {
           Ok(())
        } else {
           Err(JogoError::IncorrectED25519Signer.into())
        }
    }
    
    pub(crate) fn verify_message(&self, msg: &[u8]) -> Result<()> {
        if self.msg == msg {
            Ok(())
        } else {
            Err(JogoError::InvalidED25519Message.into())
        }
    }
}

pub struct ED25519SignatureOffsets {
    signature_offset: u16,             // offset to ed25519 signature of 64 bytes
    signature_instruction_index: u16,  // instruction index to find signature
    public_key_offset: u16,            // offset to public key of 32 bytes
    public_key_instruction_index: u16, // instruction index to find public key
    message_data_offset: u16,          // offset to start of message data
    message_data_size: u16,            // size of message data
    message_instruction_index: u16,    // index of instruction data to get message data
}

impl ED25519SignatureOffsets {
    fn is_valid(&self, msg_len: usize) -> bool {
        let exp_public_key_offset: u16 = 16; // 2*u8 + 7*u16
        let exp_signature_offset: u16 = exp_public_key_offset + 32;
        let exp_message_data_offset: u16 = exp_signature_offset + 64;

        self.signature_offset == exp_signature_offset &&
            self.signature_instruction_index == u16::MAX &&
            self.public_key_offset == exp_public_key_offset &&
            self.public_key_instruction_index == u16::MAX &&
            self.message_data_offset == exp_message_data_offset &&
            self.message_data_size == msg_len as u16 &&
            self.message_instruction_index == u16::MAX
    }
}

/// Load ED25519Program instruction data
pub fn deserialize_ed25519_instruction(instruction: &Instruction) -> Result<ED25519Data> {
    if instruction.program_id != ED25519_ID
        || !instruction.accounts.is_empty()
        || instruction.data.len() <= 112 {
        return Err(JogoError::InvalidED25519Instruction.into());
    }

    // According to this layout used by the Ed25519Program
    // https://github.com/solana-labs/solana/blob/master/sdk/src/ed25519_instruction.rs#L32

    // "Deserializing" byte slices

    let data = &instruction.data;
    let num_signatures = data[0]; // Byte  0
    let padding = data[1]; // Byte  1

    let offsets = ED25519SignatureOffsets {
        signature_offset: u16::from_le_bytes([data[2], data[3]]), // Bytes 2,3
        signature_instruction_index: u16::from_le_bytes([data[4], data[5]]), // Bytes 4,5
        public_key_offset: u16::from_le_bytes([data[6], data[7]]), // Bytes 6,7
        public_key_instruction_index: u16::from_le_bytes([data[8], data[9]]), // Bytes 8,9
        message_data_offset: u16::from_le_bytes([data[10], data[11]]), // Bytes 10,11
        message_data_size: u16::from_le_bytes([data[12], data[13]]), // Bytes 12,13
        message_instruction_index: u16::from_le_bytes([data[14], data[15]]), // Bytes 14,15
    };

    let signer = Pubkey::try_from(&data[16..48]).unwrap();
    let sig = &data[48..112];
    let msg = &data[112..];

    msg!("signature offset {}", offsets.signature_offset);
    msg!("signature instruction index {}", offsets.signature_instruction_index);
    msg!("public key offset {}", offsets.public_key_offset);
    msg!("public key instruction index {}", offsets.public_key_instruction_index);
    msg!("message data offset {}", offsets.message_data_offset);
    msg!("message data size {}", offsets.message_data_size);
    msg!("message instruction index {}", offsets.message_instruction_index);
    
    if num_signatures == 1 && padding == 0 && offsets.is_valid(msg.len()) {
        Ok(ED25519Data { signer, sig, msg })
    } else {
        Err(JogoError::InvalidED25519Instruction.into())
    }
}
