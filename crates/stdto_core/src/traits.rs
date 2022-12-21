#![allow(unused_imports, unused_macros)]

use crate::{
    enums::{Endian, HexMode},
    error::*,
};
use std::{fmt, io};

#[cfg(feature = "serde")]
use serde::{de::DeserializeOwned, Serialize};

#[cfg(feature = "bytes")]
use bincode::Options;

#[cfg(feature = "hash")]
use digest::{Digest, Output};

#[cfg(feature = "json")]
use serde_json::Value;

macro_rules! serialize {
    (data: $self:expr, option: $option:expr) => {
        $option
            .with_fixint_encoding()
            .allow_trailing_bytes()
            .serialize($self)
            .map_err(Error::Bytes)
    };
    (data: $self:expr, writer: $writer:expr, option: $option:expr) => {
        $option
            .with_fixint_encoding()
            .serialize_into($writer, $self)
            .map_err(Error::Bytes)
    };
}

macro_rules! deserialize {
    (data: $bytes:expr, option: $option:expr) => {
        $option
            .with_fixint_encoding()
            .allow_trailing_bytes()
            .deserialize($bytes)
            .map_err(Error::Bytes)
    };
    (reader: $reader:expr, option: $option:expr) => {
        $option
            .with_fixint_encoding()
            .allow_trailing_bytes()
            .deserialize_from($reader)
            .map_err(Error::Bytes)
    };
}

#[cfg(feature = "bytes")]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ToBytesOptions {
    pub endian: Endian,
}
#[cfg(feature = "bytes")]
impl ToBytesOptions {
    #[inline]
    pub const fn default() -> Self {
        ToBytesOptions {
            endian: Endian::Little,
        }
    }
}

#[cfg(feature = "bytes")]
/// # A trait that can de/serialize something with bytes. (default: little endian)
pub trait ToBytes {
    const OPTIONS: ToBytesOptions = ToBytesOptions {
        ..ToBytesOptions::default()
    };

    /// Serialize to bytes.
    #[inline]
    fn try_to_be_bytes(&self) -> Result<Vec<u8>>
    where
        Self: Serialize,
    {
        serialize!(data: self, option: bincode::options().with_big_endian())
    }
    #[inline]
    fn try_to_le_bytes(&self) -> Result<Vec<u8>>
    where
        Self: Serialize,
    {
        serialize!(data: self, option: bincode::options().with_little_endian())
    }
    #[inline]
    fn try_to_ne_bytes(&self) -> Result<Vec<u8>>
    where
        Self: Serialize,
    {
        serialize!(data: self, option: bincode::options().with_native_endian())
    }
    #[inline]
    fn try_to_be_bytes_into(&self, writer: impl io::Write) -> Result<()>
    where
        Self: Serialize,
    {
        serialize!(data: self, writer: writer, option: bincode::options().with_big_endian())
    }
    #[inline]
    fn try_to_le_bytes_into(&self, writer: impl io::Write) -> Result<()>
    where
        Self: Serialize,
    {
        serialize!(data: self, writer: writer, option: bincode::options().with_little_endian())
    }
    #[inline]
    fn try_to_ne_bytes_into(&self, writer: impl io::Write) -> Result<()>
    where
        Self: Serialize,
    {
        serialize!(data: self, writer: writer, option: bincode::options().with_native_endian())
    }
    // ---------------------
    #[inline]
    fn to_be_bytes(&self) -> Vec<u8>
    where
        Self: Serialize,
    {
        self.try_to_be_bytes().unwrap()
    }
    #[inline]
    fn to_le_bytes(&self) -> Vec<u8>
    where
        Self: Serialize,
    {
        self.try_to_le_bytes().unwrap()
    }
    #[inline]
    fn to_ne_bytes(&self) -> Vec<u8>
    where
        Self: Serialize,
    {
        self.try_to_ne_bytes().unwrap()
    }
    #[inline]
    fn to_be_bytes_into(&self, writer: impl io::Write)
    where
        Self: Serialize,
    {
        self.try_to_be_bytes_into(writer).unwrap()
    }
    #[inline]
    fn to_le_bytes_into(&self, writer: impl io::Write)
    where
        Self: Serialize,
    {
        self.try_to_le_bytes_into(writer).unwrap()
    }
    #[inline]
    fn to_ne_bytes_into(&self, writer: impl io::Write)
    where
        Self: Serialize,
    {
        self.try_to_ne_bytes_into(writer).unwrap()
    }
    // ---------------------

