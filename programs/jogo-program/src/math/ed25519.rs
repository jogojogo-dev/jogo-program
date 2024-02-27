use anchor_lang::prelude::*;
use solana_program::instruction::Instruction;
use bytemuck::{bytes_of, Zeroable, Pod};

const PUBKEY_SERIALIZED_SIZE: usize = 32;
const SIGNATURE_SERIALIZED_SIZE: usize = 64;
const SIGNATURE_OFFSETS_SERIALIZED_SIZE: usize = 14;
// bytemuck requires structures to be aligned
const SIGNATURE_OFFSETS_START: usize = 2;
const DATA_START: usize = SIGNATURE_OFFSETS_SERIALIZED_SIZE + SIGNATURE_OFFSETS_START;

#[derive(Debug, Clone)]
pub struct Ed25519;

impl Id for Ed25519 {
    fn id() -> Pubkey {
        solana_program::ed25519_program::ID
    }
}

#[derive(Default, Debug, Copy, Clone, Zeroable, Pod, Eq, PartialEq)]
#[repr(C)]
pub struct Ed25519SignatureOffsets {
    signature_offset: u16,             // offset to ed25519 signature of 64 bytes
    signature_instruction_index: u16,  // instruction index to find signature
    public_key_offset: u16,            // offset to public key of 32 bytes
    public_key_instruction_index: u16, // instruction index to find public key
    message_data_offset: u16,          // offset to start of message data
    message_data_size: u16,            // size of message data
    message_instruction_index: u16,    // index of instruction data to get message data
}

fn new_ed25519_instruction(message: &[u8], pubkey: &Pubkey, sig: &[u8; 64]) -> Instruction {
    let mut instruction_data = Vec::with_capacity(
        DATA_START + PUBKEY_SERIALIZED_SIZE + SIGNATURE_SERIALIZED_SIZE + message.len()
    );

    let num_signatures: u8 = 1;
    let public_key_offset = DATA_START;
    let signature_offset = public_key_offset + PUBKEY_SERIALIZED_SIZE;
    let message_data_offset = signature_offset + SIGNATURE_SERIALIZED_SIZE;

    // add padding byte so that offset structure is aligned
    instruction_data.extend_from_slice(bytes_of(&[num_signatures, 0]));

    let offsets = Ed25519SignatureOffsets {
        signature_offset: signature_offset as u16,
        signature_instruction_index: u16::MAX,
        public_key_offset: public_key_offset as u16,
        public_key_instruction_index: u16::MAX,
        message_data_offset: message_data_offset as u16,
        message_data_size: message.len() as u16,
        message_instruction_index: u16::MAX,
    };

    instruction_data.extend_from_slice(bytes_of(&offsets));
    instruction_data.extend_from_slice(pubkey.as_ref());
    instruction_data.extend_from_slice(sig);
    instruction_data.extend_from_slice(message);

    Instruction {
        program_id: solana_program::ed25519_program::ID,
        accounts: vec![],
        data: instruction_data,
    }
}

pub(crate) fn ed25519_verify(message: &[u8], pubkey: &Pubkey, sig: &[u8; 64]) -> Result<()> {
    let instruction = new_ed25519_instruction(message, pubkey, sig);
    solana_program::program::invoke(&instruction, &[]).map_err(Into::into)
}
