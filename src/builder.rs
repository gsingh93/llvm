use llvm_sys::*;
use llvm_sys::prelude::*;
use llvm_sys::core as llvm;

use super::*;

macro_rules! build_op_str {
    ($op_name: ident, $fn: path, $($argn: ident: $argv: path),*) => {
        impl Builder {
            pub fn $op_name(&mut self, $($argn: $argv),*, name: &str) -> LLVMValueRef {
                let c_name = CString::new(name).unwrap();
                unsafe {
                    $fn(self.ptr, $($argn),*, c_name.as_ptr())
                }
            }
        }
    }
}

macro_rules! build_op {
    ($op_name: ident, $fn: path, $($argn: ident: $argv: path),*) => {
        impl Builder {
            pub fn $op_name(&mut self, $($argn: $argv),*) -> LLVMValueRef {
                unsafe {
                    $fn(self.ptr, $($argn),*)
                }
            }
        }
    }
}

pub struct Builder {
    pub ptr: LLVMBuilderRef
}

build_op!(build_ret, llvm::LLVMBuildRet, ret_val: LLVMValueRef);
build_op!(build_ret_void, llvm::LLVMBuildRetVoid,); // TODO: Fix the trailing comma
build_op!(build_store, llvm::LLVMBuildStore, val: LLVMValueRef, ptr: LLVMValueRef);
build_op!(build_br, llvm::LLVMBuildBr, dest: LLVMBasicBlockRef);

build_op!(build_cond_br, llvm::LLVMBuildCondBr, cond: LLVMValueRef,
                                                then: LLVMBasicBlockRef,
                                                else_: LLVMBasicBlockRef);

build_op_str!(build_load, llvm::LLVMBuildLoad, ptr: LLVMValueRef);
build_op_str!(build_alloca, llvm::LLVMBuildAlloca, ty: LLVMTypeRef);

build_op_str!(build_add, llvm::LLVMBuildAdd, lhs: LLVMValueRef, rhs: LLVMValueRef);
build_op_str!(build_sub, llvm::LLVMBuildSub, lhs: LLVMValueRef, rhs: LLVMValueRef);
build_op_str!(build_mul, llvm::LLVMBuildMul, lhs: LLVMValueRef, rhs: LLVMValueRef);
build_op_str!(build_sdiv, llvm::LLVMBuildSDiv, lhs: LLVMValueRef, rhs: LLVMValueRef);

///Comment
build_op_str!(build_icmp, llvm::LLVMBuildICmp, op: LLVMIntPredicate,
                                               lhs: LLVMValueRef,
                                               rhs: LLVMValueRef);



impl Builder {
    pub fn position_at_end(&mut self, basic_block: LLVMBasicBlockRef) {
        unsafe {
            llvm::LLVMPositionBuilderAtEnd(self.ptr, basic_block);
        }
    }

    pub fn build_call(&mut self, func: Function, mut args: Vec<LLVMValueRef>,
                      name: &str) -> LLVMValueRef {
        let c_name = CString::new(name).unwrap();
        unsafe {
            llvm::LLVMBuildCall(
                self.ptr,
                func.ptr,
                args.as_mut_ptr(),
                args.len() as u32,
                c_name.as_ptr()
            )
        }
    }
    pub fn build_global_string(&self, s: &str, name: &str) -> LLVMValueRef {
        let c_s = CString::new(s).unwrap();
        let c_name = CString::new(name).unwrap();
        unsafe {
            llvm::LLVMBuildGlobalString(self.ptr, c_s.as_ptr(), c_name.as_ptr())
        }
    }

    pub fn build_in_bounds_gep(&self, ptr: LLVMValueRef, mut indices: Vec<LLVMValueRef>,
                               name: &str) -> LLVMValueRef {
        let c_name = CString::new(name).unwrap();
        unsafe {
            llvm::LLVMBuildInBoundsGEP(self.ptr, ptr, indices.as_mut_ptr(),
                                       indices.len() as u32, c_name.as_ptr())
        }
    }
}

impl Drop for Builder {
    fn drop(&mut self) {
        unsafe {
            llvm::LLVMDisposeBuilder(self.ptr);
        }
    }
}

