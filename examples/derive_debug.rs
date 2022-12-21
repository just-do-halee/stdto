use std::collections::BTreeMap;

use stdto::prelude::*;

#[stdto::bytes]
// #[cfg(feature = "debug")]
#[derive(DebugBytes)]
// #[cfg(not(feature = "debug"))]
// #[derive(Debug)]
struct Test {
    a: u32,
    b: String,
    c: [u8; 32],
    d: Vec<u8>,
    e: BTreeMap<u8, f64>,
}

#[stdto::bytes]
// #[cfg(feature = "debug")]
#[derive(DebugHex)]
// #[cfg(not(feature = "debug"))]
// #[derive(Debug)]
struct Test2 {
    a: u32,
    b: String,
    c: [u8; 32],
    d: Vec<u8>,
    e: BTreeMap<u8, f64>,
}

fn main() {
    let origin = Test {
        a: 1,
        b: "test".to_string(),
        c: [5; 32],
        d: vec![3; 32],
        e: BTreeMap::new(),
    };
    let debuged = format!("{:?}", origin);
    assert_eq!(debuged, "#[1, 0, 0, 0, 4, 0, 0, 0, 0, 0, 0, 0, 116, 101, 115, 116, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 32, 0, 0, 0, 0, 0, 0, 0, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 0, 0, 0, 0, 0, 0, 0, 0]#");

    let origin2 = Test2 {
        a: 1,
        b: "test".to_string(),
        c: [5; 32],
        d: vec![3; 32],
        e: BTreeMap::new(),
    };
    let debuged = format!("{:?}", origin2);
    assert_eq!(debuged, "#\"0x010000000400000000000000746573740505050505050505050505050505050505050505050505050505050505050505200000000000000003030303030303030303030303030303030303030303030303030303030303030000000000000000\"#");
}
