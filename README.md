# **`Stdto`**

`stdto` provides a set of functional traits for conversion between various data representations.

[![CI][ci-badge]][ci-url]
[![Crates.io][crates-badge]][crates-url]
[![Licensed][license-badge]][license-url]
[![Twitter][twitter-badge]][twitter-url]

[ci-badge]: https://github.com/just-do-halee/stdto/actions/workflows/ci.yml/badge.svg
[crates-badge]: https://img.shields.io/crates/v/stdto.svg?labelColor=383636
[license-badge]: https://img.shields.io/crates/l/stdto?labelColor=383636
[twitter-badge]: https://img.shields.io/twitter/follow/do_halee?style=flat&logo=twitter&color=4a4646&labelColor=333131&label=just-do-halee
[ci-url]: https://github.com/just-do-halee/stdto/actions
[twitter-url]: https://twitter.com/do_halee
[crates-url]: https://crates.io/crates/stdto
[license-url]: https://github.com/just-do-halee/stdto

| [Examples](./examples/) | [Docs](https://docs.rs/stdto) | [Latest Note](./CHANGELOG.md) |

```toml
stdto = "0.12.0"
```

## **`Goal`**

As a blockchain developer who specializes in Rust, I often find it challenging to work with bytes, hashes, and JSON. The Rust ecosystem is decentralized and many popular crates are old and complex. This makes it difficult to find simple, well-abstracted solutions that are easy to understand. I created the Stdto crate to address this need. The goal of Stdto is to provide a standard library-like interface that makes it easy for users to work with and understand primitive data structures.

## **`Features`**

```toml
default = ["derive", "serde", "bytes", "hash", "json", "yaml", "toml", "file", "hex"]
```
```sh
cargo add stdto  # [derive, serde, bytes, hash, json, yaml, toml, file, hex]
cargo add stdto --features "derive bytes" # [derive, serde, bytes]
cargo add stdto --features "derive hash" # [derive, serde, bytes, hash]
cargo add stdto --features "derive json" # [derive, serde, json]
cargo add stdto --features "derive yaml" # [derive, serde, yaml]
cargo add stdto --features "derive toml" # [derive, serde, toml]
cargo add stdto --features "derive file" # [derive, serde, json, yaml, toml]
cargo add stdto --features "derive hex" # [derive, hex]
```

## [**`Examples`**](./examples/)

```rust
use stdto::prelude::*;
```

```rust
// #[stdto::bytes(endian = "little")]
#[stdto::bytes]
struct Test {
    a: u32,
    b: String,
    c: [u8; 32],
    d: Vec<u8>,
    e: BTreeMap<String, f64>,
}

let bytes = Test { .. }.to_bytes();
let test = Test::from_bytes(bytes);
// Test::try_from_bytes(bytes).unwrap();
```

```rust
#[stdto::bytes]
#[stdto::hash]
struct Test {
    ...
}

let hash = test.to_hash::<sha2::Sha256>();
// Any digest crate implemented hasher type
```

```rust
#[stdto::json]
// #[stdto::yaml]
// #[stdto::toml]
struct Test {
    ...
}

let json = test.to_json();
let test = Test::from_json(json);
// Test::try_from_json(json).unwrap();

// let yaml = test.to_yaml();
// let test = Test::from_yaml(yaml);
// let toml = test.to_toml();
// let test = Test::from_toml(toml);
```

```rust
// Any AsRef<[u8]> or AsBytes implemented to hex

let hex = bytes.to_hex();
let hex = hash.to_hex();
let bytes = Vec::<u8>::from_hex(hex);
// Vec::<u8>::try_from_hex(hex).unwrap();

let mut arr = [0u8; 32];
arr.copy_from_hex(hex);
```

```rust
// Any AsRef<[u8]> or AsBytes implemented <-> String, &str

let arr = [72, 105, 77, 111, 109];
let s1 = arr.into_string(); // .try_into_string().unwrap();
let bytes = s1.to_bytes();
let s2 = bytes.as_str(); // .try_as_str().unwrap();

assert_eq!(s1, s2);
```
