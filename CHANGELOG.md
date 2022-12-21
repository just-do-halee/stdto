<!-- next-header -->

## [unreleased] - ReleaseDate


## Improved

- Default export `AsBytes` trait.
- Fixed all of `AsRef<[u8]>` to `AsBytes`.

Released by [@just-do-halee](https://github.com/just-do-halee).

## [0.9.0] - 2022-12-21

## BREAKING CHANGES

- Some of methods in the `ToJson`.

## Added

- Added features `yaml` and `toml` and `file`.
- Added `ToYaml` and `stdto::yaml` conversion.
- Added `ToToml` and `stdto::toml` conversion.

Released by [@just-do-halee](https://github.com/just-do-halee).

## [0.8.0] - 2022-12-21

## Improved

- Added `#[non_exhaustive]` attribute on the Error enum and Endian enum.

Released by [@just-do-halee](https://github.com/just-do-halee).

## [0.7.0] - 2022-12-20

## BREAKING CHANGES

- features in Cargo.toml.

## Added

- Added `AsBytes` for general purpose trait.
- Refactor `ToHex` and `ToStringForRef`.
- Renamed `ToStringForRef` to `ToStringForBytes`.
- Added `DebugBytes` derive macro for debugging with automatic bytes representation when the `DebugBytes` is derived.
- Added `DebugHex` derive macro for debugging with automatic hexadecimal string representation when the `DebugHex` is derived.

Released by [@just-do-halee](https://github.com/just-do-halee).

## [0.6.0] - 2022-12-14

## Added

- Added `ToJson` and `stdto::json` conversion.
- Added `ToStringForRef` functional trait for converting between `AsRef<[u8]>` and `String` or `&str`.

## Improved

- No need to implement `#[stdo::serde]` anymore. Also no need to concern about conflicting `#[stdto::serde]` and `#[stdto::bytes]` and `#[stdto::json]`.
- Changed all of `..from_bytes_..<'a>(..: &'a [u8])..` to `..from_bytes_..(..: impl AsRef<[u8]>)..`.
- Added `as_bytes()`, `into_bytes()` in the `ToBytesForRef` trait.

Released by [@just-do-halee](https://github.com/just-do-halee).

## [0.5.0] - 2022-12-13

## Added

- Added const fn `ToBytesOptions::default()`.
- Added `ToBytesForRef` functional trait for creating `Vec<u8>` from `AsRef<[u8]>`.

## Improved

- Refactor Cargo.toml: dependency features.
- It's okay to derive `ToBytes` without `serde::Serialize`. But if you want to use the methods of de/serialization, you should derive `serde::Serialize` or `serde::Deserialize`.
- ..from_hex(..:  `impl AsRef<str>`) to ..from_hex(..: `impl AsRef<[u8]>`).

Released by [@just-do-halee](https://github.com/just-do-halee).

## [0.4.0] - 2022-12-13

## Added

- Added `prelude` module for core and derive traits only.
- Added `to_ne_bytes..` and `from_ne_bytes..` methods in `ToBytes`.

## Fixed

- Fixed `#[stdto::bytes(endian = "native")]` typo bug.

Released by [@just-do-halee](https://github.com/just-do-halee).

## [0.3.0] - 2022-12-12

## BREAKING CHANGES

- Renamed the methods in `ToBytes` and Added the const ToBytesOptions.
- Re-design [features].

## Added

- Added an attribute #[stdto::bytes(endian = "...")]. (little/big/native)

Released by [@just-do-halee](https://github.com/just-do-halee).

## [0.2.0] - 2022-12-10

## Added

- Added `ToHash` and `stdto::hash` conversion.
- Added `ToHex` and implement it to AsRef<[u8]>.

## Improved

- Now customizable `ToBytes` trait.

Released by [@just-do-halee](https://github.com/just-do-halee).

## [0.1.0] - 2022-12-10

Released by [@just-do-halee](https://github.com/just-do-halee).

<!-- next-url -->

[unreleased]: https://github.com/just-do-halee/stdto/compare/stdto_core-v0.9.0...HEAD

[0.9.0]: https://github.com/just-do-halee/stdto/compare/stdto_core-v0.8.0...stdto_core-v0.9.0

[0.8.0]: https://github.com/just-do-halee/stdto/compare/stdto_core-v0.7.0...stdto_core-v0.8.0

[0.7.0]: https://github.com/just-do-halee/stdto/compare/stdto_core-v0.6.0...stdto_core-v0.7.0

[0.6.0]: https://github.com/just-do-halee/stdto/compare/stdto_core-v0.5.0...stdto_core-v0.6.0

[0.5.0]: https://github.com/just-do-halee/stdto/compare/v0.4.0...stdto_core-v0.5.0

[0.4.0]: https://github.com/just-do-halee/stdto/compare/v0.3.0...v0.4.0

[0.3.0]: https://github.com/just-do-halee/stdto/compare/v0.2.0...v0.3.0

[0.2.0]: https://github.com/just-do-halee/stdto/compare/v0.1.0...v0.2.0

[0.1.0]: https://github.com/just-do-halee/stdto/compare/v0.1.0...v0.1.0
