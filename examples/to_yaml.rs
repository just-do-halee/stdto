use std::{collections::BTreeMap, error::Error};

use stdto::prelude::*;

#[stdto::yaml]
#[derive(Debug, PartialEq, Eq)]
struct Test {
    a: u32,
    b: String,
    c: [u8; 32],
    d: Vec<u8>,
    e: BTreeMap<u8, u64>,
}

fn main() -> Result<(), Box<dyn Error>> {
    let test = Test {
        a: 1,
        b: "hello".to_owned(),
        c: [5; 32],
        d: vec![3; 32],
        e: BTreeMap::from([(1, 10), (2, 20)]),
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
    e: BTreeMap<u8, u64>,
}

/// Test to_bytes, from_bytes, to_hash, to_yaml, from_yaml, to_hex, from_hex
fn test_all() {
    let all = All {
        a: 1,
        b: "hello".to_owned(),
        c: [5; 32],
        d: vec![3; 32],
        e: BTreeMap::from([(1, 10), (2, 20)]),
    };
    let bytes = all.to_bytes();
    let all2 = All::from_bytes(bytes);
    assert_eq!(all, all2);

    let hash: [u8; 32] = all.to_hash::<sha2::Sha256>().into();
    assert_eq!(
        hash.to_hex(),
        "3d6c0032eeecbc00e38abb4ae0d92006480faee86494ee42270ad05501ef3791"
    );
    assert_eq!(
        hash.to_upper_hex(),
        "3D6C0032EEECBC00E38ABB4AE0D92006480FAEE86494EE42270AD05501EF3791"
    );
    assert_eq!(
        hash.to_hex_with_0x(),
        "0x3d6c0032eeecbc00e38abb4ae0d92006480faee86494ee42270ad05501ef3791"
    );

    let yaml = all.to_yaml();
    let all2 = All::from_yaml(yaml);
    assert_eq!(all, all2);
}
