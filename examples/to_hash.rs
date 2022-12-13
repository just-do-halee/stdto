use std::{collections::HashMap, error::Error};

use stdto::prelude::*;

#[stdto::bytes]
#[stdto::hash]
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

    let hash: [u8; 32] = origin.to_hash::<sha2::Sha256>().into();
    let want = [
        94, 85, 222, 199, 98, 247, 173, 139, 127, 248, 253, 9, 101, 4, 62, 119, 252, 36, 222, 79,
        4, 212, 157, 174, 66, 199, 110, 103, 88, 59, 89, 227,
    ];
    assert_eq!(hash, want);

    use sha2::Digest;
    let mut hasher = sha2::Sha256::new();
    origin.to_hash_into(&mut hasher);
    let hash: [u8; 32] = hasher.finalize().into();
    assert_eq!(hash, want);
    Ok(())
}
