/// Reader module implements the logic to read a buffer of bytes.
pub mod reader;

/// Program module implements the logic to parse instructions.
pub mod program;

/// Error module implements the error types for the library.
pub mod error;

/// Types module implements the types for the library.
pub mod types;

use crate::{error::SoleanaResult, reader::Reader, types::Indicator};

/// [`TransactionsParser`] is a struct that uses a [`Reader`] to parse transactions.
///
/// This struct is used so we can pass various `Program`'s to the parser only once, and then parse various transactions.
pub struct TransactionsParser<'a> {
    pub(crate) reader: Reader<'a>,
}

impl<'a> TransactionsParser<'a> {
    /// Creates a new [`TransactionsParser`] from a buffer of bytes.
    pub fn new() -> Self {
        Self {
            reader: Reader::new_empty(),
        }
    }

    /// Parses a transaction from a hex string.
    ///
    /// TODO: This should either accept a &'a str or a &'a [u8]
    pub fn parse_transaction(&mut self, transaction: &'a str) -> SoleanaResult<types::Transaction> {
        self.reader.set_bytes_from_str(transaction)?;

        let signatures = self.reader.read_signatures()?;
        let indicator = self.reader.indicator()?;
        let header = self.reader.read_header()?;
        let accounts = self.reader.read_accounts()?;
        let hash = self.reader.read_hash()?;
        let instructions = self.reader.read_instructions(&accounts)?;

        let luts: Option<Vec<crate::types::LUT>> = match indicator {
            Indicator::Legacy => None,
            Indicator::V0 => {
                let luts = self.reader.read_luts(&accounts)?;
                Some(luts)
            }
        };

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
