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

//! Lebicon implements the `codicon` traits for LEB128 encoding / decoding.
//!
//! # Examples
//!
//! ```rust
//! extern crate codicon;
//! extern crate lebicon;
//!
//! use codicon::{Decoder, Encoder};
//! use lebicon::Leb128;
//! use std::io::Write;
//!
//! let encoded = [198, 253, 255, 127];
//! let decoded = 268435142u64;
//!
//! let value = u64::decode(&mut &encoded[..], Leb128).unwrap();
//! assert_eq!(value, decoded);
//!
//! let mut value: Vec<u8> = Vec::new();
//! decoded.encode(&mut value, Leb128).unwrap();
//! assert_eq!(&value[..], &encoded[..]);
//! ```

extern crate codicon;
extern crate signrel;
extern crate uabs;

#[cfg(test)]
extern crate leb128;

#[cfg(test)]
mod tests;

use uabs::UnsignedAbs;
use signrel::SignRel;
use std::cmp::min;
use std::slice;
use std::error;
use std::fmt;
use std::mem;
use std::io;

pub struct Leb128;

/// The errors possibly returned when reading a LEB128-encoded integer.
#[derive(Debug)]
pub enum Error {
    /// A propagated error from the `std::io::Read` implementation.
    IoError(io::Error),

    /// The LEB128-encoded integer is too large to fit in this integer type.
    Overflow
}

impl error::Error for Error {}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Error::IoError(ref e) => e.fmt(f),
            Error::Overflow => write!(f, "LEB128 integer overflow"),
        }
    }
}

impl From<io::Error> for Error {
    fn from(e: io::Error) -> Self {
        Error::IoError(e)
    }
}

trait WriteByte: io::Write {
    fn write_byte(&mut self, byte: u8) -> io::Result<()> {
        self.write_all(slice::from_ref(&byte))?;
        Ok(())
    }
}

trait ReadByte: io::Read {
    fn read_byte(&mut self) -> io::Result<u8> {
        let mut byte = 0u8;
        self.read_exact(slice::from_mut(&mut byte))?;
        Ok(byte)
    }
}

trait ByteMax: SignRel {
    const MAX: Self::Unsigned;
}

const CONT: u8 = 0b10000000;

impl<T: io::Write> WriteByte for T {}
impl<T: io::Read> ReadByte for T {}

macro_rules! leb_impl {
    ($($s:ident:$u:ident)*) => (
        $(
            impl ByteMax for $s {
                const MAX: Self::Unsigned = 0b00111111;
            }

            impl ByteMax for $u {
                const MAX: Self::Unsigned = 0b01111111;
            }

            leb_impl! { $s }
            leb_impl! { $u }
        )*
    );

    ($t:ident) => (
        impl codicon::Decoder<Leb128> for $t {
            type Error = Error;
            fn decode(reader: &mut impl io::Read, _: Leb128) -> Result<Self, Error> {
                const BITS: u32 = mem::size_of::<$t>() as u32 * 8;
                let mut value = <Self as SignRel>::Unsigned::from(0u8);
                let mut shift = 0u32;
                let mut byte = CONT;

                while byte & CONT == CONT {
                    if shift > BITS {
                        return Err(Error::Overflow);
                    }

                    byte = reader.read_byte()?;

                    let low = <Self as SignRel>::Unsigned::from(byte & !CONT);
                    value |= low << shift;
                    shift += 7;
                }

                if shift > BITS {
                    // Ensure that none of the overflowed bits matter.
                    let offs = 1 - (Self::MAX >> 6);
                    let mask = 0b11111111 << 7 - (shift - BITS) >> offs;
                    if byte & mask != mask && byte & mask != 0 {
                        return Err(Error::Overflow);
                    }
                }

                // Convert to signed and sign extend.
                let off = BITS - min(shift, BITS);
                value <<= off;
                Ok(value as Self >> off)
            }
        }

        impl codicon::Encoder<Leb128> for $t {
            type Error = io::Error;
            fn encode(&self, writer: &mut impl io::Write, _: Leb128) -> Result<(), io::Error> {
                let mut value = *self;

                while value.uabs() > Self::MAX {
                    writer.write_byte(value as u8 | CONT)?;
                    value >>= 7;
                }

                Ok(writer.write_byte(value as u8 & !CONT)?)
            }
        }
    );
}

leb_impl! { isize:usize i128:u128 i64:u64 i32:u32 i16:u16 i8:u8 }
