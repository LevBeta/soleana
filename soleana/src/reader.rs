use crate::{
    error::{SoleanaError, SoleanaResult},
    types::{Hash, Header, Indicator, Pubkey, Signature, LUT},
};
use std::borrow::Cow;

pub struct Reader<'a> {
    bytes: Cow<'a, [u8]>,
    cursor: usize,
}

impl<'a> Reader<'a> {
    /// Creates a new reader from a buffer of bytes.
    #[inline]
    pub fn new(bytes: &'a [u8]) -> Self {
        Self {
            bytes: Cow::Borrowed(bytes),
            cursor: 0,
        }
    }

    /// Creates a new reader in a "empty" state.
    #[inline]
    pub(crate) fn new_empty() -> Self {
        Self {
            bytes: Cow::Borrowed(&[]),
            cursor: 0,
        }
    }

    /// Set the reader to a new buffer of bytes.
    #[inline]
    pub(crate) fn set_bytes(&mut self, bytes: Cow<'a, [u8]>) {
        self.bytes = bytes.into();
        self.cursor = 0;
    }

    /// Set the reader to a new buffer of bytes from a string.
    ///
    /// The string is expected to be a hex string.
    #[inline]
    pub(crate) fn set_bytes_from_str(&mut self, transaction: &'a str) -> SoleanaResult<()> {
        let bytes = (0..transaction.len())
            .step_by(2)
            .map(|i| {
                u8::from_str_radix(&transaction[i..i + 2], 16)
                    .map_err(|_| SoleanaError::InvalidHexString)
            })
            .collect::<SoleanaResult<Vec<u8>>>()?;

        self.set_bytes(bytes.into());
        Ok(())
    }

    /// Reads a sequence of bytes from the buffer.
    fn read_bytes(&mut self, count: usize) -> SoleanaResult<&[u8]> {
        let end = self.cursor + count;
        let slice = self
            .bytes
            .get(self.cursor..end)
            .ok_or(SoleanaError::NotEnoughBytes)?;
        self.cursor = end;
        Ok(slice)
    }

    /// Reads a single byte from the buffer.
    fn read_byte(&mut self) -> SoleanaResult<u8> {
        self.read_bytes(1).map(|bytes| bytes[0])
    }

    /// Peeks a single byte from the buffer.
    fn peek_byte(&self) -> Option<u8> {
        self.bytes.get(self.cursor).copied()
    }

    /// Reads a compact u16 from the buffer.
    fn read_compact_u16(&mut self) -> SoleanaResult<u16> {
        let mut value: u16 = 0;
        let mut shift: u8 = 0;

        for i in 0..3 {
            let byte = self.read_byte()?;

            if i == 2 && (byte & 0x7f) != 0 {
                return Err(SoleanaError::CompactU16Overflow);
            }

            value |= u16::from(byte & 0x7f) << shift;

            if byte & 0x80 == 0 {
                break;
            }

            shift += 7;
        }

        Ok(value)
    }

    fn read_compact_array(&mut self) -> SoleanaResult<Vec<u8>> {
        let len = self.read_compact_u16()?;
        self.read_bytes(len as usize).map(|bytes| bytes.to_vec())
    }

    /// Reads the indicator from the buffer.
    ///
    /// Either `Legacy`(0x00) or `V0`(0x80).
    pub(crate) fn indicator(&mut self) -> SoleanaResult<Indicator> {
        match self.peek_byte() {
            Some(0x80) => {
                self.read_byte()?;
                Ok(Indicator::V0)
            }
            _ => Ok(Indicator::Legacy),
        }
    }

    /// Reads the signatures from the buffer.
    ///
    /// The number of signatures is read from the buffer using a compact u16.
    /// Each signature is read from the buffer using a 64-byte slice.
    pub(crate) fn read_signatures(&mut self) -> SoleanaResult<Vec<Signature>> {
        (0..self.read_compact_u16()? as usize)
            .map(|_| {
                self.read_bytes(64)?
                    .try_into()
                    .map_err(|_| SoleanaError::NotEnoughBytes)
            })
            .collect()
    }

    /// Reads the header from the buffer.
    ///
    /// The header is read from the buffer using a 3-byte slice.
    pub(crate) fn read_header(&mut self) -> SoleanaResult<Header> {
        let header_bytes = self.read_bytes(Header::BYTE_SIZE)?;

        let num_required_signatures = header_bytes[0];
        let num_readonly_signed_accounts = header_bytes[1];
        let num_readonly_unsigned_accounts = header_bytes[2];

        Ok(Header {
            num_required_signatures,
            num_readonly_signed_accounts,
            num_readonly_unsigned_accounts,
        })
    }

    /// Reads the accounts from the buffer.
    ///
    /// The accounts are read from the buffer using a 32-byte slice.
    pub(crate) fn read_accounts(&mut self) -> SoleanaResult<Vec<Pubkey>> {
        (0..self.read_compact_u16()? as usize)
            .map(|_| {
                self.read_bytes(32)?
                    .try_into()
                    .map_err(|_| SoleanaError::NotEnoughBytes)
            })
            .collect()
    }

    /// Reads a hash from the buffer.
    ///
    /// The hash is read from the buffer using a 32-byte slice.
    pub(crate) fn read_hash(&mut self) -> SoleanaResult<Hash> {
        self.read_bytes(32)?
            .try_into()
            .map_err(|_| SoleanaError::NotEnoughBytes)
    }

    /// Reads the instructions from the buffer.
    ///
    /// We don't parse them at this point, we actually wait to read the lut's
    /// before we parse them, since there is accounts there we will need
    /// to parse the instructions.
    pub(crate) fn read_instructions(
        &mut self,
        accounts: &[Pubkey],
    ) -> SoleanaResult<Vec<(Pubkey, Vec<u8>, Vec<u8>)>> {
        (0..self.read_byte()? as usize)
            .map(|_| {
                let program_id = accounts[self.read_byte()? as usize];
                let ix_acc = self.read_compact_array()?;
                let data = self.read_compact_array()?;
                Ok((program_id, ix_acc, data))
            })
            .collect()
    }

    /// Reads the LUTs from the buffer and extends the provided accounts vector with writable and readonly accounts.
    pub(crate) fn read_luts(&mut self, accounts: &mut Vec<Pubkey>) -> SoleanaResult<Vec<LUT>> {
        let mut writable_accounts = Vec::new();
        let mut readonly_accounts = Vec::new();

        let registry = crate::registry::registry().read().unwrap();

        let luts = (0..self.read_byte()? as usize)
            .map(|_| {
                let pk_bytes = self.read_bytes(32)?;
                let pk: Pubkey = pk_bytes
                    .try_into()
                    .map_err(|_| SoleanaError::NotEnoughBytes)?;

                let writable_indexes = self.read_compact_array()?;
                let readonly_indexes = self.read_compact_array()?;

                if let Some(lut) = registry.luts.get(&pk) {
                    writable_accounts.extend(writable_indexes.iter().map(|&i| lut[i as usize]));
                    readonly_accounts.extend(readonly_indexes.iter().map(|&i| lut[i as usize]));
                }

                Ok(LUT {
                    account_key: pk,
                    writable_indexes,
                    readonly_indexes,
                })
            })
            .collect::<SoleanaResult<Vec<LUT>>>()?;

        accounts.extend(writable_accounts);
        accounts.extend(readonly_accounts);

        Ok(luts)
    }
}
