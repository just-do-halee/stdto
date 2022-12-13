mod enums;
mod traits;

pub mod error;

#[cfg(feature = "hash")]
pub extern crate digest;
#[cfg(feature = "serde")]
pub extern crate serde;

#[cfg(feature = "bytes")]
pub use crate::{
    enums::{Endian, HexMode},
    traits::{ToBytes, ToBytesOptions},
};

#[cfg(feature = "hash")]
pub use crate::traits::ToHash;

#[cfg(feature = "hex")]
pub use crate::traits::ToHex;
