[![GitHub Actions Workflow Status](https://img.shields.io/github/actions/workflow/status/qjerome/human-bytes/rust.yml?style=for-the-badge)](https://github.com/qjerome/human-bytes/actions/workflows/rust.yml)
[![Crates.io Version](https://img.shields.io/crates/v/huby?style=for-the-badge)](https://crates.io/crates/huby)
[![docs.rs](https://img.shields.io/docsrs/huby?style=for-the-badge&logo=docs.rs&color=blue)](https://docs.rs/huby)

<!-- cargo-rdme start -->

# Human Bytes

`huby` is a library for easily handling byte sizes.

# Crate features

## Default
* `std`: Enable feature depending on the Rust standard library

## Optional
* `serde`: Enable serialization/deserialization via [serde](https://serde.rs/).

# Examples

## Basics

```rust
use huby::ByteSize;

assert_eq!("42.42 KB".parse::<ByteSize>().unwrap(), ByteSize::from_kb_f64(42.42));
```

## Use with serde

```rust
use huby::ByteSize;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct Logger {
    path: String,
    max_size: ByteSize,
}

let logger = Logger {
    path: "some_path".into(),
    max_size: ByteSize::from_gb(1),
};

// Serialize
let j = serde_json::to_string(&logger).unwrap();
assert_eq!(r#"{"path":"some_path","max_size":"1GB"}"#, j);

// Deserialize
let l: Logger = serde_json::from_str(&j).unwrap();
assert_eq!(l.max_size, ByteSize::from_mb(1024));
```

<!-- cargo-rdme end -->
