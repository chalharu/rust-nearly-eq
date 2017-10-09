# rust-nearly-eq
[![crates.io badge](https://img.shields.io/crates/v/nearly-eq.svg)](https://crates.io/crates/nearly-eq)
[![Build Status](https://travis-ci.org/chalharu/rust-nearly-eq.svg)](https://travis-ci.org/chalharu/rust-nearly-eq)
[![docs.rs](https://docs.rs/nearly_eq/badge.svg)](https://docs.rs/nearly_eq)
[![Coverage Status](https://coveralls.io/repos/github/chalharu/rust-nearly-eq/badge.svg)](https://coveralls.io/github/chalharu/rust-nearly-eq)

rust crate: nearly equal


## If you are using the num-complex crate
To depend on the `nearly_eq` crate with this feature enabled, put the following in
your project's `Cargo.toml` file:

```toml
[dependencies.nearly_eq]
features = ["num-complex"]
version = ...  # Whichever version you are using
```