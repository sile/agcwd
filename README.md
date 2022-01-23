agcwd
=====

[![agcwd](https://img.shields.io/crates/v/agcwd.svg)](https://crates.io/crates/agcwd)
[![Documentation](https://docs.rs/agcwd/badge.svg)](https://docs.rs/agcwd)
[![Actions Status](https://github.com/sile/agcwd/workflows/CI/badge.svg)](https://github.com/sile/agcwd/actions)
[![Coverage Status](https://coveralls.io/repos/github/sile/agcwd/badge.svg?branch=main)](https://coveralls.io/github/sile/agcwd?branch=main)
![License](https://img.shields.io/crates/l/efmt)

A Rust implementation of the AGCWD algorithm.

AGCWD is described in the paper ["Efficient Contrast Enhancement Using Adaptive Gamma Correction With Weighting Distribution"][AGCWD].

[Here](https://sile.github.io/agcwd/examples/enhance.html) is a live demo of enhancing images from your camera in real-time.

[AGCWD]: https://ieeexplore.ieee.org/abstract/document/6336819/

Examples
--------

A Rust code snippet to enhance an RGB image:
```rust
// An example image containing 2 RGB pixels.
let mut pixels = vec![0, 1, 2, 3, 4, 5];

let agcwd = agcwd::Agcwd::new();
agcwd.enhance_rgb_image(&mut pixels);
```

You can apply AGCWD to a PNG image by executing the following command:
```console
$ cargo run --example enhance-png /path/to/image.png --output-path 
```
