use crate::programs::ProgramInstructions;

/// Equivalent to the `Pubkey` from the solana-pubkey. <https://docs.rs/solana-pubkey/latest/solana_pubkey/struct.Pubkey.html>
pub type Pubkey = [u8; 32];

/// Equivalent to the `Hash` from the solana-hash. <https://docs.rs/solana-hash/latest/solana_hash/struct.Hash.html>
pub type Hash = [u8; 32];

/// Equivalent to the `Signature` from the solana-signature. <https://docs.rs/solana-signature/latest/solana_signature/struct.Signature.htmlhttps://docs.rs/solana-signature/latest/solana_signature/struct.Signature.html>
pub type Signature = [u8; 64];

/// "Equivalent" to the 'Instruction' from solana-instruction. <https://docs.rs/solana-instruction/latest/solana_instruction/struct.Instruction.html>
///
/// Every data fom solana-instruction::Instruction is included here. But not 100% equivalent since it implements more data.
#[derive(Debug)]
pub struct Instruction {
    /// The program ID of the instruction.
    pub program_id: Pubkey,

    pub parsed: Option<Box<dyn ProgramInstructions>>,

    pub raw: Vec<u8>,
}

/// Equivalent to the `MessageHeader` from the solana-message. <https://docs.rs/solana-message/latest/solana_message/struct.MessageHeader.html>
#[derive(Debug)]
pub struct Header {
    pub num_required_signatures: u8,
    pub num_readonly_signed_accounts: u8,
    pub num_readonly_unsigned_accounts: u8,
}

impl Header {
    pub(crate) const BYTE_SIZE: usize = 3;
}

#[derive(Debug)]
pub enum Indicator {
    Legacy,
    V0,
}

#[derive(Debug)]
pub struct LUT {
    pub account_key: Pubkey,
    pub writable_indexes: Vec<u8>,
    pub readonly_indexes: Vec<u8>,
}

#[derive(Debug)]
pub struct CompleteAddressLookupTable {
    pub account_key: Pubkey,
    pub accounts: Vec<Pubkey>,
}

impl From<(Pubkey, Vec<Pubkey>)> for CompleteAddressLookupTable {
    fn from(value: (Pubkey, Vec<Pubkey>)) -> Self {
        Self {
            account_key: value.0,
            accounts: value.1,
        }
    }
}

#[derive(Debug)]
pub struct Transaction {
    pub transaction_type: Indicator,
    pub signatures: Vec<Signature>,
    pub header: Header,
    pub hash: Hash,
    pub instructions: Vec<Instruction>,
    pub luts: Option<Vec<LUT>>,
}
