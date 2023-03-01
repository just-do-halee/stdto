#![allow(clippy::needless_update)]

use std::{collections::BTreeMap, error::Error};

use stdto::prelude::*;

#[stdto::borsh_bytes]
struct Test {
    a: u32,
    b: String,
    c: [u8; 32],
    d: Vec<u8>,
    e: BTreeMap<u8, f64>,
}

fn main() -> Result<(), Box<dyn Error>> {
    let origin = Test {
        a: 1,
        b: "test".to_string(),
        c: [5; 32],
        d: vec![3; 32],
        e: BTreeMap::new(),
    };

    let mut bytes: Vec<u8>;

    bytes = origin.to_borsh_bytes();
    Test::from_borsh_bytes(&bytes);

    bytes.clear();

    bytes = origin.try_to_borsh_bytes()?;
    Test::try_from_borsh_bytes(&bytes)?;

    bytes.clear();

    origin.to_borsh_bytes_into(&mut bytes);
    Test::from_borsh_bytes_from(bytes.as_slice());

    bytes.clear();

    origin.try_to_borsh_bytes_into(&mut bytes)?;
    Test::try_from_borsh_bytes_from(bytes.as_slice())?;

    Test::from_borsh_bytes(bytes);

    Ok(())
}
