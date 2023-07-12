use std::io;

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("Receiver didn't acknowledged receiving message")]
    NoAck,
    #[error("Message length of {0} exceeded maximum value (u32::MAX == {})", u32::MAX)]
    MessageLimitExceeded(u64),
    #[error("Io Error: {0}")]
    Io(#[from] io::Error),
    #[error("Json Error: {0}")]
    SerdeJson(#[from] serde_json::Error),
}

pub type Result<T, E = Error> = std::result::Result<T, E>;
