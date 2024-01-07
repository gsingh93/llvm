

extern crate libc;
extern crate llvm_sys;
extern crate anyhow;


use std::ffi::{CString, CStr};

// This should only be used for static strings
macro_rules! c_str_to_str {
    ($s:expr) => {
        ::std::str::from_utf8(CStr::from_ptr($s).to_bytes()).unwrap()
    }
}

#[macro_use]
mod macros;
mod context;
pub mod types;
mod builder;
mod module;
mod function;
mod pass_manager;
mod target;
mod execution_engine;
mod value;

// TODO: This was to maintain compatiblity, we should remove this
pub use context::*;
pub use types::{Type, ContextType};
pub use builder::*;
pub use module::*;
pub use function::*;
pub use pass_manager::*;
pub use target::*;
pub use execution_engine::*;
pub use value::*;
