[![Build Status](https://travis-ci.org/npmccallum/leben.svg?branch=master)](https://travis-ci.org/npmccallum/leben)
![Rust Version 1.28+](https://img.shields.io/badge/rustc-v1.28%2B-blue.svg)
[![Crate](https://img.shields.io/crates/v/leben.svg)](https://crates.io/crates/leben)
[![Docs](https://docs.rs/leben/badge.svg)](https://docs.rs/leben)

Leben is a crate for encoding or decoding integers in LEB128 format.

This is accomplished by extending the Rust native integer types with two traits:
  * `Reader` (one associated function: `leb128_read`)
  * `Writer` (one method: `leb128_write`)

# Examples

Reading and writing is done on any value that implements the `std::io::Read` or
`std::io::Write`, respectively. For example, we can write to `std::io::Sink`:

```rust
use std::io::sink;
use leben::Writer;

let mut writer = sink();
let number: i16 = -582;
number.leb128_write(&mut writer).unwrap();
```

Don't forget that `std::vec::Vec<u8>` implements `std::io::Write` and `[u8]`
implements `std::io::Read`:

```rust
use leben::{Reader, Writer};
use std::io::Write;

let encoded = [198, 253, 255, 127];
let decoded = 268435142u64;

let value = u64::leb128_read(&mut &encoded[..]).unwrap();
assert_eq!(value, decoded);

let mut value: Vec<u8> = Vec::new();
decoded.leb128_write(&mut value).unwrap();
assert_eq!(&value[..], &encoded[..]);
```
