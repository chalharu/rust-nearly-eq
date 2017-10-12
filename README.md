# rust-nearly-eq
[![crates.io badge](https://img.shields.io/crates/v/nearly-eq.svg)](https://crates.io/crates/nearly-eq)
[![Build Status](https://travis-ci.org/chalharu/rust-nearly-eq.svg)](https://travis-ci.org/chalharu/rust-nearly-eq)
[![docs.rs](https://docs.rs/nearly_eq/badge.svg)](https://docs.rs/nearly_eq)
[![Coverage Status](https://coveralls.io/repos/github/chalharu/rust-nearly-eq/badge.svg)](https://coveralls.io/github/chalharu/rust-nearly-eq)

rust crate: nearly equal

## How-to Use
See the [crate documentation](https://docs.rs/nearly_eq/) for more details.

### Optional Features

- **`num-complex`** - Implement `NearlyEq` traits for `num_complex::Complex`. This adds a dependency on the `num-complex` crate.

- **`ndarray`** - Implement `NearlyEq` traits for `ndarray::ArrayBase`. This adds a dependency on the `ndarray` crate.

- **`i128`** - Implement `NearlyEq` traits for `i128` and `u128`. **Available only on Rust nightly channel.**
