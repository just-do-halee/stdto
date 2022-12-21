#[non_exhaustive]
#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[cfg(feature = "bytes")]
    #[error("bytes conversion error: {0}")]
    Bytes(#[from] bincode::Error),

    #[cfg(feature = "json")]
    #[error("json conversion error: {0}")]
    Json(#[from] serde_json::Error),

    #[cfg(feature = "yaml")]
    #[error("yaml conversion error: {0}")]
    Yaml(#[from] serde_yaml::Error),

    #[cfg(feature = "toml")]
    #[error("toml conversion error: {0}")]
    TomlSerialize(#[from] toml::ser::Error),
    #[cfg(feature = "toml")]
    #[error("toml conversion error: {0}")]
    TomlDeserialize(#[from] toml::de::Error),

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
}

pub type Result<T> = std::result::Result<T, Error>;
