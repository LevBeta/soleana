pub type SoleanaResult<T> = Result<T, SoleanaError>;

#[derive(Debug, PartialEq, Eq)]
pub enum SoleanaError {
    InvalidHexString,
    NotEnoughBytes,
    CompactU16Overflow,

    InvalidInstruction,
}
