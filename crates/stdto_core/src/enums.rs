#![allow(unused)]

use core::fmt;

#[non_exhaustive]
#[derive(Default, Debug, Clone, Copy, PartialEq, Eq)]
pub enum Endian {
    Big,
    #[default]
    Little,
    Native,
}

#[derive(Default, Debug, Clone, Copy, PartialEq, Eq)]
pub enum HexMode {
    #[default]
    Lower,
    Upper,
    Lower0x,
    Upper0x,
}

impl HexMode {
    #[inline]
    pub fn has_0x(&self) -> bool {
        matches!(self, HexMode::Lower0x | HexMode::Upper0x)
    }
    #[inline]
    pub fn is_lower(&self) -> bool {
        matches!(self, HexMode::Lower | HexMode::Lower0x)
    }
    #[inline]
    pub fn is_upper(&self) -> bool {
        matches!(self, HexMode::Upper | HexMode::Upper0x)
    }
}

impl fmt::Display for HexMode {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}
