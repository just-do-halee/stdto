pub use stdto_core::error;

// ----------------------------------------------------

/// core and derive traits.
pub mod prelude {
    #[cfg(feature = "bytes")]
    pub use stdto_core::{ToBytes, ToBytesForRef, ToStringForRef};
    #[cfg(all(feature = "bytes", feature = "derive"))]
    pub use stdto_derive::ToBytes;

    #[cfg(feature = "hash")]
    pub use stdto_core::ToHash;
    #[cfg(all(feature = "hash", feature = "derive"))]
    pub use stdto_derive::ToHash;

    #[cfg(feature = "json")]
    pub use stdto_core::ToJson;
    #[cfg(all(feature = "json", feature = "derive"))]
    pub use stdto_derive::ToJson;

    #[cfg(feature = "hex")]
    pub use stdto_core::ToHex;
}
pub use prelude::*;

// -----------------------------------------------------

#[cfg(feature = "serde")]
pub use stdto_core::serde;
#[cfg(all(feature = "serde", feature = "derive"))]
pub use stdto_derive::serde;

// -----------------------------------------------------

#[cfg(feature = "bytes")]
pub use stdto_core::{Endian, ToBytesOptions};
#[cfg(all(feature = "bytes", feature = "derive"))]
pub use stdto_derive::bytes;

// -----------------------------------------------------

#[cfg(feature = "hash")]
pub use stdto_core::digest;
#[cfg(all(feature = "hash", feature = "derive"))]
pub use stdto_derive::hash;

// -----------------------------------------------------

#[cfg(feature = "json")]
pub use stdto_core::serde_json;
#[cfg(all(feature = "json", feature = "derive"))]
pub use stdto_derive::json;

// -----------------------------------------------------

#[cfg(feature = "hex")]
pub use stdto_core::HexMode;

// -----------------------------------------------------
