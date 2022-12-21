use std::{collections::BTreeMap, error::Error};

use stdto::prelude::*;

#[stdto::toml]
#[derive(Debug, PartialEq, Eq)]
struct Test {
    a: u32,
    b: String,
    c: [u8; 32],
    d: Vec<u8>,
    e: BTreeMap<String, u64>,
}

fn main() -> Result<(), Box<dyn Error>> {
    let test = Test {
        a: 1,
        b: "hello".to_owned(),
        c: [5; 32],
        d: vec![3; 32],
        e: BTreeMap::from([("Hello".to_string(), 10), ("World".to_string(), 20)]),
    };
    let toml = test.to_toml();
    let test2 = Test::from_toml(toml);
    assert_eq!(test, test2);

    let test = Test {
        a: 100,
        b: "world".to_owned(),
        c: [14; 32],
        d: vec![12; 25],
        e: BTreeMap::from([("Hello".to_string(), 10), ("World".to_string(), 20)]),
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
    e: BTreeMap<String, u64>,
}

/// Test to_bytes, from_bytes, to_hash, to_json, from_json, to_hex, from_hex
fn test_all() {
    let all = All {
        a: 1,
        b: "hello".to_owned(),
        c: [5; 32],
        d: vec![3; 32],
        e: BTreeMap::from([("Test".to_string(), 10), ("Man".to_string(), 20)]),
    };
    let bytes = all.to_bytes();
    let all2 = All::from_bytes(bytes);
    assert_eq!(all, all2);

    let hash: [u8; 32] = all.to_hash::<sha2::Sha256>().into();
    assert_eq!(
        hash.to_hex(),
        "23bf6d08f3c5a50748578af6e42d993d0068038656e02cb7eaa5794cc0c90a4f"
    );
    assert_eq!(
        hash.to_upper_hex(),
        "23BF6D08F3C5A50748578AF6E42D993D0068038656E02CB7EAA5794CC0C90A4F"
    );
    assert_eq!(
        hash.to_hex_with_0x(),
        "0x23bf6d08f3c5a50748578af6e42d993d0068038656e02cb7eaa5794cc0c90a4f"
    );

    let toml = all.to_toml();
    let all2 = All::from_toml(toml);
    assert_eq!(all, all2);

    let toml = all.to_toml_pretty();
    let all2 = All::from_toml(toml);
    assert_eq!(all, all2);
}
