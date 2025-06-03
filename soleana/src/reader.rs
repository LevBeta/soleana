use crate::{
    error::{SoleanaError, SoleanaResult},
    TransactionType,
};
use solana_message::{v0::MessageAddressTableLookup, MessageHeader};
use solana_sdk::{
    hash::Hash, instruction::CompiledInstruction, pubkey::Pubkey, signature::Signature,
};

const SIGNATURE_BYTE_SIZE: usize = 64;
const ACCOUNT_BYTE_SIZE: usize = 32;
const HEADER_BYTE_SIZE: usize = 3;

pub(crate) struct TxReader<'a> {
    bytes: &'a [u8],
    cursor: usize,
}

impl<'a> TxReader<'a> {
    pub fn new(bytes: &'a [u8]) -> Self {
        Self { bytes, cursor: 0 }
    }

    /// Read a fixed number of bytes from the current cursor position
    fn read_bytes(&mut self, len: usize) -> SoleanaResult<&'a [u8]> {
        if self.cursor + len > self.bytes.len() {
            return Err(SoleanaError::ReaderError(format!(
                "Not enough bytes to read. Expected {} bytes, got {}",
                len,
                self.bytes.len() - self.cursor
            )));
        }

        let result = &self.bytes[self.cursor..self.cursor + len];
        self.cursor += len;
        Ok(result)
    }

    /// Read a single byte from the current cursor position
    fn read_u8(&mut self) -> SoleanaResult<u8> {
        let bytes = self.read_bytes(1)?;
        Ok(bytes[0])
    }

    /// Read a single byte from the current cursor position without moving the cursor
    pub fn peek_u8(&self) -> SoleanaResult<u8> {
        Ok(self.bytes[self.cursor])
    }

    fn read_compact_u16(&mut self) -> SoleanaResult<u16> {
        let mut value: u16 = 0;
        let mut shift: u8 = 0;

        // Iter over at maximum 3 bytes, compact u16 encoding maxes out at 3 bytes
        for i in 0..3 {
            let byte = self.read_u8()?;

            // If we're on the last byte and it doesn't have the high 2 bits set, return an error
            if i == 2 && (byte & 0x7f) != 0 {
                return Err(SoleanaError::ReaderError(
                    "Compact u16 overflow 1".to_string(),
                ));
            }

            // Mask the continuation bit and shift the value left by the number of bits we've shifted so far
            value |= u16::from(byte & 0x7F) << shift;

            // If the continuation bit is not set, we're done
            if byte & 0x80 == 0 {
                return Ok(value);
            }

            // Shift the value left by 7 bits and increment the shift count
            shift += 7;
        }

        Err(SoleanaError::ReaderError(
            "Compact u16 overflow 2".to_string(),
        ))
    }

    fn read_compact_array(&mut self) -> SoleanaResult<Vec<u8>> {
        let len = self.read_compact_u16()?;
        self.read_bytes(len as usize).map(|bytes| bytes.to_vec())
    }

    /// Read the type of the transaction
    ///
    /// Only reads the next byte and moves the cursor forward if the byte is 0x80
    pub fn indicator_byte(&mut self) -> SoleanaResult<TransactionType> {
        match self.peek_u8()? {
            0x80 => {
                self.read_u8()?;
                Ok(TransactionType::V0)
            }
            _ => Ok(TransactionType::Legacy),
        }
    }

    /// Read `SIGNATURE_BYTE_SIZE`-bytes signatures starting from the current cursor position
    ///
    /// First byte is the quantity of signatures
    pub fn read_signatures(&mut self) -> SoleanaResult<Vec<Signature>> {
        (0..self.read_compact_u16()? as usize)
            .map(|_| {
                Signature::try_from(self.read_bytes(SIGNATURE_BYTE_SIZE)?).map_err(|_| {
                    SoleanaError::ReaderError("Failed to convert bytes to signature".to_string())
                })
            })
            .collect()
    }

    /// Read the header of the transaction
    ///
    /// The header is 3 bytes long and contains the following information:
    /// - Number of required signatures
    /// - Number of readonly signed accounts
    /// - Number of readonly unsigned accounts
    pub fn read_header(&mut self) -> SoleanaResult<MessageHeader> {
        let header_bytes = self.read_bytes(HEADER_BYTE_SIZE)?;
        let header = MessageHeader {
            num_required_signatures: header_bytes[0],
            num_readonly_signed_accounts: header_bytes[1],
            num_readonly_unsigned_accounts: header_bytes[2],
        };
        Ok(header)
    }

    /// Read the accounts of the transaction
    ///
    /// The accounts are the accounts that are used in the transaction
    ///
    /// The number of accounts is the first byte
    pub fn read_accounts(&mut self) -> SoleanaResult<Vec<Pubkey>> {
        (0..self.read_u8()? as usize)
            .map(|_| {
                Pubkey::try_from(self.read_bytes(ACCOUNT_BYTE_SIZE)?).map_err(|_| {
                    SoleanaError::ReaderError("Failed to convert bytes to pubkey".to_string())
                })
            })
            .collect()
    }

    /// Read the hash of the transaction
    ///
    /// The hash is 32 bytes long
    pub fn read_hash(&mut self) -> SoleanaResult<Hash> {
        Ok(Hash::new_from_array(
            self.read_bytes(32)?
                .try_into()
                .map_err(|_| SoleanaError::ReaderError("Invalid hash length".to_string()))?,
        ))
    }

    /// Read the instructions of the transaction
    ///
    /// The instructions are the instructions that are used in the transaction
    ///
    /// The number of instructions is the first byte
    pub fn read_instructions(&mut self) -> SoleanaResult<Vec<CompiledInstruction>> {
        (0..self.read_u8()? as usize)
            .map(|_| {
                let index = self.read_u8()?;
                let accounts = self.read_compact_array()?;
                let data = self.read_compact_array()?;
                Ok(CompiledInstruction {
                    program_id_index: index,
                    accounts,
                    data,
                })
            })
            .collect::<SoleanaResult<Vec<CompiledInstruction>>>()
    }

    pub fn read_luts(&mut self) -> SoleanaResult<Vec<MessageAddressTableLookup>> {
        (0..self.read_u8()? as usize)
            .map(|_| {
                let pk = Pubkey::try_from(self.read_bytes(ACCOUNT_BYTE_SIZE)?).map_err(|_| {
                    SoleanaError::ReaderError("Failed to convert bytes to pubkey".to_string())
                })?;
                Ok(MessageAddressTableLookup {
                    account_key: pk,
                    writable_indexes: self.read_compact_array()?,
                    readonly_indexes: self.read_compact_array()?,
                })
            })
            .collect::<SoleanaResult<Vec<MessageAddressTableLookup>>>()
    }
}
