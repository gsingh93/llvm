
use std::ffi::{CString, CStr};

use llvm_sys::*;
use llvm_sys::prelude::*;
use llvm_sys::target_machine::*;
use llvm_sys::target::*;
use llvm_sys::core as llvm;

pub struct Function {
    pub ptr: LLVMValueRef,
}

impl Function {
    pub fn from_value_ref(p: LLVMValueRef) -> Function {
        Function {
            ptr: p
        }
    }

    pub fn params(&self) -> FunctionParamIter {
        FunctionParamIter {
            arg: self.ptr,
            first: true,
        }
    }

    pub fn get_param(&self, index: u32) -> Option<LLVMValueRef> {
        let p = unsafe {
            llvm::LLVMGetParam(self.ptr, index)
        };

        if p.is_null() {
            return None;
        } else  {
            return Some(p);
        }
    }

}


pub struct FunctionParamIter {
    arg: LLVMValueRef,
    first: bool,
}

// TODO: Needs testing
impl Iterator for FunctionParamIter {
    type Item = LLVMValueRef;

    fn next(&mut self) -> Option<LLVMValueRef> {
        let res = if self.first {
            unsafe { llvm::LLVMGetFirstParam(self.arg) }
        } else {
            unsafe { llvm::LLVMGetNextParam(self.arg) }
        };

        if res.is_null() {
            None
        } else {
            Some(res)
        }
    }
}
