[![Build Status](https://travis-ci.org/psilocybin/lebicon.svg?branch=master)](https://travis-ci.org/psilocybin/lebicon)
![Rust Version 1.28+](https://img.shields.io/badge/rustc-v1.28%2B-blue.svg)
[![Crate](https://img.shields.io/crates/v/lebicon.svg)](https://crates.io/crates/lebicon)
[![Docs](https://docs.rs/lebicon/badge.svg)](https://docs.rs/lebicon)

Lebicon is a crate for encoding or decoding integers in LEB128 format.

This is accomplished by implementing the `codicon` traits for all of Rust's
built-in integer types.

# Install

Run this command:

    $ cargo add lebicon

# Examples

```rust
extern crate codicon;
extern crate lebicon;

use codicon::{Decoder, Encoder};
use lebicon::Leb128;
use std::io::Write;

let encoded = [198, 253, 255, 127];
let decoded = 268435142u64;

let value = u64::decode(&mut &encoded[..], Leb128).unwrap();
assert_eq!(value, decoded);

let mut value: Vec<u8> = Vec::new();
decoded.encode(&mut value, Leb128).unwrap();
assert_eq!(&value[..], &encoded[..]);
```
