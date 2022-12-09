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
stdto = "0.1.0"
```

## **`How to`**

```rust
#[stdto::bytes]
struct Test {
    a: u32,
    b: String,
    c: [u8; 32],
    d: Vec<u8>,
    e: HashMap<u8, f64>,
}

bytes = origin.to_bytes();
Test::from_bytes(&bytes);
```
