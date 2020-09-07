[![Workflow Status](https://github.com/enarx/lebicon/workflows/test/badge.svg)](https://github.com/enarx/lebicon/actions?query=workflow%3A%22test%22)
[![Average time to resolve an issue](https://isitmaintained.com/badge/resolution/enarx/lebicon.svg)](https://isitmaintained.com/project/enarx/lebicon "Average time to resolve an issue")
[![Percentage of issues still open](https://isitmaintained.com/badge/open/enarx/lebicon.svg)](https://isitmaintained.com/project/enarx/lebicon "Percentage of issues still open")
![Maintenance](https://img.shields.io/badge/maintenance-activly--developed-brightgreen.svg)

# lebicon

Lebicon implements the `codicon` traits for LEB128 encoding / decoding.

## Examples

```rust
use codicon::*;
use lebicon::Leb128;

let encoded = [198, 253, 255, 127];
let decoded = 268435142u64;

let value = u64::decode(&mut &encoded[..], Leb128).unwrap();
assert_eq!(value, decoded);

let mut value: Vec<u8> = Vec::new();
decoded.encode(&mut value, Leb128).unwrap();
assert_eq!(&value[..], &encoded[..]);
```

License: Apache-2.0
