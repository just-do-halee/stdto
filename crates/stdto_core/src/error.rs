#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("bytes conversion error: {0}")]
    Bytes(#[from] bincode::Error),
    #[error("json conversion error: {0}")]
    Json(#[from] serde_json::Error),
    #[error("io error: {0}")]
    Io(#[from] std::io::Error),
    #[error("fmt error: {0}")]
    Fmt(#[from] std::fmt::Error),
    #[error("utf error: {0}")]
    Utf8(#[from] std::str::Utf8Error),
    #[error("parse int error: {0}")]
    ParseInt(#[from] std::num::ParseIntError),
    #[error("out of bounds error: {0} < {1}")]
    OutOfBounds(usize, usize),
    #[error("odd length")]
    OddLength,
    #[error("try into errpr")]
    TryIntoError,
}

pub type Result<T> = std::result::Result<T, Error>;
