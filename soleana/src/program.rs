use crate::{error::SoleanaResult, types::Pubkey};

pub trait Program {
    const PROGRAM_ID: Pubkey;

    type ProgramInstructions;
}

pub trait ProgramInstructions {
    fn parse_instructions() -> SoleanaResult<Self>
    where
        Self: Sized;
}
