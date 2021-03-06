// SPDX-License-Identifier: Apache-2.0

//! Lebicon implements the `codicon` traits for LEB128 encoding / decoding.
//!
//! # Examples
//!
//! ```rust
//! use codicon::*;
//! use lebicon::Leb128;
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

use codicon::*;
use signrel::SignRel;
use uabs::Uabs;

pub struct Leb128;

#[derive(Debug)]
pub enum Error {
    IoError(std::io::Error),
    Overflow,
}

impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Error::Overflow => None,
            Error::IoError(e) => Some(e),
        }
    }
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match *self {
            Error::IoError(ref e) => e.fmt(f),
            Error::Overflow => write!(f, "LEB128 integer overflow"),
        }
    }
}

impl From<std::io::Error> for Error {
    fn from(e: std::io::Error) -> Self {
        Error::IoError(e)
    }
}

trait ByteMax: SignRel {
    const BYTE_MAX: Self::Unsigned;
}

const CONT: u8 = 0b10000000;

macro_rules! leb_impl {
    ($($s:ident:$u:ident)*) => (
        $(
            impl ByteMax for $s {
                const BYTE_MAX: Self::Unsigned = 0b00111111;
            }

            impl ByteMax for $u {
                const BYTE_MAX: Self::Unsigned = 0b01111111;
            }

            leb_impl! { $s }
            leb_impl! { $u }
        )*
    );

    ($t:ident) => (
        impl Decoder<Leb128> for $t {
            type Error = Error;

            fn decode(mut reader: impl Read, _: Leb128) -> Result<Self, Error> {
                const BITS: u32 = std::mem::size_of::<$t>() as u32 * 8;
                let mut value = <Self as SignRel>::Unsigned::from(0u8);
                let mut shift = 0u32;
                let mut byte = CONT;

                while byte & CONT == CONT {
                    if shift > BITS {
                        return Err(Error::Overflow);
                    }

                    let mut bytes = [0u8; 1];
                    reader.read_exact(&mut bytes)?;
                    byte = bytes[0];

                    let low = <Self as SignRel>::Unsigned::from(byte & !CONT);
                    value |= low << shift;
                    shift += 7;
                }

                if shift > BITS {
                    // Ensure that none of the overflowed bits matter.
                    let offs = 1 - (Self::BYTE_MAX >> 6);
                    let mask = 0b11111111 << 7 - (shift - BITS) >> offs;
                    if byte & mask != mask && byte & mask != 0 {
                        return Err(Error::Overflow);
                    }
                }

                // Convert to signed and sign extend.
                let off = BITS - std::cmp::min(shift, BITS);
                value <<= off;
                Ok(value as Self >> off)
            }
        }

        impl Encoder<Leb128> for $t {
            type Error = std::io::Error;

            fn encode(&self, mut writer: impl Write, _: Leb128) -> std::io::Result<()> {
                let mut value = *self;

                while value.uabs() > Self::BYTE_MAX {
                    writer.write_all(&[value as u8 | CONT])?;
                    value >>= 7;
                }

                Ok(writer.write_all(&[value as u8 & !CONT])?)
            }
        }
    );
}

leb_impl! { isize:usize i128:u128 i64:u64 i32:u32 i16:u16 i8:u8 }
