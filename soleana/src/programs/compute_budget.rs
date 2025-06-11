use crate::prelude::program_impl::*;

pub(crate) struct ComputeBudget;

impl Program for ComputeBudget {
    fn program_id() -> Pubkey {
        [
            3, 6, 70, 111, 229, 33, 23, 50, 255, 236, 173, 186, 114, 195, 155, 231, 188, 140, 229,
            187, 197, 247, 18, 107, 44, 67, 155, 58, 64, 0, 0, 0,
        ]
    }

    type Instructions = ComputeBudgetInstructions;

    fn parse_instruction(
        _: Pubkey,
        _: &Vec<u8>,
        data: &[u8],
        _: &[Pubkey],
    ) -> SoleanaResult<Self::Instructions> {
        match data[0..1].to_vec()[..] {
            [0x02] => Ok(ComputeBudgetInstructions::SetComputeUnitLimit {
                units: u32::from_le_bytes(data[1..5].try_into().unwrap()),
            }),
            [0x03] => Ok(ComputeBudgetInstructions::SetComputeUnitPrice {
                micro_lamports: u64::from_le_bytes(data[1..9].try_into().unwrap()),
            }),
            _ => Err(SoleanaError::InvalidInstruction),
        }
    }
}

#[derive(Debug)]
pub enum ComputeBudgetInstructions {
    SetComputeUnitLimit { units: u32 },
    SetComputeUnitPrice { micro_lamports: u64 },
}

impl ProgramInstructions for ComputeBudgetInstructions {}
