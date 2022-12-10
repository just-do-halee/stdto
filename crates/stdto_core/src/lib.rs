mod enums;
mod traits;

pub use enums::HexMode;
pub use traits::{ToBytes, ToHash, ToHex};

pub mod error;

pub extern crate digest;
pub extern crate serde;
