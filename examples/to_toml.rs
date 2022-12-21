use std::{collections::HashMap, error::Error};

use stdto::prelude::*;

#[stdto::toml]
#[derive(Debug, PartialEq, Eq)]
struct Test {
    a: u32,
    b: String,
    c: [u8; 32],
    d: Vec<u8>,
    e: HashMap<String, u64>,
}

fn main() -> Result<(), Box<dyn Error>> {
    let test = Test {
        a: 1,
        b: "hello".to_owned(),
        c: [5; 32],
        d: vec![3; 32],
        e: HashMap::from([("Hello".to_string(), 10), ("World".to_string(), 20)]),
    };
    let toml = test.to_toml();
    let test2 = Test::from_toml(toml);
    assert_eq!(test, test2);

    let test = Test {
        a: 100,
        b: "world".to_owned(),
        c: [14; 32],
        d: vec![12; 25],
        e: HashMap::from([("Hello".to_string(), 10), ("World".to_string(), 20)]),
    };
    let toml = test.to_toml_pretty();
    let test2 = Test::from_toml(toml);
    assert_eq!(test, test2);

    test_all();
    Ok(())
}

#[stdto::bytes]
#[stdto::hash]
#[stdto::toml]
#[derive(Debug, PartialEq, Eq)]
struct All {
    a: u32,
    b: String,
    c: [u8; 32],
    d: Vec<u8>,
    e: HashMap<String, u64>,
}

/// Test to_bytes, from_bytes, to_hash, to_json, from_json, to_hex, from_hex
fn test_all() {
    let all = All {
        a: 1,
        b: "hello".to_owned(),
        c: [5; 32],
        d: vec![3; 32],
        e: HashMap::from([("Test".to_string(), 10), ("Man".to_string(), 20)]),
    };
    let bytes = all.to_bytes();
    let all2 = All::from_bytes(bytes);
    assert_eq!(all, all2);

    let hash: [u8; 32] = all.to_hash::<sha2::Sha256>().into();
    assert_eq!(
        hash.to_hex(),
        "a4d8ad5f4c574bfdd3a4027290cb01c3f9fc11740296b5497512f009e40c10eb"
    );
    assert_eq!(
        hash.to_upper_hex(),
        "A4D8AD5F4C574BFDD3A4027290CB01C3F9FC11740296B5497512F009E40C10EB"
    );
    assert_eq!(
        hash.to_hex_with_0x(),
        "0xa4d8ad5f4c574bfdd3a4027290cb01c3f9fc11740296b5497512f009e40c10eb"
    );

    let toml = all.to_toml();
    let all2 = All::from_toml(toml);
    assert_eq!(all, all2);

    let toml = all.to_toml_pretty();
    let all2 = All::from_toml(toml);
    assert_eq!(all, all2);
}