    /// Deserialize from bytes.
    #[inline]
    fn try_from_be_bytes(bytes: impl AsRef<[u8]>) -> Result<Self>
    where
        Self: DeserializeOwned,
    {
        deserialize!(data: bytes.as_ref(), option: bincode::options().with_big_endian())
    }
    #[inline]
    fn try_from_le_bytes(bytes: impl AsRef<[u8]>) -> Result<Self>
    where
        Self: DeserializeOwned,
    {
        deserialize!(data: bytes.as_ref(), option: bincode::options().with_little_endian())
    }
    #[inline]
    fn try_from_ne_bytes(bytes: impl AsRef<[u8]>) -> Result<Self>
    where
        Self: DeserializeOwned,
    {
        deserialize!(data: bytes.as_ref(), option: bincode::options().with_native_endian())
    }
    #[inline]
    fn try_from_be_bytes_from(reader: impl io::Read) -> Result<Self>
    where
        Self: DeserializeOwned,
    {
        deserialize!(reader: reader, option: bincode::options().with_big_endian())
    }
    #[inline]
    fn try_from_le_bytes_from(reader: impl io::Read) -> Result<Self>
    where
        Self: DeserializeOwned,
    {
        deserialize!(reader: reader, option: bincode::options().with_little_endian())
    }
    #[inline]
    fn try_from_ne_bytes_from(reader: impl io::Read) -> Result<Self>
    where
        Self: DeserializeOwned,
    {
        deserialize!(reader: reader, option: bincode::options().with_native_endian())
    }
    // ---------------------
    #[inline]
    fn from_be_bytes(bytes: impl AsRef<[u8]>) -> Self
    where
        Self: DeserializeOwned,
    {
        Self::try_from_be_bytes(bytes).unwrap()
    }
    #[inline]
    fn from_le_bytes(bytes: impl AsRef<[u8]>) -> Self
    where
        Self: DeserializeOwned,
    {
        Self::try_from_le_bytes(bytes).unwrap()
    }
    #[inline]
    fn from_ne_bytes(bytes: impl AsRef<[u8]>) -> Self
    where
        Self: DeserializeOwned,
    {
        Self::try_from_ne_bytes(bytes).unwrap()
    }
    #[inline]
    fn from_be_bytes_from(reader: impl io::Read) -> Self
    where
        Self: DeserializeOwned,
    {
        Self::try_from_be_bytes_from(reader).unwrap()
    }
    #[inline]
    fn from_le_bytes_from(reader: impl io::Read) -> Self
    where
        Self: DeserializeOwned,
    {
        Self::try_from_le_bytes_from(reader).unwrap()
    }
    #[inline]
    fn from_ne_bytes_from(reader: impl io::Read) -> Self
    where
        Self: DeserializeOwned,
    {
        Self::try_from_ne_bytes_from(reader).unwrap()
    }

