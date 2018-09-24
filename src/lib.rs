//
// Copyright 2018 Red Hat, Inc.
//
// Author: Nathaniel McCallum <npmccallum@redhat.com>
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.
//

//! Leben is a crate for encoding or decoding integers in LEB128 format.
//!
//! This is accomplished by extending the Rust native integer types with two traits:
//!   * `Reader` (one associated function: `leb128_read`)
//!   * `Writer` (one method: `leb128_write`)
//!
//! # Examples
//!
//! Reading and writing is done on any value that implements the `std::io::Read` or
//! `std::io::Write`, respectively. For example, we can write to `std::io::Sink`:
//!
//! ```
//! use std::io::sink;
//! use leben::Writer;
//!
//! let mut writer = sink();
//! let number: i16 = -582;
//! number.leb128_write(&mut writer).unwrap();
//! ```
//!
//! Don't forget that `std::vec::Vec<u8>` implements `std::io::Write` and `[u8]` implements
//! `std::io::Read`:
//!
//! ```
//! use leben::{Reader, Writer};
//! use std::io::Write;
//!
//! let encoded = [198, 253, 255, 127];
//! let decoded = 268435142u64;
//!
//! let value = u64::leb128_read(&mut &encoded[..]).unwrap();
//! assert_eq!(value, decoded);
//!
//! let mut value: Vec<u8> = Vec::new();
//! decoded.leb128_write(&mut value).unwrap();
//! assert_eq!(&value[..], &encoded[..]);
//! ```

#[cfg(test)]
extern crate leb128;

#[cfg(test)]
mod tests;

mod reader;
mod writer;
mod error;

pub use reader::Reader;
pub use writer::Writer;
pub use error::Error;
