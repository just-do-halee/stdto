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
stdto = "0.7.0"
```

## **`Goal`**

As a blockchain developer who specializes in Rust, I often find it challenging to work with bytes, hashes, and JSON. The Rust ecosystem is decentralized and many popular crates are old and complex. This makes it difficult to find simple, well-abstracted solutions that are easy to understand. I created the Stdto crate to address this need. The goal of Stdto is to provide a standard library-like interface that makes it easy for users to work with and understand primitive data structures.

## **`Features`**

```toml
default = ["derive", "serde", "bytes", "hash", "json", "hex"]
```
```sh
cargo add stdto --features "derive bytes" # [derive, serde, bytes]
cargo add stdto --features "derive hash" # [derive, serde, bytes, hash]
cargo add stdto --features "derive json" # [derive, serde, json]
cargo add stdto --features "derive hex" # [derive, serde, bytes, hex]
```

## **`Examples`**

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
    e: HashMap<u8, f64>,
}

bytes = test.to_bytes();
Test::from_bytes(bytes);
```

```rust
#[stdto::bytes]
#[stdto::hash]
struct Test {
    ...
}

hash = test.to_hash::<Sha256>();
```

```rust
#[stdto::json]
struct Test {
    a: u32,
    b: String,
    c: [u8; 32],
    d: Vec<u8>,
    e: HashMap<u8, f64>,
}

json = test.to_json();
Test::from_json(json);
```

```rust
// AsRef<[u8]> to hex

hex = hash.to_hex();
Vec::<u8>::from_hex(hex);

mut arr = [0u8; 32];
arr.copy_from_hex(hex);
```

```rust
// AsRef<[u8]> <-> String, &str

arr = [72, 105, 77, 111, 109];
s1 = arr.into_string();
bytes = s1.to_bytes();
s2 = bytes.as_str();

assert_eq!(s1, s2);
```
