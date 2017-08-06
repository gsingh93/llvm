#![doc(test(attr(allow(unused_variables), deny(warnings))))]
#![allow(unused_doc_comment)]
#![deny(warnings,
        missing_debug_implementations,
        trivial_numeric_casts,
        unused_import_braces,
        unused_qualifications)]


extern crate libc;
extern crate llvm_sys;

#[macro_use]
extern crate error_chain;

use std::ffi::{CString, CStr};

use llvm_sys::prelude::*;
use llvm_sys::core as llvm;

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

pub fn set_value_name(val: LLVMValueRef, name: &str) {
    let c_name = CString::new(name).unwrap();
    unsafe {
        llvm::LLVMSetValueName(val, c_name.as_ptr());
    }
}
