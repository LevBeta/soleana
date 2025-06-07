use crate::{
    error::SoleanaResult,
    instructions::system::SystemInstructions,
    instructions::{parse_instruction, Program},
};
use solana_message::MessageHeader;
use solana_sdk::{instruction::CompiledInstruction, pubkey::Pubkey, signature::Signature};

#[derive(Debug, PartialEq, Eq)]
pub struct ExtendedTransaction<P: Program> {
    pub signatures: Vec<Signature>,
    pub header: MessageHeader,
    pub accounts: Vec<Pubkey>,
    pub instructions: Vec<ExtendedInstruction<P>>,
}

impl<P: Program> ExtendedTransaction<P> {
    pub fn instructions(&self) -> &Vec<ExtendedInstruction<P>> {
        &self.instructions
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct ExtendedInstruction<P: Program> {
    pub raw: CompiledInstruction,
    pub inner: Option<ExtendedInstructionInner<P>>,
}

impl<P: Program> ExtendedInstruction<P> {
    pub fn new(
        instruction: &CompiledInstruction,
        accounts: &Vec<Pubkey>,
        contracts: &Vec<P>,
    ) -> SoleanaResult<Self> {
        let parsed_instruction = parse_instruction(instruction, accounts, contracts)?;

        let inner = match parsed_instruction {
            Instruction::System(_) => Some(ExtendedInstructionInner {
                program_pubkey: accounts[instruction.program_id_index as usize],
                accounts: accounts.clone(),
                instruction: parsed_instruction,
            }),
            Instruction::Program(_) => Some(ExtendedInstructionInner {
                program_pubkey: accounts[instruction.program_id_index as usize],
                accounts: accounts.clone(),
                instruction: parsed_instruction,
            }),
            Instruction::Unknown => None,
        };

        Ok(ExtendedInstruction {
            raw: instruction.clone(),
            inner,
        })
    }

    pub fn is_parsed(&self) -> bool {
        self.inner.is_some()
    }

    pub fn parsed_instruction(&self) -> Option<&ExtendedInstructionInner<P>> {
        self.inner.as_ref()
    }

    pub fn raw_instruction(&self) -> &CompiledInstruction {
        &self.raw
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum Instruction<P: Program> {
    System(SystemInstructions),
    Program(P::Instructions),
    Unknown,
}

#[derive(Debug, PartialEq, Eq)]
pub struct ExtendedInstructionInner<P: Program> {
    pub program_pubkey: Pubkey,
    pub accounts: Vec<Pubkey>,
    pub instruction: Instruction<P>,
}

impl<P: Program> ExtendedInstructionInner<P> {
    pub fn program_pubkey(&self) -> &Pubkey {
        &self.program_pubkey
    }

    pub fn accounts(&self) -> &Vec<Pubkey> {
        &self.accounts
    }

    pub fn instruction(&self) -> &Instruction<P> {
        &self.instruction
    }
}
