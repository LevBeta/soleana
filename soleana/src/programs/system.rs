use crate::prelude::program_impl::*;

pub(crate) struct System;

impl Program for System {
    fn program_id() -> Pubkey {
        [0; 32]
    }

    type Instructions = SystemInstructions;

    fn parse_instruction(
        _: Pubkey,
        ix_accounts_indexes: &Vec<u8>,
        data: &[u8],
        accounts: &[Pubkey],
    ) -> SoleanaResult<Self::Instructions> {
        let accs = System::match_accounts(ix_accounts_indexes, accounts);
        match data[0..4].to_vec()[..] {
            [0x02, 0x00, 0x00, 0x00] => {
                let lamports = u64::from_le_bytes(data[4..12].try_into().unwrap());
                Ok(SystemInstructions::Transfer {
                    lamports,
                    accounts: SystemTransferAccounts {
                        from: accs[0],
                        to: accs[1],
                    },
                })
            }
            _ => Err(SoleanaError::InvalidInstruction),
        }
    }
}

/// Enum of all the instructions that can be given by the system program (Those who are implemented).
#[derive(Debug)]
pub enum SystemInstructions {
    Transfer {
        lamports: u64,
        accounts: SystemTransferAccounts,
    },
}

impl ProgramInstructions for SystemInstructions {}

#[derive(Debug)]
pub struct SystemTransferAccounts {
    pub from: Pubkey,
    pub to: Pubkey,
}
