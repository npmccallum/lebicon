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

use super::*;

const UVALS: &[u64] = &[
    0, 1, 2, 3, 4, 5, 6, 7,
    
    u8::max_value() as u64 - 1,
    u8::max_value() as u64,
    u8::max_value() as u64 + 1,
    
    u16::max_value() as u64 - 1,
    u16::max_value() as u64,
    u16::max_value() as u64 + 1,
    
    u32::max_value() as u64 - 1,
    u32::max_value() as u64,
    u32::max_value() as u64 + 1,
    
    u64::max_value() - 1,
    u64::max_value(),
];

#[test]
fn u64_read_leb128() {
    use leb128;

    for i in UVALS {
        let mut b = Vec::new();
        let n = leb128::write::unsigned(&mut b, *i).unwrap();
        eprintln!("{:20}: {:?}", *i, b);
        let v = u64::leb128_read(&mut &b[..n]).unwrap();
        assert_eq!(v, *i);
    }
}

#[test]
fn u64_write_leb128() {
    use leb128;

    for i in UVALS {
        let mut b = Vec::new();
        i.leb128_write(&mut b).unwrap();
        eprintln!("{:20}: {:?}", *i, b);
        let v = leb128::read::unsigned(&mut &b[..]).unwrap();
        assert_eq!(v, *i);
    }
}

const SVALS: &[i64] = &[
    i64::min_value(),
    i64::min_value() + 1,
    
    i32::min_value() as i64 - 1,
    i32::min_value() as i64,
    i32::min_value() as i64 + 1,
    
    i16::min_value() as i64 - 1,
    i16::min_value() as i64,
    i16::min_value() as i64 + 1,
    
    i8::min_value() as i64 - 1,
    i8::min_value() as i64,
    i8::min_value() as i64 + 1,
    
    -3, -2, -1, 0, 1, 2, 3,
    
    i8::max_value() as i64 - 1,
    i8::max_value() as i64,
    i8::max_value() as i64 + 1,
    
    i16::max_value() as i64 - 1,
    i16::max_value() as i64,
    i16::max_value() as i64 + 1,
    
    i32::max_value() as i64 - 1,
    i32::max_value() as i64,
    i32::max_value() as i64 + 1,
    
    i64::max_value() - 1,
    i64::max_value(),
];

#[test]
fn i64_read_leb128_i64() {
    use leb128;

    for i in SVALS {
        let mut b = Vec::new();
        let n = leb128::write::signed(&mut b, *i).unwrap();
        eprintln!("{:20}: {:?}", *i, b);
        let v = i64::leb128_read(&mut &b[..n]).unwrap();
        assert_eq!(v, *i);
    }
}

#[test]
fn i64_write_leb128() {
    use leb128;

    for i in SVALS {
        let mut b = Vec::new();
        i.leb128_write(&mut b).unwrap();
        eprintln!("{:20}: {:?}", *i, b);
        let v = leb128::read::signed(&mut &b[..]).unwrap();
        assert_eq!(v, *i);
    }
}

// http://dwarfstd.org/doc/DWARF4.pdf (p162)
const UDWARF: &'static [(u16, &'static [u8])] = &[
    (2, &[2]),
    (127, &[127]),
    (128, &[0+0x80, 1]),
    (129, &[1+0x80, 1]),
    (130, &[2+0x80, 1]),
    (12857, &[57+0x80, 100]),
];

#[test]
fn u16_read_dwarf() {
    for (i, b) in UDWARF {
        assert_eq!(u16::leb128_read(&mut &**b).unwrap(), *i);
    }
}

#[test]
fn u16_write_dwarf() {
    for (i, b) in UDWARF {
        let mut v = Vec::new();
        i.leb128_write(&mut v).unwrap();
        assert_eq!(&v[..], *b);
    }
}

// From http://dwarfstd.org/doc/DWARF4.pdf (p163)
const SDWARF: &'static [(i16, &'static [u8])] = &[
    (2,          &[2]),
    (-2,         &[0x7e]),
    (127,        &[127+0x80, 0]),
    (-127,       &[1+0x80, 0x7f]),
    (128,        &[0+0x80, 1]),
    (-128,       &[0+0x80, 0x7f]),
    (129,        &[1+0x80, 1]),
    (-129,       &[0x7f+0x80, 0x7e]),
];

#[test]
fn i16_read_dwarf() {
    for (i, b) in SDWARF {
        assert_eq!(i16::leb128_read(&mut &**b).unwrap(), *i);
    }
}

#[test]
fn i16_write_dwarf() {
    for (i, b) in SDWARF {
        let mut v = Vec::new();
        i.leb128_write(&mut v).unwrap();
        assert_eq!(&v[..], *b);
    }
}

fn overflow<T: Reader>(buf: &[u8]) {
    match T::leb128_read(&mut &buf[..]) {
        Ok(_) => panic!("Unexpected success!"),
        Err(e) => match e {
            Error::Overflow => (),
                          _ => panic!("Unexpected error value!"),
        }
    }
}

#[test]
fn u8_overflow() {
    overflow::<u8>(&[128, 2]);
}

#[test]
fn u16_overflow() {
    overflow::<u16>(&[128, 128, 4]);
}

#[test]
fn u32_overflow() {
    overflow::<u32>(&[128, 128, 128, 128, 16]);
}

#[test]
fn u64_overflow() {
    overflow::<u64>(&[128, 128, 128, 128, 128, 128, 128, 128, 128, 2]);
}

#[test]
fn u128_overflow() {
    overflow::<u128>(&[128, 128, 128, 128, 128, 128, 128, 128, 128,
                       128, 128, 128, 128, 128, 128, 128, 128, 128, 4]);
}

#[test]
fn i8_overflow() {
    overflow::<i8>(&[128, 1]);
    overflow::<i8>(&[128, 254]);
}

#[test]
fn i16_overflow() {
    overflow::<i8>(&[128, 128, 2]);
    overflow::<i8>(&[128, 128, 252]);
}

#[test]
fn i32_overflow() {
    overflow::<i8>(&[128, 128, 128, 128, 8]);
    overflow::<i8>(&[128, 128, 128, 128, 240]);
}

#[test]
fn i64_overflow() {
    overflow::<i8>(&[128, 128, 128, 128, 128, 128, 128, 128, 128, 1]);
    overflow::<i8>(&[128, 128, 128, 128, 128, 128, 128, 128, 128, 254]);
}

#[test]
fn i128_overflow() {
    overflow::<i128>(&[128, 128, 128, 128, 128, 128, 128, 128, 128,
                       128, 128, 128, 128, 128, 128, 128, 128, 128, 2]);
    overflow::<i128>(&[128, 128, 128, 128, 128, 128, 128, 128, 128,
                       128, 128, 128, 128, 128, 128, 128, 128, 128, 252]);
}
