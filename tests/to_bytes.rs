use std::collections::HashMap;

use stdto::ToBytes;

#[stdto::bytes]
#[derive(Debug, PartialEq, Eq)]
struct Test<T>
where
    T: stdto::serde::Serialize,
{
    a: u32,
    b: String,
    c: [u8; 32],
    d: Vec<u8>,
    e: HashMap<u32, T>,
}

fn assert_eq_struct<T: PartialEq + Eq>(a: &T, b: &T) {
    if a != b {
        panic!("assertion failed: struct");
    }
}

#[test]
fn to_bytes() {
    let origin: Test<String> = Test {
        a: 1,
        b: "test".to_string(),
        c: [0; 32],
        d: vec![0; 32],
        e: HashMap::new(),
    };

    let bytes = origin.to_bytes();
    let new1 = Test::from_bytes(&bytes);

    assert_eq_struct(&origin, &new1);

    let mut bytes2 = Vec::new();
    origin.to_bytes_into(&mut bytes2);
    let new2 = Test::from_bytes(&bytes2);

    assert_eq_struct(&origin, &new2);

    assert_eq!(bytes, bytes2);

    assert_eq_struct(&new1, &new2);
}
