/// Reader module implements the logic to read a buffer of bytes.
pub mod reader;

/// Error module implements the error types for the library.
pub mod error;

/// Types module implements the types for the library.
pub mod types;

/// Programs module implements the logic to parse various programs.
pub mod programs;

/// Registry module implements the logic to register programs.
pub(crate) mod registry;

/// Prelude module implements the prelude for the library.
pub mod prelude;

/// TransactionsParser module implements the logic to parse transactions.
use crate::{
    error::SoleanaResult,
    programs::{system::System, Program, ProgramInstructions},
    reader::Reader,
    types::{Indicator, Instruction, Pubkey},
};

use std::collections::HashMap;

/// [`TransactionsParser`] is a struct that uses a [`Reader`] to parse transactions.
///
/// This struct is used so we can pass various `Program`'s to the parser only once, and then parse various transactions.
pub struct TransactionsParser<'a> {
    pub(crate) reader: Reader<'a>,
}

impl<'a> TransactionsParser<'a> {
    /// Creates a new [`TransactionsParser`] from a buffer of bytes.
    pub fn new() -> Self {
        registry::register_program::<System>();

        Self {
            reader: Reader::new_empty(),
        }
    }

    /// Registers a program to the parser.
    pub fn register_program<P: Program>(&self)
    where
        P::Instructions: ProgramInstructions + 'static,
    {
        registry::register_program::<P>();
    }

    /// Registers a lut to the parser.
    pub fn register_lut<T: Into<crate::types::CompleteAddressLookupTable>>(&self, lut: T) {
        registry::register_lut(lut.into());
    }

    /// Registers a lut fetch function to the parser.
    pub fn register_lut_fetch_fn<F, R>(&self, fetch_fn: F)
    where
        F: Fn(&[u8; 32]) -> R + Send + Sync + 'static,
        R: Into<crate::types::CompleteAddressLookupTable>,
    {
        let mut registry = registry::registry().write().unwrap();
        registry.lut_fetch_fn = Some(Box::new(move |key| fetch_fn(key).into()));
    }

    /// Fetches a lut from the fetch function and registers it to the parser.
    pub fn fetch_and_register_lut(&self, lut_account: Pubkey) -> SoleanaResult<()> {
        let registry = registry::registry().read().unwrap();

        let fetch_fn = registry
            .lut_fetch_fn
            .as_ref()
            .ok_or_else(|| crate::error::SoleanaError::NoLutFetchFnRegistered)?;

        self.register_lut((fetch_fn)(&lut_account));

        Ok(())
    }

    fn parse_instructions(
        &self,
        instructions: Vec<(Pubkey, Vec<u8>, Vec<u8>)>,
        accounts: &[Pubkey],
        programs: &HashMap<Pubkey, crate::registry::ParserFn>,
    ) -> SoleanaResult<Vec<Instruction>> {
        instructions
            .iter()
            .map(|(program_id, ix_acc, data)| {
                let parsed: Option<Box<dyn ProgramInstructions + 'static>> =
                    if let Some(parser) = programs.get(program_id) {
                        let instruction = parser(*program_id, ix_acc, data, accounts)?;
                        Some(instruction)
                    } else {
                        None
                    };
                Ok(Instruction {
                    program_id: *program_id,
                    parsed,
                    raw: data.clone(),
                })
            })
            .collect()
    }

    /// Parses a transaction from a hex string.
    ///
    /// TODO: This should either accept a &'a str or a &'a [u8]
    pub fn parse_transaction(&mut self, transaction: &'a str) -> SoleanaResult<types::Transaction> {
        self.reader.set_bytes_from_str(transaction)?;

        let signatures = self.reader.read_signatures()?;
        let indicator = self.reader.indicator()?;
        let header = self.reader.read_header()?;
        let mut accounts = self.reader.read_accounts()?;
        let hash = self.reader.read_hash()?;
        let instructions = self.reader.read_instructions(&accounts)?;

        let luts: Option<Vec<crate::types::LUT>> = match indicator {
            Indicator::Legacy => None,
            Indicator::V0 => Some(self.reader.read_luts(&mut accounts)?),
        };

        let instructions = self.parse_instructions(
            instructions,
            &accounts,
            &registry::registry().read().unwrap().programs,
        )?;

        let transaction = types::Transaction {
            transaction_type: indicator,
            signatures,
            header,
            hash,
            instructions,
            luts,
        };

        Ok(transaction)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_transaction() {
        let mut parser = TransactionsParser::new();
        let transaction = parser.parse_transaction("01c79cc65469fdfcc8fb10150150e33c73220b976162999d1e38a81176de3aaf90af7f39eacbd261932badd65c3551cdac3f1e60585e2c92e3b52f117bac35750680010002040e7698886e86cd5f4faf3ab562b70f97736ffd2c62eaa7bfe194a2021a82d97cbf971b59108b5b85a04fb093f1e21b4e3fd4c4c8f487dd09b95752769f0dd8c300000000000000000000000000000000000000000000000000000000000000000306466fe5211732ffecadba72c39be7bc8ce5bbc5f7126b2c439b3a400000000124ad783cd3b62be732496acc325d8337e80f1fa06d278a9b534f28fe60a4740203000502e8030000020200010c02000000401f00000000000000");
        println!("{:?}", transaction.unwrap());
    }
}
