# LLVM Rust bindings
[![crates-badge][]][crates] [![coveralls-badge][]][coveralls] [![license-badge][]][license]

This library is intended to be a safe wrapper around the [llvm-sys bindings](https://crates.io/crates/llvm-sys).
It is currently very incomplete however it is possible to use this and `llvm-sys` at the same time, most structures
are able to be converted into `LLVM*Ref`

If you find that you need a functionality thats only present in `llvm-sys` please file an issue.

A simple JIT example is available [here](examples/jit.rs).

## Safety

While it is better to use this library over `llvm-sys` directly, this library is still not completely safe. Some functions still return `LLVM*Ref` types, which are type aliases for raw pointers. Until I finish converting these raw pointers into safe wrapper types, there is still a possibility for unsafe behavior, although in practice this is rare.



[crates-badge]: https://img.shields.io/crates/v/llvm.svg?style=flat-square
[travis-badge]: https://img.shields.io/travis/gsingh93/llvm/master.svg?style=flat-square
[appveyor-badge]: https://img.shields.io/appveyor/ci/afonso360/llvm/master.svg?style=flat-square
[coveralls-badge]: https://img.shields.io/coveralls/gsingh93/llvm/master.svg?style=flat-square
[license-badge]: https://img.shields.io/github/license/gsingh93/llvm.svg?style=flat-square
[gitter-badge]: https://img.shields.io/gitter/room/gsingh93/llvm.svg?style=flat-square
[crates]: https://crates.io/crates/llvm
[travis]: https://travis-ci.org/gsingh93/llvm
[appveyor]: https://ci.appveyor.com/project/afonso360/llvm
[coveralls]: https://coveralls.io/github/gsingh93/llvm
[license]: LICENSE
[gitter]: https://gitter.im/llvm-rs
