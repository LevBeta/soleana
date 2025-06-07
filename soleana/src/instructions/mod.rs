use crate::{instructions::system::System, model::Instruction, SoleanaResult};
use solana_sdk::{instruction::CompiledInstruction, pubkey::Pubkey};

/// System instruction's
pub mod system;

/// System program id
const SYSTEM_PROGRAM_ID: Pubkey = Pubkey::new_from_array([0; 32]);

pub trait Program {
    /// Program id of the account
    const PROGRAM_ID: Pubkey;

    /// Enum for all the instruction that the contract can handle
    type Instructions: ProgramInstructions + std::fmt::Debug + PartialEq + Eq;

    fn parse_instruction(
        &self,
        instruction: &CompiledInstruction,
        accounts: &Vec<Pubkey>,
    ) -> SoleanaResult<Self::Instructions>
    where
        Self::Instructions: Sized,
    {
        Self::Instructions::parse_instruction(instruction, accounts)
    }

    fn program_id(&self) -> Pubkey {
        Self::PROGRAM_ID
    }
}

pub trait ProgramInstructions {
    fn parse_instruction(
        instruction: &CompiledInstruction,
        accounts: &Vec<Pubkey>,
    ) -> SoleanaResult<Self>
    where
        Self: Sized;
}

pub(crate) fn parse_instruction<P: Program>(
    instruction: &CompiledInstruction,
    accounts: &Vec<Pubkey>,
    contracts: &Vec<P>,
) -> SoleanaResult<Instruction<P>> {
    match accounts[instruction.program_id_index as usize] {
        SYSTEM_PROGRAM_ID => Ok(Instruction::System(System::parse_instruction(
            &System,
            instruction,
            accounts,
        )?)),
        _ => {
            match contracts
                .iter()
                .find(|p| p.program_id() == accounts[instruction.program_id_index as usize])
            {
                Some(contract) => Ok(Instruction::Program(
                    contract.parse_instruction(instruction, accounts)?,
                )),
                None => Ok(Instruction::Unknown),
            }
        }
    }
}
