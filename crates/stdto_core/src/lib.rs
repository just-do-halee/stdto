mod enums;
mod traits;

pub mod error;

#[cfg(feature = "hash")]
pub extern crate digest;
#[cfg(feature = "serde")]
pub extern crate serde;
#[cfg(feature = "json")]
pub extern crate serde_json;
#[cfg(feature = "yaml")]
pub extern crate serde_yaml;
#[cfg(feature = "toml")]
pub extern crate toml as serde_toml;

pub use traits::AsBytes;

#[cfg(feature = "bytes")]
pub use crate::{
    enums::{Endian, HexMode},
    traits::{ToBytes, ToBytesOptions, ToStringForBytes},
};

#[cfg(feature = "hash")]
pub use crate::traits::ToHash;

#[cfg(feature = "json")]
pub use crate::{serde_json::Value as JsonValue, traits::ToJson};

#[cfg(feature = "yaml")]
pub use crate::{serde_yaml::Value as YamlValue, traits::ToYaml};

#[cfg(feature = "toml")]
pub use crate::{serde_toml::Value as TomlValue, traits::ToToml};

#[cfg(feature = "hex")]
pub use crate::traits::ToHex;
