use crate::{
    error::SoleanaError, instructions::system::SystemInstruction, model::Instruction, SoleanaResult,
};
use solana_sdk::{instruction::CompiledInstruction, pubkey::Pubkey};

use std::str::FromStr;

/// System instruction's
pub mod system;

/// System program id
const SYSTEM_PROGRAM_ID: Pubkey = Pubkey::new_from_array([0; 32]);

pub(crate) fn parse_instruction(
    instruction: &CompiledInstruction,
    accounts: &Vec<Pubkey>,
) -> SoleanaResult<Instruction> {
    match accounts[instruction.program_id_index as usize] {
        SYSTEM_PROGRAM_ID => Ok(Instruction::System(SystemInstruction::parse_instruction(
            instruction,
            accounts,
        )?)),
        _ => Ok(Instruction::Unknown),
    }
}