    // ------------- default endians -------------
    #[inline]
    fn try_to_bytes(&self) -> Result<Vec<u8>>
    where
        Self: Serialize,
    {
        match Self::OPTIONS.endian {
            Endian::Big => self.try_to_be_bytes(),
            Endian::Little => self.try_to_le_bytes(),
            Endian::Native => self.try_to_ne_bytes(),
        }
    }
    #[inline]
    fn try_to_bytes_into(&self, writer: impl io::Write) -> Result<()>
    where
        Self: Serialize,
    {
        match Self::OPTIONS.endian {
            Endian::Big => self.try_to_be_bytes_into(writer),
            Endian::Little => self.try_to_le_bytes_into(writer),
            Endian::Native => self.try_to_ne_bytes_into(writer),
        }
    }
    #[inline]
    fn try_from_bytes(bytes: impl AsRef<[u8]>) -> Result<Self>
    where
        Self: DeserializeOwned,
    {
        match Self::OPTIONS.endian {
            Endian::Big => Self::try_from_be_bytes(bytes),
            Endian::Little => Self::try_from_le_bytes(bytes),
            Endian::Native => Self::try_from_ne_bytes(bytes),
        }
    }
    #[inline]
    fn try_from_bytes_from(reader: impl io::Read) -> Result<Self>
    where
        Self: DeserializeOwned,
    {
        match Self::OPTIONS.endian {
            Endian::Big => Self::try_from_be_bytes_from(reader),
            Endian::Little => Self::try_from_le_bytes_from(reader),
            Endian::Native => Self::try_from_ne_bytes_from(reader),
        }
    }
    // --------------------------------------------------
    #[inline]
    fn to_bytes(&self) -> Vec<u8>
    where
        Self: Serialize,
    {
        self.try_to_bytes().unwrap()
    }
    #[inline]
    fn to_bytes_into(&self, writer: impl io::Write)
    where
        Self: Serialize,
    {
        self.try_to_bytes_into(writer).unwrap()
    }
    #[inline]
    fn from_bytes(bytes: impl AsRef<[u8]>) -> Self
    where
        Self: DeserializeOwned,
    {
        Self::try_from_bytes(bytes).unwrap()
    }
    #[inline]
    fn from_bytes_from(reader: impl io::Read) -> Self
    where
        Self: DeserializeOwned,
    {
        Self::try_from_bytes_from(reader).unwrap()
    }
}

#[cfg(feature = "hash")]
/// # A trait that can hash bytes.
pub trait ToHash: ToBytes {
    #[inline]
    fn try_to_hash<T: Digest + io::Write>(&self) -> Result<Output<T>>
    where
        Self: Serialize,
    {
        let mut hasher = T::new();
        self.try_to_hash_into(&mut hasher)?;
        Ok(hasher.finalize())
    }
    #[inline]
    fn try_to_hash_into<T: Digest + io::Write>(&self, hasher: &mut T) -> Result<()>
    where
        Self: Serialize,
    {
        self.try_to_bytes_into(hasher)
    }
    #[inline]
    fn to_hash<T: Digest + io::Write>(&self) -> Output<T>
    where
        Self: Serialize,
    {
        let mut hasher = T::new();
        self.to_hash_into(&mut hasher);
        hasher.finalize()
    }
    #[inline]
    fn to_hash_into<T: Digest + io::Write>(&self, hasher: &mut T)
    where
        Self: Serialize,
    {
        self.to_bytes_into(hasher)
    }
}

#[cfg(feature = "json")]
/// # A trait that can be converted to a hex string.
pub trait ToJson {
    // --- Value ----
    #[inline]
    fn try_to_json_value(&self) -> Result<Value>
    where
        Self: Serialize,
    {
        serde_json::to_value(self).map_err(Error::Json)
    }
    #[inline]
    fn try_to_json_value_into(&self, writer: impl io::Write) -> Result<()>
    where
        Self: Serialize,
    {
        serde_json::to_writer(writer, self).map_err(Error::Json)
    }
    #[inline]
    fn try_from_json_value(json: impl AsRef<[u8]>) -> Result<Value>
    where
        Self: DeserializeOwned,
    {
        serde_json::from_slice(json.as_ref()).map_err(Error::Json)
    }
    #[inline]
    fn try_from_json_value_from(reader: impl io::Read) -> Result<Value>
    where
        Self: DeserializeOwned,
    {
        serde_json::from_reader(reader).map_err(Error::Json)
    }
    // --------------
    #[inline]
    fn to_json_value(&self) -> Value
    where
        Self: Serialize,
    {
        self.try_to_json_value().unwrap()
    }
    #[inline]
    fn to_json_value_into(&self, writer: impl io::Write)
    where
        Self: Serialize,
    {
        self.try_to_json_value_into(writer).unwrap()
    }
    #[inline]
    fn from_json_value(json: impl AsRef<[u8]>) -> Value
    where
        Self: DeserializeOwned,
    {
        Self::try_from_json_value(json).unwrap()
    }
    #[inline]
    fn from_json_value_from(reader: impl io::Read) -> Value
    where
        Self: DeserializeOwned,
    {
        Self::try_from_json_value_from(reader).unwrap()
    }
    // --------------

