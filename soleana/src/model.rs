use crate::{
    error::SoleanaResult, instructions::parse_instruction, instructions::system::SystemInstruction,
};
use solana_message::MessageHeader;
use solana_sdk::{instruction::CompiledInstruction, pubkey::Pubkey, signature::Signature};

#[derive(Debug, PartialEq, Eq)]
pub struct ExtendedTransaction {
    pub signatures: Vec<Signature>,
    pub header: MessageHeader,
    pub accounts: Vec<Pubkey>,
    pub instructions: Vec<ExtendedInstruction>,
}

impl ExtendedTransaction {
    pub fn instructions(&self) -> &Vec<ExtendedInstruction> {
        &self.instructions
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct ExtendedInstruction {
    pub raw: CompiledInstruction,
    pub inner: Option<ExtendedInstructionInner>,
}

impl ExtendedInstruction {
    pub fn new(instruction: &CompiledInstruction, accounts: &Vec<Pubkey>) -> SoleanaResult<Self> {
        let parsed_instruction = parse_instruction(instruction, accounts)?;

        let inner = match parsed_instruction {
            Instruction::System(_) => Some(ExtendedInstructionInner {
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

    pub fn parsed_instruction(&self) -> Option<&ExtendedInstructionInner> {
        self.inner.as_ref()
    }

    pub fn raw_instruction(&self) -> &CompiledInstruction {
        &self.raw
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum Instruction {
    System(SystemInstruction),
    Unknown,
}

#[derive(Debug, PartialEq, Eq)]
pub struct ExtendedInstructionInner {
    pub program_pubkey: Pubkey,
    pub accounts: Vec<Pubkey>,
    pub instruction: Instruction,
}

impl ExtendedInstructionInner {
    pub fn program_pubkey(&self) -> &Pubkey {
        &self.program_pubkey
    }

    pub fn accounts(&self) -> &Vec<Pubkey> {
        &self.accounts
    }

    pub fn instruction(&self) -> &Instruction {
        &self.instruction
    }
}
