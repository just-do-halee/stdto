use std::{collections::HashMap, error::Error};

use stdto::prelude::*;

#[stdto::yaml]
#[derive(Debug, PartialEq, Eq)]
struct Test {
    a: u32,
    b: String,
    c: [u8; 32],
    d: Vec<u8>,
    e: HashMap<u8, u64>,
}

fn main() -> Result<(), Box<dyn Error>> {
    let test = Test {
        a: 1,
        b: "hello".to_owned(),
        c: [5; 32],
        d: vec![3; 32],
        e: HashMap::from([(1, 10), (2, 20)]),
    };
    let yaml = test.to_yaml();
    let test2 = Test::from_yaml(yaml);
    assert_eq!(test, test2);

    test_all();
    Ok(())
}

#[stdto::bytes]
#[stdto::hash]
#[stdto::yaml]
#[derive(Debug, PartialEq, Eq)]
struct All {
    a: u32,
    b: String,
    c: [u8; 32],
    d: Vec<u8>,
    e: HashMap<u8, u64>,
}

/// Test to_bytes, from_bytes, to_hash, to_yaml, from_yaml, to_hex, from_hex
fn test_all() {
    let all = All {
        a: 1,
        b: "hello".to_owned(),
        c: [5; 32],
        d: vec![3; 32],
        e: HashMap::from([(1, 10), (2, 20)]),
    };
    let bytes = all.to_bytes();
    let all2 = All::from_bytes(bytes);
    assert_eq!(all, all2);

    let hash: [u8; 32] = all.to_hash::<sha2::Sha256>().into();
    assert_eq!(
        hash.to_hex(),
        "01050a405bda699cbaaf708faac6cc5111f2105332e9e487538c7e2cb8e160eb"
    );
    assert_eq!(
        hash.to_upper_hex(),
        "01050A405BDA699CBAAF708FAAC6CC5111F2105332E9E487538C7E2CB8E160EB"
    );
    assert_eq!(
        hash.to_hex_with_0x(),
        "0x01050a405bda699cbaaf708faac6cc5111f2105332e9e487538c7e2cb8e160eb"
    );

    let yaml = all.to_yaml();
    let all2 = All::from_yaml(yaml);
    assert_eq!(all, all2);
}
