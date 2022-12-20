#![allow(clippy::needless_update)]

use std::{collections::HashMap, error::Error, io::Cursor};

use stdto::prelude::*;

#[stdto::bytes(endian = "big")]
struct Test {
    a: u32,
    b: String,
    c: [u8; 32],
    d: Vec<u8>,
    e: HashMap<u8, f64>,
}

fn main() -> Result<(), Box<dyn Error>> {
    let origin = Test {
        a: 1,
        b: "test".to_string(),
        c: [5; 32],
        d: vec![3; 32],
        e: HashMap::new(),
    };

    let mut bytes: Vec<u8>;

    bytes = origin.to_bytes();
    Test::from_bytes(&bytes);

    bytes.clear();

    bytes = origin.try_to_bytes()?;
    Test::try_from_bytes(&bytes)?;

    bytes.clear();

    origin.to_bytes_into(&mut bytes);
    Test::from_bytes_from(bytes.as_slice());

    bytes.clear();

    origin.try_to_bytes_into(&mut bytes)?;
    Test::try_from_bytes_from(bytes.as_slice())?;

    Test::from_bytes(bytes);

    let be_bytes = origin.to_be_bytes();
    let le_bytes = origin.to_le_bytes();
    assert_ne!(be_bytes, le_bytes);

    test_custom()?;

    let s = "Hello world".to_string();
    let as_bytes = s.as_byte_slice();
    assert_eq!(as_bytes, s.as_bytes());

    let bytes = s.to_bytes();
    let bytes2 = s.as_bytes().to_vec();
    let bytes3 = s.into_bytes();
    assert_eq!(bytes, bytes2);
    assert_eq!(bytes, bytes3);

    let arr = [1u8, 2, 3, 4, 5];
    let bytes = arr.to_bytes();
    let bytes2 = arr.to_vec();
    assert_eq!(bytes, bytes2);

    Ok(())
}

#[stdto::serde]
struct Custom {
    a: u32,
    b: String,
    c: [u8; 32],
    d: Vec<u8>,
    e: HashMap<u8, f64>,
}

use stdto::{Endian, ToBytesOptions};

// custom options
impl ToBytes for Custom {
    const OPTIONS: ToBytesOptions = ToBytesOptions {
        endian: Endian::Big,
        ..ToBytesOptions::default()
    };
}

fn test_custom() -> Result<(), Box<dyn Error>> {
    let origin = Custom {
        a: 1,
        b: "test".to_string(),
        c: [5; 32],
        d: vec![3; 32],
        e: HashMap::new(),
    };

    let mut bytes: Vec<u8>;

    bytes = origin.to_bytes();
    Custom::from_bytes(&bytes);

    bytes.clear();

    bytes = origin.try_to_bytes()?;
    Custom::try_from_bytes(&bytes)?;

    bytes.clear();

    origin.to_bytes_into(&mut bytes);
    Custom::from_bytes_from(&mut Cursor::new(&bytes));

    bytes.clear();

    origin.try_to_bytes_into(&mut bytes)?;
    Custom::try_from_bytes_from(&mut Cursor::new(&bytes))?;

    let be_bytes = origin.to_be_bytes();
    let le_bytes = origin.to_le_bytes();
    assert_ne!(be_bytes, le_bytes);

    Ok(())
}
