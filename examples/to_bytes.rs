use std::{collections::HashMap, error::Error, io::Cursor};

use stdto::ToBytes;

// #[stdto::serde]
// #[derive(ToBytes)]
// same as
#[stdto::bytes]
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

    let mut bytes: Vec<u8>;

    bytes = origin.to_bytes();
    Test::from_bytes(&bytes);

    bytes.clear();

    bytes = origin.try_to_bytes()?;
    Test::try_from_bytes(&bytes)?;

    bytes.clear();

    origin.to_bytes_into(&mut bytes);
    Test::from_bytes_from(&mut Cursor::new(&bytes));

    bytes.clear();

    origin.try_to_bytes_into(&mut bytes)?;
    Test::try_from_bytes_from(&mut Cursor::new(&bytes))?;

    Ok(())
}
