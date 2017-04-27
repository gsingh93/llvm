use llvm_sys::prelude::*;
use llvm_sys::core as llvm;

use super::*;

use value::IntoConstValue;

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

    pub fn append_basic_block(&self, func: &mut Function, name: &str) -> LLVMBasicBlockRef {
        let c_name = CString::new(name).unwrap();
        unsafe {
            llvm::LLVMAppendBasicBlockInContext(self.ptr, func.ptr, c_name.as_ptr())
        }
    }

    /// Creates a constant in this context
    /// The value must implement the trait `IntoValue`
    pub fn cons<T: IntoConstValue>(&self, val: T) -> LLVMValueRef {
        val.gen_const(self)
    }
}

impl Drop for Context {
    fn drop(&mut self) {
        unsafe {
            llvm::LLVMContextDispose(self.ptr);
        }
    }
}