    // --- String ----
    #[inline]
    fn try_to_json(&self) -> Result<String>
    where
        Self: Serialize,
    {
        serde_json::to_string(self).map_err(Error::Json)
    }
    #[inline]
    fn try_to_json_into(&self, writer: impl io::Write) -> Result<()>
    where
        Self: Serialize,
    {
        serde_json::to_writer(writer, self).map_err(Error::Json)
    }
    #[inline]
    fn try_from_json(json: impl AsRef<[u8]>) -> Result<Self>
    where
        Self: DeserializeOwned,
    {
        serde_json::from_slice(json.as_ref()).map_err(Error::Json)
    }
    #[inline]
    fn try_from_json_from(reader: impl io::Read) -> Result<Self>
    where
        Self: DeserializeOwned,
    {
        serde_json::from_reader(reader).map_err(Error::Json)
    }
    // --------------
    #[inline]
    fn to_json(&self) -> String
    where
        Self: Serialize,
    {
        self.try_to_json().unwrap()
    }
    #[inline]
    fn to_json_into(&self, writer: impl io::Write)
    where
        Self: Serialize,
    {
        self.try_to_json_into(writer).unwrap()
    }
    #[inline]
    fn from_json(json: impl AsRef<[u8]>) -> Self
    where
        Self: DeserializeOwned,
    {
        Self::try_from_json(json).unwrap()
    }
    #[inline]
    fn from_json_from(reader: impl io::Read) -> Self
    where
        Self: DeserializeOwned,
    {
        Self::try_from_json_from(reader).unwrap()
    }
    // --------------
    #[inline]
    fn try_to_json_pretty(&self) -> Result<String>
    where
        Self: Serialize,
    {
        serde_json::to_string_pretty(self).map_err(Error::Json)
    }
    #[inline]
    fn try_to_json_pretty_into(&self, writer: impl io::Write) -> Result<()>
    where
        Self: Serialize,
    {
        serde_json::to_writer_pretty(writer, self).map_err(Error::Json)
    }
    // --------------
    #[inline]
    fn to_json_pretty(&self) -> String
    where
        Self: Serialize,
    {
        self.try_to_json_pretty().unwrap()
    }
    #[inline]
    fn to_json_pretty_into(&self, writer: impl io::Write)
    where
        Self: Serialize,
    {
        self.try_to_json_pretty_into(writer).unwrap()
    }
    // --------------
}

#[cfg(feature = "bytes")]
/// # A trait that can convert to a slice of bytes.
pub trait AsBytes {
    fn as_byte_slice(&self) -> &[u8];
    #[inline]
    fn to_bytes(&self) -> Vec<u8> {
        self.as_byte_slice().to_vec()
    }
    #[inline]
    fn into_bytes(self) -> Vec<u8>
    where
        Self: Sized,
    {
        self.to_bytes()
    }
    #[inline]
    fn try_to_bytes_into(&self, mut writer: impl io::Write) -> Result<()> {
        writer.write_all(self.as_byte_slice()).map_err(Error::Io)
    }
    #[inline]
    fn to_bytes_into(&self, writer: impl io::Write) {
        self.try_to_bytes_into(writer).unwrap()
    }
}

#[cfg(feature = "bytes")]
/// implement `AsBytes` for `impl AsRef<[u8]>`
impl<T: AsRef<[u8]>> AsBytes for T {
    #[inline]
    fn as_byte_slice(&self) -> &[u8] {
        self.as_ref()
    }
}

