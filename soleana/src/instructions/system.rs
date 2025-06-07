use solana_sdk::{instruction::CompiledInstruction, pubkey::Pubkey};

use crate::{
    error::{SoleanaError, SoleanaResult},
    instructions::{Program, ProgramInstructions},
};

pub(crate) struct System;

impl Program for System {
    const PROGRAM_ID: Pubkey = Pubkey::new_from_array([0; 32]);

    type Instructions = SystemInstructions;
}

#[derive(Debug, PartialEq, Eq)]
pub enum SystemInstructions {
    Transfer { lamports: u64 },
}

impl ProgramInstructions for SystemInstructions {
    fn parse_instruction(
        instruction: &CompiledInstruction,
        _accounts: &Vec<Pubkey>,
    ) -> SoleanaResult<Self> {
        match instruction.data[0..4].to_vec()[..] {
            [0x02, 0x00, 0x00, 0x00] => {
                let lamports = u64::from_le_bytes(instruction.data[4..12].try_into().unwrap());
                Ok(Self::Transfer { lamports })
            }
            _ => Err(SoleanaError::InvalidInstructionId),
        }
    }
}
