![GitHub Workflow Status (with event)](https://img.shields.io/github/actions/workflow/status/mangopanda455/signum/.github%2Fworkflows%2Frust.yml)
![Crates.io](https://img.shields.io/crates/d/signum-sign)
# Signum
Adds the signum function to Rust. The signum function or sign function will return 1 when a positive number is passed in, -1 when a negative number is passed in, or 0 when 0 is passed in.

## Installation:
```sh
cargo add signum-sign
```

## Usage:
```rust
use signum_sign::sgn;

fn main() {
    println!("{}, {}, {}", sgn(0), sgn(-2), sgn(2)); // Outputs "0, -1, 1"
}
```