#[cfg(feature = "hex")]
/// # A trait that can convert bytes to hex string. (encode/decode)
pub trait ToHex: AsBytes {
    #[inline]
    fn try_to_hex_into_with_mode(&self, mut writer: impl fmt::Write, mode: HexMode) -> Result<()> {
        if mode.has_0x() {
            write!(writer, "0x")?;
        }
        let is_lower = mode.is_lower();
        for byte in self.as_byte_slice() {
            if is_lower {
                write!(writer, "{:02x}", byte)?;
            } else {
                write!(writer, "{:02X}", byte)?;
            }
        }
        Ok(())
    }
    #[inline]
    fn try_to_hex_with_mode(&self, mode: HexMode) -> Result<String> {
        let mut hex = String::with_capacity(
            self.as_byte_slice().len() * 2 + if mode.has_0x() { 2 } else { 0 }, //
        );
        self.try_to_hex_into_with_mode(&mut hex, mode)?;
        Ok(hex)
    }
    #[inline]
    fn try_from_hex(hex: impl AsRef<[u8]>) -> Result<Vec<u8>> {
        let hex = hex.as_ref();
        let hex = if hex.starts_with(&[b'0', b'x']) {
            &hex[2..]
        } else {
            hex
        };
        if hex.len() % 2 != 0 {
            return Err(Error::OddLength);
        }
        let mut bytes = Vec::with_capacity(hex.len() / 2);
        for i in (0..hex.len()).step_by(2) {
            let s = std::str::from_utf8(&hex[i..i + 2])?;
            let byte = u8::from_str_radix(s, 16)?;
            bytes.push(byte);
        }
        Ok(bytes)
    }
    #[inline]
    fn try_from_hex_from(mut reader: impl io::Read) -> Result<Vec<u8>> {
        let mut double = [0u8; 2];
        reader.read_exact(&mut double)?;
        let mut v = Vec::new();
        let mut take_into_v = |double: &mut [u8; 2]| -> Result<()> {
            let ch = std::str::from_utf8(double)?;
            let byte = u8::from_str_radix(ch, 16)?;
            v.push(byte);
            double[0] = 0;
            double[1] = 0;
            Ok(())
        };
        if double != [b'0', b'x'] {
            take_into_v(&mut double)?;
        }
        loop {
            if let Err(e) = reader.read_exact(&mut double) {
                match e.kind() {
                    io::ErrorKind::UnexpectedEof => {
                        if double[0] == 0 {
                            break;
                        }
                        return Err(Error::OddLength);
                    }
                    _ => return Err(Error::Io(e)),
                }
            }
            take_into_v(&mut double)?;
        }
        Ok(v)
    }
    #[inline]
    fn try_copy_from_hex(&mut self, hex: impl AsRef<[u8]>) -> Result<usize>
    where
        Self: AsMut<[u8]>,
    {
        let hex = hex.as_ref();
        let hex = if hex.starts_with(&[b'0', b'x']) {
            &hex[2..]
        } else {
            hex
        };
        if hex.len() % 2 != 0 {
            return Err(Error::OddLength);
        }
        let hex_bytes_len = hex.len() / 2;
        let bytes = self.as_mut();
        if hex_bytes_len > bytes.len() {
            return Err(Error::OutOfBounds(bytes.len(), hex_bytes_len));
        }
        for i in (0..hex.len()).step_by(2) {
            let s = std::str::from_utf8(&hex[i..i + 2])?;
            let byte = u8::from_str_radix(s, 16)?;
            bytes[i / 2] = byte;
        }
        Ok(hex_bytes_len)
    }

    #[inline]
    fn try_to_hex(&self) -> Result<String> {
        self.try_to_hex_with_mode(HexMode::Lower)
    }
    #[inline]
    fn try_to_upper_hex(&self) -> Result<String> {
        self.try_to_hex_with_mode(HexMode::Upper)
    }
    #[inline]
    fn try_to_hex_with_0x(&self) -> Result<String> {
        self.try_to_hex_with_mode(HexMode::Lower0x)
    }
    #[inline]
    fn try_to_upper_hex_with_0x(&self) -> Result<String> {
        self.try_to_hex_with_mode(HexMode::Upper0x)
    }
    #[inline]
    fn to_hex(&self) -> String {
        self.try_to_hex().unwrap()
    }
    #[inline]
    fn to_upper_hex(&self) -> String {
        self.try_to_upper_hex().unwrap()
    }
    #[inline]
    fn to_hex_with_0x(&self) -> String {
        self.try_to_hex_with_0x().unwrap()
    }
    #[inline]
    fn to_upper_hex_with_0x(&self) -> String {
        self.try_to_upper_hex_with_0x().unwrap()
    }

