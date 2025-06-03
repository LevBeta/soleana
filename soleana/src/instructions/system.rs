use solana_sdk::{instruction::CompiledInstruction, pubkey::Pubkey};

use crate::error::{SoleanaError, SoleanaResult};

#[derive(Debug, PartialEq, Eq)]
pub enum SystemInstruction {
    Transfer { lamports: u64 },
}

impl SystemInstruction {
    pub fn parse_instruction(
        instruction: &CompiledInstruction,
        _accounts: &Vec<Pubkey>,
    ) -> SoleanaResult<SystemInstruction> {
        let instruction_id = instruction.data[0..4].to_vec();

        match instruction_id[..] {
            [0x02, 0x00, 0x00, 0x00] => SystemInstruction::parse_transfer(instruction),
            _ => Err(SoleanaError::InvalidInstructionId),
        }
    }

    fn parse_transfer(instruction: &CompiledInstruction) -> SoleanaResult<SystemInstruction> {
        let lamports = u64::from_le_bytes(instruction.data[4..12].try_into().unwrap());
        Ok(SystemInstruction::Transfer { lamports })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use std::str::FromStr;

    #[test]
    fn test_parse_transfer() {
        let instruction = CompiledInstruction {
            program_id_index: 2,
            accounts: vec![0, 1],
            data: vec![2, 0, 0, 0, 32, 215, 163, 35, 0, 0, 0, 0],
        };

        let accounts = vec![
            Pubkey::from_str("8HDCnPt3mTbUC6V4m8NfaETmfvWXHNUmwinTmg4vWtQg").unwrap(),
            Pubkey::from_str("J4NxYyfyYLfCrjiKQ15aXAQ27YamyaWhWp95n54ezEyN").unwrap(),
            Pubkey::from_str("11111111111111111111111111111111").unwrap(),
        ];

        let instruction = SystemInstruction::parse_instruction(&instruction, &accounts);
        assert_eq!(
            instruction,
            Ok(SystemInstruction::Transfer {
                lamports: 597940000
            })
        );
    }
}
