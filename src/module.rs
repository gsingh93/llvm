
use std::ffi::{CString, CStr};

use llvm_sys::*;
use llvm_sys::prelude::*;
use llvm_sys::target_machine::*;
use llvm_sys::target::*;
use llvm_sys::core as llvm;

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

    pub fn add_function(&mut self, func_ty: LLVMTypeRef, name: &str) -> LLVMValueRef {
        let c_name = CString::new(name).unwrap();
        unsafe {
            llvm::LLVMAddFunction(self.module, c_name.as_ptr(), func_ty)
        }
    }

    pub fn get_named_function(&mut self, name: &str) -> Option<LLVMValueRef> {
        let c_name = CString::new(name).unwrap();
        let res = unsafe {
            llvm::LLVMGetNamedFunction(self.module, c_name.as_ptr())
        };

        if res.is_null() {
            None
        } else {
            Some(res)
        }
    }
}
