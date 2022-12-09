#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("bytes conversion error: {0}")]
    BytesConversionError(#[from] bincode::Error),
}

pub type Result<T> = std::result::Result<T, Error>;
