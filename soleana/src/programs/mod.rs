use crate::{error::SoleanaResult, types::Pubkey};

pub mod system;

/// A trait for programs that can be parsed by the parser.
pub trait Program {
    fn program_id() -> Pubkey
    where
        Self: Sized;

    type Instructions: ProgramInstructions;

    fn parse_instruction(
        program_id: Pubkey,
        ix_accounts: &Vec<u8>,
        data: &[u8],
        accounts: &[Pubkey],
    ) -> SoleanaResult<Self::Instructions>
    where
        Self: Sized;

    fn match_accounts(ix_accounts: &Vec<u8>, accounts: &[Pubkey]) -> Vec<Pubkey> {
        ix_accounts.iter().map(|&i| accounts[i as usize]).collect()
    }
}

/// A trait for instructions that can be parsed by the parser.
pub trait ProgramInstructions: std::fmt::Debug {}

/// Enum of all the instructions that can be given by the parser.
#[derive(Debug)]
pub enum Instructions {
    /// System instructions.
    System(system::SystemInstructions),
    /// User defined Program instructions.
    Program(Box<dyn ProgramInstructions>),
}
