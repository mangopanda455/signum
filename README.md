![GitHub Workflow Status (with event)](https://img.shields.io/github/actions/workflow/status/mangopanda455/signum/.github%2Fworkflows%2Frust.yml)
![Crates.io](https://img.shields.io/crates/d/signum-sign)
![Crates.io](https://img.shields.io/crates/v/signum-sign)
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

## Heron:
This branch also adds the Heron method of approximating the square root of a number. This method is more accurate than the Babylonian method, but is slower.

## Usage:
```rust
use signum_sign::heron;

fn main() {
    println!("{}", heron(2.0)); // Outputs "1.414213562373095"
}
```
