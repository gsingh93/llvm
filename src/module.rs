use std::fmt;

use libc;
use std::ffi::{CString, CStr};

use llvm_sys::*;
use llvm_sys::prelude::*;
use llvm_sys::target_machine::*;
use llvm_sys::target::*;
use llvm_sys::core as llvm;

use super::*;

// No `Drop` impl is needed as this is disposed of when the associated context is disposed
pub struct Module {
    pub module: LLVMModuleRef
}

impl Module {
    pub fn dump(&self) {
        unsafe {
            llvm::LLVMDumpModule(self.module)
        }
    }

    pub fn add_function(&mut self, func_ty: LLVMTypeRef, name: &str) -> Function {
        let c_name = CString::new(name).unwrap();
        let p = unsafe {
            llvm::LLVMAddFunction(self.module, c_name.as_ptr(), func_ty)
        };
        Function {
            ptr: p
        }
    }

    pub fn get_named_function(&mut self, name: &str) -> Option<Function> {
        let c_name = CString::new(name).unwrap();
        let res = unsafe {
            llvm::LLVMGetNamedFunction(self.module, c_name.as_ptr())
        };

        if res.is_null() {
            None
        } else {
            Some(Function::from_value_ref(res))
        }
    }
}

impl fmt::Display for Module {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        unsafe {
            let c_str = llvm::LLVMPrintModuleToString(self.module);
            let len = libc::strlen(c_str);
            let s = String::from_raw_parts(
                c_str as *mut u8,
                (len + 1) as usize,
                (len + 1) as usize
            );
            write!(f, "{}", s)
        }
    }
}
