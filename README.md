# LLVM Rust bindings

This library is intended to be a safe wrapper around the [llvm_sys bindings](https://crates.io/crates/llvm_sys). It is currently incomplete, as only methods I use for my projects have been added. If there are any methods you would like added, please open an issue or directly make a pull request. Note that if you need a method from `llvm_sys` and don't want to update this library just to call it, the raw pointers for all of the types are stored in a public field, so you can still use any of the `llvm_sys` functions.

## Safety

While it is better to use this library over `llvm_sys` directly, this library is still not completely safe. Some functions still return `LLVM*Ref` types, which are type aliases for raw pointers. Until I finish converting these raw pointers into safe wrapper types, there is still a possibility for unsafe behavior, although in practice this is rare.
