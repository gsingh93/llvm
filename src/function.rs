use llvm_sys::prelude::*;
use llvm_sys::core::*;

#[derive(Debug)]
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

    // TODO: Check if there is an optimization so that we could
    // call func.params().nth(1) and call this function
    pub fn get_param(&self, index: u32) -> Option<LLVMValueRef> {
        let p = unsafe {
            LLVMGetParam(self.ptr, index)
        };

        if p.is_null() {
            return None;
        } else  {
            return Some(p);
        }
    }

    pub fn count_basic_blocks(&self) -> u32 {
        unsafe {
            LLVMCountBasicBlocks(self.ptr)
        }
    }
}


#[derive(Debug)]
pub struct FunctionParamIter {
    arg: LLVMValueRef,
    first: bool,
}

// TODO: Needs testing
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
