use crate::{
    enums::{Endian, HexMode},
    error::*,
};
use std::{fmt, io};

use bincode::Options;
use digest::{Digest, Output};
use serde::{de::DeserializeOwned, Deserialize, Serialize};

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

pub struct ToBytesOptions {
    pub endian: Endian,
}

/// # A trait that can de/serialize something with bytes. (default: little endian)
pub trait ToBytes: Serialize {
    const OPTIONS: ToBytesOptions = ToBytesOptions {
        endian: Endian::Little,
    };

    /// Serialize to bytes.
    #[inline]
    fn try_to_be_bytes(&self) -> Result<Vec<u8>> {
        serialize!(data: self, option: bincode::options().with_big_endian())
    }
    #[inline]
    fn try_to_le_bytes(&self) -> Result<Vec<u8>> {
        serialize!(data: self, option: bincode::options().with_little_endian())
    }
    #[inline]
    fn try_to_be_bytes_into(&self, writer: impl io::Write) -> Result<()> {
        serialize!(data: self, writer: writer, option: bincode::options().with_big_endian())
    }
    #[inline]
    fn try_to_le_bytes_into(&self, writer: impl io::Write) -> Result<()> {
        serialize!(data: self, writer: writer, option: bincode::options().with_little_endian())
    }
    #[inline]
    fn to_be_bytes(&self) -> Vec<u8> {
        serialize!(data: self, option: bincode::options().with_big_endian()).unwrap()
    }
    #[inline]
    fn to_le_bytes(&self) -> Vec<u8> {
        serialize!(data: self, option: bincode::options().with_little_endian()).unwrap()
    }
    #[inline]
    fn to_be_bytes_into(&self, writer: impl io::Write) {
        serialize!(data: self, writer: writer, option: bincode::options().with_big_endian())
            .unwrap()
    }
    #[inline]
    fn to_le_bytes_into(&self, writer: impl io::Write) {
        serialize!(data: self, writer: writer, option: bincode::options().with_little_endian())
            .unwrap()
    }

    /// Deserialize from bytes.
    #[inline]
    fn try_from_be_bytes<'a>(bytes: &'a [u8]) -> Result<Self>
    where
        Self: Deserialize<'a>,
    {
        deserialize!(data: bytes, option: bincode::options().with_big_endian())
    }
    #[inline]
    fn try_from_le_bytes<'a>(bytes: &'a [u8]) -> Result<Self>
    where
        Self: Deserialize<'a>,
    {
        deserialize!(data: bytes, option: bincode::options().with_little_endian())
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
    fn from_be_bytes<'a>(bytes: &'a [u8]) -> Self
    where
        Self: Deserialize<'a>,
    {
        Self::try_from_be_bytes(bytes).unwrap()
    }
    #[inline]
    fn from_le_bytes<'a>(bytes: &'a [u8]) -> Self
    where
        Self: Deserialize<'a>,
    {
        Self::try_from_le_bytes(bytes).unwrap()
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

    // ------------- default endians -------------
    #[inline]
    fn try_to_bytes(&self) -> Result<Vec<u8>> {
        match Self::OPTIONS.endian {
            Endian::Big => self.try_to_be_bytes(),
            Endian::Little => self.try_to_le_bytes(),
            Endian::Native => self.try_to_le_bytes(),
        }
    }
    #[inline]
    fn try_to_bytes_into(&self, writer: impl io::Write) -> Result<()> {
        match Self::OPTIONS.endian {
            Endian::Big => self.try_to_be_bytes_into(writer),
            Endian::Little => self.try_to_le_bytes_into(writer),
            Endian::Native => self.try_to_le_bytes_into(writer),
        }
    }
    #[inline]
    fn try_from_bytes<'a>(bytes: &'a [u8]) -> Result<Self>
    where
        Self: Deserialize<'a>,
    {
        match Self::OPTIONS.endian {
            Endian::Big => Self::try_from_be_bytes(bytes),
            Endian::Little => Self::try_from_le_bytes(bytes),
            Endian::Native => Self::try_from_le_bytes(bytes),
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
            Endian::Native => Self::try_from_le_bytes_from(reader),
        }
    }
    // --------------------------------------------------
    #[inline]
    fn to_bytes(&self) -> Vec<u8> {
        self.try_to_bytes().unwrap()
    }
    #[inline]
    fn to_bytes_into(&self, writer: impl io::Write) {
        self.try_to_bytes_into(writer).unwrap()
    }
    #[inline]
    fn from_bytes<'a>(bytes: &'a [u8]) -> Self
    where
        Self: Deserialize<'a>,
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

