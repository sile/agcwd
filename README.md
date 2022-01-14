agcwd
=====

[![agcwd](https://img.shields.io/crates/v/agcwd.svg)](https://crates.io/crates/agcwd)
[![Documentation](https://docs.rs/agcwd/badge.svg)](https://docs.rs/agcwd)
[![Actions Status](https://github.com/sile/agcwd/workflows/CI/badge.svg)](https://github.com/sile/agcwd/actions)
[![Coverage Status](https://coveralls.io/repos/github/sile/agcwd/badge.svg?branch=main)](https://coveralls.io/github/sile/agcwd?branch=main)
![License](https://img.shields.io/crates/l/efmt)

A Rust implementation of the AGCWD algorithm.

AGCWD is described in the paper ["Efficient Contrast Enhancement Using Adaptive Gamma Correction With Weighting Distribution"][AGCWD].

[AGCWD]: https://ieeexplore.ieee.org/abstract/document/6336819/

Examples
--------

```rust
// An example image containing 2 RGB pixels.
let mut pixels = vec![0, 1, 2, 3, 4, 5];

let agcwd = agcwd::Agcwd::new(0.5);
agcwd.enhance_rgb_image(&mut pixels);
```
