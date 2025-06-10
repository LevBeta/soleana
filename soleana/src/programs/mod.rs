use crate::{error::SoleanaResult, types::Pubkey};

pub mod system;

pub trait Program {
    fn program_id() -> Pubkey
    where
        Self: Sized;

    type Instructions: ProgramInstructions;

    fn parse_instruction(
        program_id: Pubkey,
        accounts: &[Pubkey],
        data: &[u8],
    ) -> SoleanaResult<Self::Instructions>
    where
        Self: Sized;
}

pub trait ProgramInstructions: std::fmt::Debug {}

#[derive(Debug)]
pub enum Instructions {
    System(system::SystemInstructions),
    Program(Box<dyn ProgramInstructions>),
}
