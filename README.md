# rust-nearly-eq
[![crates.io badge](https://img.shields.io/crates/v/nearly-eq.svg)](https://crates.io/crates/nearly-eq)
[![Build Status](https://travis-ci.org/chalharu/rust-nearly-eq.svg?branch=master)](https://travis-ci.org/chalharu/rust-nearly-eq)
[![docs.rs](https://docs.rs/nearly_eq/badge.svg)](https://docs.rs/nearly_eq)
[![Coverage Status](https://coveralls.io/repos/github/chalharu/rust-nearly-eq/badge.svg?branch=master)](https://coveralls.io/github/chalharu/rust-nearly-eq?branch=master)

Implementing the `NearlyEq` traits, Can asserts that the two expressions are nearly(approximately) equal to each other.

## How-to Use
See the [crate documentation](https://docs.rs/nearly_eq/) for more details.

### Examples

```rust
assert_nearly_eq!(1f64, 1.5f64, 0.6f64); // does not panic
assert_nearly_eq!(0f64, 1e-12f64); // does not panic
```

```rust:should_panic
assert_nearly_eq!(1f64, 2f64); // panics
```

### Optional Features

- **`complex`** - Implement `NearlyEq` traits for `num_complex::Complex`. This adds a dependency on the `num-complex` crate.

- **`rational`** - Implement `NearlyEq` traits for `num_rational::Ratio`. This adds a dependency on the `num-rational` crate.

- **`fixed-point`** - Implement `NearlyEq` traits for fixed-point types of ['fpa'](https://crates.io/crates/fpa) crate.

- **`i128`** - Implement `NearlyEq` traits for `i128` and `u128`. **Available only on Rust nightly channel.**