    #[inline]
    fn try_to_hex_into(&self, writer: impl fmt::Write) -> Result<()> {
        self.try_to_hex_into_with_mode(writer, HexMode::Lower)
    }
    #[inline]
    fn try_to_upper_hex_into(&self, writer: impl fmt::Write) -> Result<()> {
        self.try_to_hex_into_with_mode(writer, HexMode::Upper)
    }
    #[inline]
    fn try_to_hex_into_with_0x(&self, writer: impl fmt::Write) -> Result<()> {
        self.try_to_hex_into_with_mode(writer, HexMode::Lower0x)
    }
    #[inline]
    fn try_to_upper_hex_into_with_0x(&self, writer: impl fmt::Write) -> Result<()> {
        self.try_to_hex_into_with_mode(writer, HexMode::Upper0x)
    }
    #[inline]
    fn to_hex_into(&self, writer: impl fmt::Write) {
        self.try_to_hex_into(writer).unwrap()
    }
    #[inline]
    fn to_upper_hex_into(&self, writer: impl fmt::Write) {
        self.try_to_upper_hex_into(writer).unwrap()
    }
    #[inline]
    fn to_hex_into_with_0x(&self, writer: impl fmt::Write) {
        self.try_to_hex_into_with_0x(writer).unwrap()
    }
    #[inline]
    fn to_upper_hex_into_with_0x(&self, writer: impl fmt::Write) {
        self.try_to_upper_hex_into_with_0x(writer).unwrap()
    }

    #[inline]
    fn from_hex(hex: impl AsRef<[u8]>) -> Vec<u8> {
        Self::try_from_hex(hex).unwrap()
    }
    #[inline]
    fn from_hex_from(reader: impl io::Read) -> Vec<u8> {
        Self::try_from_hex_from(reader).unwrap()
    }
    #[inline]
    fn copy_from_hex(&mut self, hex: impl AsRef<[u8]>) -> usize
    where
        Self: AsMut<[u8]>,
    {
        self.try_copy_from_hex(hex).unwrap()
    }
}

#[cfg(feature = "hex")]
/// implement `ToHex` for `impl AsBytes`
impl<T: AsBytes> ToHex for T {}

#[cfg(feature = "bytes")]
/// # A trait for converting a byte slice to a string.
pub trait ToStringForBytes: AsBytes {
    #[inline]
    fn try_as_str(&self) -> Result<&str> {
        std::str::from_utf8(self.as_byte_slice()).map_err(Error::Utf8)
    }
    #[inline]
    fn try_to_string(&self) -> Result<String> {
        self.try_as_str().map(|s| s.to_string())
    }
    /// # Safety
    /// This function is unsafe because it does not check if the bytes are valid UTF-8.
    #[inline]
    fn as_str(&self) -> &str {
        self.try_as_str().unwrap()
    }
    /// # Safety
    /// This function is unsafe because it does not check if the bytes are valid UTF-8.
    #[inline]
    fn to_string(&self) -> String {
        self.try_to_string().unwrap()
    }
    #[inline]
    fn to_string_lossy(&self) -> String {
        String::from_utf8_lossy(self.as_byte_slice()).to_string()
    }
    #[inline]
    fn try_into_string(self) -> Result<String>
    where
        Self: Sized,
    {
        self.try_to_string()
    }
    /// # Safety
    /// This function is unsafe because it does not check if the bytes are valid UTF-8.
    #[inline]
    fn into_string(self) -> String
    where
        Self: Sized,
    {
        self.to_string()
    }
}

#[cfg(feature = "bytes")]
/// implement `AsBytes` for `impl AsBytes`
impl<T: AsBytes> ToStringForBytes for T {}

