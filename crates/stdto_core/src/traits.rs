use crate::error::*;
use std::io;

use serde::{de::DeserializeOwned, Deserialize, Serialize};

pub trait ToBytes: Serialize {
    #[inline]
    fn try_to_bytes(&self) -> Result<Vec<u8>> {
        bincode::serialize(self).map_err(Error::BytesConversionError)
    }
    #[inline]
    fn to_bytes(&self) -> Vec<u8> {
        self.try_to_bytes().unwrap()
    }
    #[inline]
    fn try_to_bytes_into<T: io::Write>(&self, writer: &mut T) -> Result<()> {
        bincode::serialize_into(writer, self).map_err(Error::BytesConversionError)
    }
    #[inline]
    fn to_bytes_into<T: io::Write>(&self, writer: &mut T) {
        self.try_to_bytes_into(writer).unwrap()
    }

    #[inline]
    fn try_from_bytes<'a>(bytes: &'a [u8]) -> Result<Self>
    where
        Self: Deserialize<'a>,
    {
        bincode::deserialize(bytes).map_err(Error::BytesConversionError)
    }
    #[inline]
    fn from_bytes<'a>(bytes: &'a [u8]) -> Self
    where
        Self: Deserialize<'a>,
    {
        Self::try_from_bytes(bytes).unwrap()
    }
    #[inline]
    fn try_from_bytes_into<T: io::Read>(reader: &mut T) -> Result<Self>
    where
        Self: DeserializeOwned,
    {
        bincode::deserialize_from(reader).map_err(Error::BytesConversionError)
    }
    #[inline]
    fn from_bytes_into<T: io::Read>(reader: &mut T) -> Self
    where
        Self: DeserializeOwned,
    {
        Self::try_from_bytes_into(reader).unwrap()
    }
}
