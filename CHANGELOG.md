<!-- next-header -->

## [unreleased] - ReleaseDate

## Added

- Added const fn `ToBytesOptions::default()`.

## Improved

- Refactor Cargo.toml: dependency features.

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

[unreleased]: https://github.com/just-do-halee/stdto/compare/stdto_core-v0.4.0...HEAD

[0.4.0]: https://github.com/just-do-halee/stdto/compare/v0.3.0...stdto_core-v0.4.0

[0.3.0]: https://github.com/just-do-halee/stdto/compare/v0.2.0...v0.3.0

[0.2.0]: https://github.com/just-do-halee/stdto/compare/v0.1.0...v0.2.0

[0.1.0]: https://github.com/just-do-halee/stdto/compare/v0.1.0...v0.1.0
