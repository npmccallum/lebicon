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

use super::error::Error;

use std::cmp::min;
use std::slice;
use std::mem;
use std::io;

/// A trait which allows reading of a LEB128-encoded integer.
pub trait Reader: Sized {
    /// Read a LEB128-encoded integer from the supplied `std::io::Read` implementation.
    fn leb128_read<T: io::Read>(reader: &mut T) -> Result<Self, Error>;
}

macro_rules! rw_impl {
    ($($s:ident:$u:ty)*) => (
        $(
            impl Reader for $s {
                fn leb128_read<T: io::Read>(reader: &mut T) -> Result<Self, Error> {
                    let bits = mem::size_of::<Self>() as u32 * 8;
                    let mut value: $u = 0;
                    let mut shift = 0u32;
                    let mut done = false;
                    let mut byte = 0u8;

                    while !done {
                        if shift > bits {
                            return Err(Error::Overflow);
                        }

                        reader.read_exact(slice::from_mut(&mut byte))?;

                        done = byte & 0b10000000 == 0;
                        let low = byte & 0b01111111;
                        value |= low as $u << shift;
                        shift += 7;
                    }

                    if shift > bits {
                        // Ensure that none of the overflowed bits matter.
                        let mask = 0b11111111 << (7 - (shift - bits)) >> 1;
                        if byte & mask != mask && byte & mask != 0 {
                            return Err(Error::Overflow);
                        }
                    }

                    // Convert to signed and sign extend.
                    let off = bits - min(shift, bits);
                    value <<= off;
                    Ok(value as $s >> off)
                }
            }

            impl Reader for $u {
                fn leb128_read<T: io::Read>(reader: &mut T) -> Result<Self, Error> {
                    let bits = mem::size_of::<Self>() as u32 * 8;
                    let mut value: $u = 0;
                    let mut shift = 0u32;
                    let mut done = false;

                    while !done {
                        let mut byte = 0u8;
                        reader.read_exact(slice::from_mut(&mut byte))?;

                        done = byte & 0b10000000u8 == 0;
                        let low = byte & 0b01111111u8;
                        value |= low as $u << shift;
                        shift += 7;

                        if shift + 1 - low.leading_zeros() > bits {
                            return Err(Error::Overflow);
                        }
                    }

                    Ok(value)
                }
            }
        )*
    )
}

rw_impl! { isize:usize i128:u128 i64:u64 i32:u32 i16:u16 i8:u8 }
