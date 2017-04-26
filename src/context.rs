use llvm_sys::prelude::*;
use llvm_sys::core as llvm;

use super::*;


// LLVM Wrappers

pub struct Context {
    pub ptr: LLVMContextRef
}

impl Context {
    pub fn new() -> Self {
        let context = unsafe {
            llvm::LLVMContextCreate()
        };
        Context { ptr: context }
    }

    pub fn create_builder(&self) -> Builder {
        let builder = unsafe {
            llvm::LLVMCreateBuilderInContext(self.ptr)
        };
        Builder { ptr: builder }
    }

    pub fn module_create_with_name(&self, name: &str) -> Module {
        let c_name = CString::new(name).unwrap();
        let module = unsafe {
            llvm::LLVMModuleCreateWithNameInContext(c_name.as_ptr(), self.ptr)
        };
        Module { ptr: module }
    }

    pub fn void_type(&self) -> LLVMTypeRef {
        unsafe {
            llvm::LLVMVoidTypeInContext(self.ptr)
        }
    }

    pub fn int64_type(&self) -> LLVMTypeRef {
        unsafe {
            llvm::LLVMInt64TypeInContext(self.ptr)
        }
    }

    pub fn int32_type(&self) -> LLVMTypeRef {
        unsafe {
            llvm::LLVMInt32TypeInContext(self.ptr)
        }
    }

    pub fn int16_type(&self) -> LLVMTypeRef {
        unsafe {
            llvm::LLVMInt16TypeInContext(self.ptr)
        }
    }

    pub fn int8_type(&self) -> LLVMTypeRef {
        unsafe {
            llvm::LLVMInt8TypeInContext(self.ptr)
        }
    }

    pub fn int1_type(&self) -> LLVMTypeRef {
        unsafe {
            llvm::LLVMInt1TypeInContext(self.ptr)
        }
    }

    pub fn const_bool(&self, val: bool) -> LLVMValueRef {
        let ty = self.int1_type();
        const_int(ty, val as u64, false)
    }

    pub fn append_basic_block(&self, func: Function, name: &str) -> LLVMBasicBlockRef {
        let c_name = CString::new(name).unwrap();
        unsafe {
            llvm::LLVMAppendBasicBlockInContext(self.ptr, func.ptr, c_name.as_ptr())
        }
    }
}

impl Drop for Context {
    fn drop(&mut self) {
        unsafe {
            llvm::LLVMContextDispose(self.ptr);
        }
    }
}


