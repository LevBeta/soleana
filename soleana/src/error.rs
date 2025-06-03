use thiserror::Error;

pub type SoleanaResult<T> = Result<T, SoleanaError>;

#[derive(Error, Debug, PartialEq, Eq)]
pub enum SoleanaError {
    #[error("Invalid transaction: {0}")]
    InvalidTransaction(String),

    #[error("Invalid length: {0}")]
    InvalidLength(String),

    #[error("Reader error | Not enough bytes to read: {0}")]
    ReaderError(String),

    #[error("Invalid signature")]
    InvalidSignature,

    #[error("Invalid instruction id")]
    InvalidInstructionId,
}
