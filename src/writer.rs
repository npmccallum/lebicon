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

use std::slice;
use std::io;

/// A trait which allows writing of a LEB128-encoded integer.
pub trait Writer {
    /// Write a LEB128-encoded integer to the supplied `std::io::Write` implementation.
    fn leb128_write<T: io::Write>(self, writer: &mut T) -> io::Result<()>;
}

macro_rules! rw_impl {
    ($($s:ident:$u:ty)*) => (
        $(
            impl Writer for $s {
                fn leb128_write<T: io::Write>(self, writer: &mut T) -> io::Result<()> {
                    let mut value = self;

                    #[inline]
                    fn abs(val: $s) -> $u {
                        match val.checked_abs() {
                            Some(v) => v as $u,
                            None => val as $u,
                        }
                    }

                    while abs(value) > 0b00111111 {
                        let byte = value as u8 & 0b01111111 | 0b10000000;
                        writer.write_all(slice::from_ref(&byte))?;
                        value >>= 7;
                    }

                    let byte = value as u8 & 0b01111111;
                    writer.write_all(slice::from_ref(&byte))?;
                    Ok(())
                }
            }

            impl Writer for $u {
                fn leb128_write<T: io::Write>(self, writer: &mut T) -> io::Result<()> {
                    let mut value = self;

                    while value > 0b01111111 {
                        let byte = value as u8 & 0b01111111 | 0b10000000;
                        writer.write_all(slice::from_ref(&byte))?;
                        value >>= 7;
                    }

                    let byte = value as u8;
                    writer.write_all(slice::from_ref(&byte))?;
                    Ok(())
                }
            }
        )*
    )
}

rw_impl! { isize:usize i128:u128 i64:u64 i32:u32 i16:u16 i8:u8 }
