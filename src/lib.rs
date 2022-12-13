pub use stdto_core::error;

// ----------------------------------------------------

/// core and derive traits.
pub mod prelude {
    #[cfg(feature = "bytes")]
    pub use stdto_core::ToBytes;
    #[cfg(all(feature = "bytes", feature = "derive"))]
    pub use stdto_derive::ToBytes;

    #[cfg(feature = "hash")]
    pub use stdto_core::ToHash;
    #[cfg(all(feature = "hash", feature = "derive"))]
    pub use stdto_derive::ToHash;

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

#[cfg(feature = "hex")]
pub use stdto_core::HexMode;

// -----------------------------------------------------

// #[cfg(feature = "json")]
// pub use stdto_core::ToJson;
// #[cfg(all(feature = "derive", feature = "json"))]
// pub use stdto_derive::json;
//