#[cfg(test)]
mod tests {
    use super::*;
    use serde::Deserialize;

    use sha2::Sha256;
    #[derive(Serialize, Deserialize, PartialEq, Eq, Debug)]
    struct Test {
        a: u32,
        b: String,
        c: [u8; 32],
        d: Vec<u8>,
    }
    impl ToBytes for Test {}
    impl ToHash for Test {}
    impl ToJson for Test {}

    #[test]
    fn test_to_bytes() {
        let test = Test {
            a: 1,
            b: "hello".to_owned(),
            c: [0; 32],
            d: vec![1, 2, 3],
        };
        let bytes = test.to_bytes();
        let test2 = Test::from_bytes(bytes);
        if test != test2 {
            panic!("test != test2");
        }
    }

    #[test]
    fn test_from_bytes() {
        let test = Test {
            a: 1,
            b: "hello".to_owned(),
            c: [0; 32],
            d: vec![1, 2, 3],
        };
        let bytes = test.to_bytes();
        let test2 = Test::from_bytes(bytes);
        if test != test2 {
            panic!("test != test2");
        }
    }

    #[test]
    fn test_from_bytes_from() {
        let test = Test {
            a: 1,
            b: "hello".to_owned(),
            c: [0; 32],
            d: vec![1, 2, 3],
        };
        let bytes = test.to_bytes();
        let mut reader = io::Cursor::new(bytes);
        let test2 = Test::from_bytes_from(&mut reader);
        if test != test2 {
            panic!("test != test2");
        }
    }

    #[test]
    fn test_endians() {
        let test = Test {
            a: 1,
            b: "hello".to_owned(),
            c: [0; 32],
            d: vec![1, 2, 3],
        };
        let be_bytes = test.to_be_bytes();
        let le_bytes = test.to_le_bytes();
        assert_ne!(be_bytes, le_bytes);
    }

    #[test]
    fn test_to_bytes_for_ref() {
        let s = "Hello world".to_string();
        let bytes = s.to_bytes();
        let bytes2 = s.as_bytes().to_vec();
        let bytes3 = s.into_bytes();
        assert_eq!(bytes, bytes2);
        assert_eq!(bytes, bytes3);

        let arr = [1u8, 2, 3, 4, 5];
        let bytes = arr.to_bytes();
        let bytes2 = arr.to_vec();
        assert_eq!(bytes, bytes2);
    }

    #[test]
    fn test_to_string_for_ref() {
        let s = "Hello world".to_string();
        let _s2 = s.as_bytes();
        let s2 = _s2.as_str();
        assert_eq!(s, s2);

        let arr = [72, 105, 32, 77, 111, 109];
        let s = arr.as_str();
        assert_eq!(s, "Hi Mom");

        let bytes = s.to_bytes();
        let s2 = bytes.to_string();
        assert_eq!(s, s2);

        let a = arr.to_string();
        let b = arr.into_string().into_bytes().to_string();
        assert_eq!(a, b);

        let arr = [72, 105, 77, 111, 109];
        let s1 = arr.into_string();
        let bytes = s1.to_bytes();
        let s2 = bytes.as_str();
        assert_eq!(s1, s2);
    }

    #[test]
    fn test_to_hash() {
        let test = Test {
            a: 1,
            b: "hello".to_owned(),
            c: [0; 32],
            d: vec![1, 2, 3],
        };
        let hash: [u8; 32] = test.to_hash::<Sha256>().into();
        let want = [
            47, 4, 143, 18, 245, 11, 240, 126, 40, 67, 246, 163, 209, 169, 27, 33, 175, 175, 122,
            26, 143, 64, 16, 251, 138, 178, 167, 255, 87, 173, 70, 37,
        ];
        assert_eq!(hash, want);
    }

    #[test]
    fn test_to_hash_into() {
        let test = Test {
            a: 1,
            b: "hello".to_owned(),
            c: [0; 32],
            d: vec![1, 2, 3],
        };
        let mut hasher = Sha256::new();
        test.to_hash_into(&mut hasher);
        let hash: [u8; 32] = hasher.finalize().into();
        let want = [
            47, 4, 143, 18, 245, 11, 240, 126, 40, 67, 246, 163, 209, 169, 27, 33, 175, 175, 122,
            26, 143, 64, 16, 251, 138, 178, 167, 255, 87, 173, 70, 37,
        ];
        assert_eq!(hash, want);
    }

