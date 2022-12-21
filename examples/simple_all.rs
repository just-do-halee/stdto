#![allow(unreachable_code, unused_mut)]

use std::{collections::HashMap, error::Error};

use stdto::prelude::*;

#[stdto::bytes]
#[stdto::hash]
#[stdto::json]
#[stdto::yaml]
#[stdto::toml]
#[derive(DebugHex, PartialEq, Eq)]
struct All {
    a: u32,
    b: String,
    c: [u8; 32],
    d: Vec<u8>,
    e: HashMap<String, u64>,
}

fn main() -> Result<(), Box<dyn Error>> {
    let all = All {
        a: 1,
        b: "hello".to_owned(),
        c: [5; 32],
        d: vec![3; 32],
        e: HashMap::from([("Test".to_string(), 10), ("Man".to_string(), 20)]),
    };
    let mut all2: All;

    let bytes = all.to_bytes();
    all2 = All::from_bytes(bytes);
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

    let debuged = format!("{:?}", all);
    assert_eq!(debuged, "#\"0x01000000050000000000000068656C6C6F050505050505050505050505050505050505050505050505050505050505050520000000000000000303030303030303030303030303030303030303030303030303030303030303020000000000000003000000000000004D616E14000000000000000400000000000000546573740A00000000000000\"#");
    let json = all.to_json();
    all2 = All::from_json(json);
    assert_eq!(all, all2);

    let yaml = all.to_yaml();
    all2 = All::from_yaml(yaml);
    assert_eq!(all, all2);

    let toml = all.to_toml();
    all2 = All::from_toml(toml);
    assert_eq!(all, all2);
    Ok(())
}
