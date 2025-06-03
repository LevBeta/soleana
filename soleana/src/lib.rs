/// Error module
pub mod error;

/// Reader module
pub mod reader;

/// Instructions module
#[cfg(feature = "extended")]
pub mod instructions;

#[cfg(feature = "extended")]
pub mod model;

use crate::error::SoleanaResult;
use crate::reader::TxReader;

#[cfg(feature = "extended")]
use crate::model::ExtendedTransaction;

use solana_message::{
    legacy::Message as LegacyMessage, v0::Message as V0Message, VersionedMessage,
};

/// Type of a transaction
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum TransactionType {
    V0,
    Legacy,
}

#[cfg(feature = "extended")]
pub fn extended_parse(transaction: &str) -> SoleanaResult<ExtendedTransaction> {
    let binding: Vec<u8> = (0..transaction.len())
        .step_by(2)
        .map(|i| u8::from_str_radix(&transaction[i..i + 2], 16).unwrap())
        .collect();

    let mut reader = TxReader::new(&binding);
    let signatures = reader.read_signatures()?;
    let indicator_byte = reader.indicator_byte()?;
    let message = match indicator_byte {
        TransactionType::V0 => todo!(),
        TransactionType::Legacy => {
            let header = reader.read_header()?;
            let accounts = reader.read_accounts()?;
            let hash = reader.read_hash()?;
            let instructions = reader.read_instructions()?;
            println!("instructions: {:?}", instructions);
            let extended_instructions = instructions
                .iter()
                .map(|instruction| model::ExtendedInstruction::new(instruction, &accounts).unwrap())
                .collect();
            Ok(ExtendedTransaction {
                signatures,
                header,
                accounts,
                instructions: extended_instructions,
            })
        }
    };

    Ok(message?)
}

/// Parse a transaction into a VersionedMessage
pub fn parse(transaction: &str) -> SoleanaResult<VersionedMessage> {
    let binding: Vec<u8> = (0..transaction.len())
        .step_by(2)
        .map(|i| u8::from_str_radix(&transaction[i..i + 2], 16).unwrap())
        .collect();

    let mut reader = TxReader::new(&binding);
    let _ = reader.read_signatures()?;
    let indicator_byte = reader.indicator_byte()?;
    let message = match indicator_byte {
        TransactionType::V0 => {
            let header = reader.read_header()?;
            let accounts = reader.read_accounts()?;
            let hash = reader.read_hash()?;
            let instructions = reader.read_instructions()?;
            let luts = reader.read_luts()?;
            Ok(VersionedMessage::V0(V0Message {
                header: header,
                account_keys: accounts,
                recent_blockhash: hash,
                instructions: instructions,
                address_table_lookups: luts,
            }))
        }
        TransactionType::Legacy => {
            let header = reader.read_header()?;
            let accounts = reader.read_accounts()?;
            let hash = reader.read_hash()?;
            let instructions = reader.read_instructions()?;
            Ok(VersionedMessage::Legacy(LegacyMessage {
                header: header,
                account_keys: accounts,
                recent_blockhash: hash,
                instructions: instructions,
            }))
        }
    };

    Ok(message?)
}
