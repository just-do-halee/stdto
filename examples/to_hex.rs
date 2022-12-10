use std::{collections::HashMap, error::Error};

use stdto::{ToHash, ToHex};

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
    let bytes = b"hello world";
    assert_eq!(bytes.to_hex(), "68656c6c6f20776f726c64");
    assert_eq!(bytes.to_upper_hex(), "68656C6C6F20776F726C64");
    assert_eq!(bytes.to_hex_with_0x(), "0x68656c6c6f20776f726c64");
    assert_eq!(bytes.to_upper_hex_with_0x(), "0x68656C6C6F20776F726C64");

    let mut hex = String::new();
    bytes.to_hex_into(&mut hex);
    assert_eq!(hex, "68656c6c6f20776f726c64");
    hex.clear();
    bytes.to_upper_hex_into(&mut hex);
    assert_eq!(hex, "68656C6C6F20776F726C64");
    hex.clear();
    bytes.to_hex_into_with_0x(&mut hex);
    assert_eq!(hex, "0x68656c6c6f20776f726c64");
    hex.clear();
    bytes.to_upper_hex_into_with_0x(&mut hex);
    assert_eq!(hex, "0x68656C6C6F20776F726C64");

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
    assert_eq!(
        hash.to_hex(),
        "5e55dec762f7ad8b7ff8fd0965043e77fc24de4f04d49dae42c76e67583b59e3"
    );
    assert_eq!(
        hash.to_upper_hex(),
        "5E55DEC762F7AD8B7FF8FD0965043E77FC24DE4F04D49DAE42C76E67583B59E3"
    );
    assert_eq!(
        hash.to_hex_with_0x(),
        "0x5e55dec762f7ad8b7ff8fd0965043e77fc24de4f04d49dae42c76e67583b59e3"
    );
    assert_eq!(
        hash.to_upper_hex_with_0x(),
        "0x5E55DEC762F7AD8B7FF8FD0965043E77FC24DE4F04D49DAE42C76E67583B59E3"
    );

    let hex = "2f048f12f50bf07e2843f6a3d1a91b21afaf7a1a8f4010fb8ab2a7ff57ad4625";
    let mut hash = [0u8; 42];
    let n = hash.copy_from_hex(hex);
    let want = [
        47, 4, 143, 18, 245, 11, 240, 126, 40, 67, 246, 163, 209, 169, 27, 33, 175, 175, 122, 26,
        143, 64, 16, 251, 138, 178, 167, 255, 87, 173, 70, 37,
    ];
    assert_eq!(&hash[..n], want);

    let hash = Vec::<u8>::from_hex(hex);
    assert_eq!(hash, want);

    let hex = "0x2f048f12f50bf07e2843f6a3d1a91b21afaf7a1a8f4010fb8ab2a7ff57ad4625";
    let hash: [u8; 32] = Vec::<u8>::from_hex(hex).try_into().unwrap();
    assert_eq!(hash, want);

    let hash = Vec::<u8>::from_hex_from(hex.as_bytes());
    assert_eq!(hash, want);

    Ok(())
}
