pub use stdto_core::error;

// -----------------------------------------------------

#[cfg(feature = "hex")]
pub use stdto_core::{HexMode, ToHex};

// -----------------------------------------------------

#[cfg(feature = "bytes")]
pub use stdto_core::ToBytes;

#[cfg(all(feature = "bytes", feature = "derive"))]
pub use stdto_core::serde;
#[cfg(all(feature = "bytes", feature = "derive"))]
pub use stdto_derive::{bytes, serde, ToBytes};

// -----------------------------------------------------

#[cfg(feature = "hash")]
pub use stdto_core::{digest, ToHash};

#[cfg(all(feature = "hash", feature = "derive"))]
pub use stdto_derive::{hash, ToHash};

// -----------------------------------------------------

// #[cfg(feature = "json")]
// pub use stdto_core::ToJson;
// #[cfg(all(feature = "derive", feature = "json"))]
// pub use stdto_derive::json;
