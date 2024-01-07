
use llvm_sys::{prelude::*, core::*};
use derive_more::{Deref, DerefMut};


// TODO Documentation
#[derive(Debug, Deref, DerefMut)]
pub struct Function {
    pub ptr: LLVMValueRef,
}

impl Function {
    // TODO Documentation
    pub fn from_value_ref(ptr: LLVMValueRef) -> Function {
        Function { ptr }
    }

    // TODO Documentation
    pub fn params(&self) -> FunctionParamIter {
        FunctionParamIter {
            arg: self.ptr,
            first: true,
        }
    }

    // TODO: Check if there is an optimization so that we could
    // call func.params().nth(1) and call this function
    // TODO Documentation
    pub fn get_param(&self, index: u32) -> Option<LLVMValueRef> {
        let param = unsafe {
            LLVMGetParam(self.ptr, index)
        };

        if param.is_null() {
            return None;
        } else  {
            return Some(param);
        }
    }

    // TODO Documentation
    pub fn count_basic_blocks(&self) -> u32 {
        unsafe {
            LLVMCountBasicBlocks(self.ptr)
        }
    }
}


// TODO Documentation
#[derive(Debug)]
pub struct FunctionParamIter {
    arg: LLVMValueRef,
    first: bool,
}

// TODO: Needs testing
// TODO Documentation
impl Iterator for FunctionParamIter {
    type Item = LLVMValueRef;

    fn next(&mut self) -> Option<LLVMValueRef> {
        self.arg = if self.first {
            unsafe { LLVMGetFirstParam(self.arg) }
        } else {
            unsafe { LLVMGetNextParam(self.arg) }
        };

        if self.arg.is_null() {
            None
        } else {
            Some(self.arg)
        }
    }
}