    #[test]
    fn test_to_json() {
        let test = Test {
            a: 1,
            b: "hello".to_owned(),
            c: [0; 32],
            d: vec![1, 2, 3],
        };
        let json = test.to_json();
        let test2 = Test::from_json(json);
        if test != test2 {
            panic!("test != test2");
        }
    }

    #[test]
    fn test_to_json_pretty() {
        let test = Test {
            a: 1,
            b: "hello".to_owned(),
            c: [0; 32],
            d: vec![1, 2, 3],
        };
        let json = test.to_json_pretty();
        let test2 = Test::from_json(json);
        if test != test2 {
            panic!("test != test2");
        }
    }

    #[test]
    fn test_to_hex() {
        let bytes = b"hello world";
        assert_eq!(bytes.to_hex(), "68656c6c6f20776f726c64");
        assert_eq!(bytes.to_upper_hex(), "68656C6C6F20776F726C64");
        assert_eq!(bytes.to_hex_with_0x(), "0x68656c6c6f20776f726c64");
        assert_eq!(bytes.to_upper_hex_with_0x(), "0x68656C6C6F20776F726C64");
    }

    #[test]
    fn test_to_hex_into() {
        let bytes = b"hello world";
        let mut hex = String::new();

        bytes.to_hex_into(&mut hex);
        assert_eq!(hex, "68656c6c6f20776f726c64");
        hex.clear();

        bytes.to_upper_hex_into(&mut hex);
        assert_eq!(hex, "68656C6C6F20776F726C64");
        hex.clear();

        bytes.to_hex_into_with_0x(&mut hex);
        assert_eq!(hex, "0x68656c6c6f20776f726c64");
        hex.clear();

        bytes.to_upper_hex_into_with_0x(&mut hex);
        assert_eq!(hex, "0x68656C6C6F20776F726C64");
    }

    #[test]
    fn test_from_hex() {
        let bytes = b"hello world";
        assert_eq!(Vec::<u8>::from_hex("68656c6c6f20776f726c64"), bytes);
        assert_eq!(Vec::<u8>::from_hex("68656C6C6F20776F726C64"), bytes);
        assert_eq!(Vec::<u8>::from_hex("0x68656c6c6f20776f726c64"), bytes);
        assert_eq!(Vec::<u8>::from_hex("0x68656C6C6F20776F726C64"), bytes);
    }

    #[test]
    fn test_from_hex_from() {
        let bytes = b"hello world";
        let mut reader = io::Cursor::new("68656c6c6f20776f726c64");
        assert_eq!(Vec::<u8>::from_hex_from(&mut reader), bytes);
        reader.set_position(0);
        assert_eq!(Vec::<u8>::from_hex_from(&mut reader), bytes);
        reader.set_position(0);
        assert_eq!(Vec::<u8>::from_hex_from(&mut reader), bytes);
        reader.set_position(0);
        assert_eq!(Vec::<u8>::from_hex_from(&mut reader), bytes);
    }

    #[test]
    fn test_try_from_hex() {
        let bytes = b"hello world";
        let hex = bytes.to_hex();
        let bytes2 = Vec::<u8>::try_from_hex(hex).unwrap();
        assert_eq!(bytes, &bytes2[..]);
    }

    #[test]
    fn test_try_from_hex_from() {
        let bytes = b"hello world";
        let hex = bytes.to_hex();
        let bytes2 = Vec::<u8>::try_from_hex_from(hex.as_bytes()).unwrap();
        assert_eq!(bytes, &bytes2[..]);
    }

    #[test]
    fn test_try_copy_from_hex() {
        let bytes = b"helloz world";
        let mut a = [0u8; 12];
        a.try_copy_from_hex(bytes.to_hex()).unwrap();
        assert_eq!(bytes, &a[..]);
    }
}
