use llvm_sys::prelude::*;
use llvm_sys::core as llvm;

use super::*;

use value::IntoConstValue;

// LLVM Wrappers

#[derive(Debug)]
pub struct Context {
    pub ptr: LLVMContextRef
}
impl_llvm_ref!(Context, LLVMContextRef);

impl Context {
    pub fn new() -> Self {
        let context = unsafe {
            llvm::LLVMContextCreate()
        };
        Context { ptr: context }
    }
    pub fn global() -> Self {
        unsafe {
            Context {
                ptr: llvm::LLVMGetGlobalContext()
            }
        }
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

    pub fn void_type<'a>(&'a self) -> &'a types::Void {
        unsafe { llvm::LLVMVoidTypeInContext(self.ptr).into() }
    }

    pub fn i1_type(&self) -> &types::Integer {
        unsafe { llvm::LLVMInt1TypeInContext(self.ptr).into() }
    }

    pub fn i8_type(&self) -> &types::Integer {
        unsafe { llvm::LLVMInt8TypeInContext(self.ptr).into() }
    }

    pub fn i16_type(&self) -> &types::Integer {
        unsafe { llvm::LLVMInt16TypeInContext(self.ptr).into() }
    }

    pub fn i32_type(&self) -> &types::Integer {
        unsafe { llvm::LLVMInt32TypeInContext(self.ptr).into() }
    }

    pub fn i64_type(&self) -> &types::Integer {
        unsafe { llvm::LLVMInt64TypeInContext(self.ptr).into() }
    }

    pub fn i128_type(&self) -> &types::Integer {
        unsafe { llvm::LLVMInt128TypeInContext(self.ptr).into() }
    }

    pub fn integer_type(&self, num_bits: u32) -> &types::Integer {
        unsafe { llvm::LLVMIntTypeInContext(self.ptr, num_bits).into() }
    }

    pub fn append_basic_block(&self, func: &Function, name: &str) -> LLVMBasicBlockRef {
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

impl Default for Context {
    /// Returns the global context
    fn default() -> Self {
        Context::global()
    }
}