/// # A trait that can hash bytes.
pub trait ToHash: ToBytes {
    #[inline]
    fn try_to_hash<T: Digest + io::Write>(&self) -> Result<Output<T>> {
        let mut hasher = T::new();
        self.try_to_hash_into(&mut hasher)?;
        Ok(hasher.finalize())
    }
    #[inline]
    fn try_to_hash_into<T: Digest + io::Write>(&self, hasher: &mut T) -> Result<()> {
        self.try_to_bytes_into(hasher)
    }
    #[inline]
    fn to_hash<T: Digest + io::Write>(&self) -> Output<T> {
        let mut hasher = T::new();
        self.to_hash_into(&mut hasher);
        hasher.finalize()
    }
    #[inline]
    fn to_hash_into<T: Digest + io::Write>(&self, hasher: &mut T) {
        self.to_bytes_into(hasher)
    }
}

/// # A trait that can convert bytes to hex string. (encode/decode)
pub trait ToHex: AsRef<[u8]> {
    #[inline]
    fn try_to_hex_into_with_mode(&self, mut writer: impl fmt::Write, mode: HexMode) -> Result<()> {
        if mode.has_0x() {
            write!(writer, "0x")?;
        }
        let is_lower = mode.is_lower();
        for byte in self.as_ref() {
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
            self.as_ref().len() * 2 + if mode.has_0x() { 2 } else { 0 }, //
        );
        self.try_to_hex_into_with_mode(&mut hex, mode)?;
        Ok(hex)
    }
    #[inline]
    fn try_from_hex(hex: impl AsRef<str>) -> Result<Vec<u8>> {
        let hex = hex.as_ref();
        let hex = hex.strip_prefix("0x").unwrap_or(hex);
        if hex.len() % 2 != 0 {
            return Err(Error::OddLength);
        }
        let mut bytes = Vec::with_capacity(hex.len() / 2);
        for i in (0..hex.len()).step_by(2) {
            let byte = u8::from_str_radix(&hex[i..i + 2], 16)?;
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
            let ch = std::str::from_utf8(double).map_err(Error::Utf8)?;
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
    fn try_copy_from_hex(&mut self, hex: impl AsRef<str>) -> Result<usize>
    where
        Self: AsMut<[u8]>,
    {
        let hex = hex.as_ref();
        let hex = hex.strip_prefix("0x").unwrap_or(hex);
        if hex.len() % 2 != 0 {
            return Err(Error::OddLength);
        }
        let hex_bytes_len = hex.len() / 2;
        let bytes = self.as_mut();
        if hex_bytes_len > bytes.len() {
            return Err(Error::OutOfBounds(bytes.len(), hex_bytes_len));
        }
        for i in (0..hex.len()).step_by(2) {
            let byte = u8::from_str_radix(&hex[i..i + 2], 16)?;
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
    fn from_hex(hex: impl AsRef<str>) -> Vec<u8> {
        Self::try_from_hex(hex).unwrap()
    }
    #[inline]
    fn from_hex_from(reader: impl io::Read) -> Vec<u8> {
        Self::try_from_hex_from(reader).unwrap()
    }
    #[inline]
    fn copy_from_hex(&mut self, hex: impl AsRef<str>) -> usize
    where
        Self: AsMut<[u8]>,
    {
        self.try_copy_from_hex(hex).unwrap()
    }
}

impl<T: AsRef<[u8]>> ToHex for T {}

#[cfg(test)]
mod tests {
    use super::*;

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

    #[test]
    fn test_to_bytes() {
        let test = Test {
            a: 1,
            b: "hello".to_owned(),
            c: [0; 32],
            d: vec![1, 2, 3],
        };
        let bytes = test.to_bytes();
        let test2 = Test::from_bytes(&bytes);
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
        let test2 = Test::from_bytes(&bytes);
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
        let bytes2 = Vec::<u8>::try_from_hex(&hex).unwrap();
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
