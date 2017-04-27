use llvm_sys::*;
use llvm_sys::prelude::*;
use llvm_sys::core as llvm;

use super::*;

pub struct Builder {
    pub ptr: LLVMBuilderRef
}
impl_llvm_ref!(Builder, LLVMBuilderRef);

impl Builder {
    pub fn position_at_end(&mut self, basic_block: LLVMBasicBlockRef) {
        unsafe {
            llvm::LLVMPositionBuilderAtEnd(self.ptr, basic_block);
        }
    }

    pub fn build_ret(&mut self, ret_val: LLVMValueRef) -> LLVMValueRef {
        unsafe {
            llvm::LLVMBuildRet(self.ptr, ret_val)
        }
    }

    pub fn build_ret_void(&self) -> LLVMValueRef {
        unsafe {
            llvm::LLVMBuildRetVoid(self.ptr)
        }
    }

    pub fn build_alloca(&mut self, ty: LLVMTypeRef, name: &str) -> LLVMValueRef {
        let c_name = CString::new(name).unwrap();
        unsafe {
            llvm::LLVMBuildAlloca(self.ptr, ty, c_name.as_ptr())
        }
    }

    pub fn build_store(&mut self, val: LLVMValueRef, ptr: LLVMValueRef) -> LLVMValueRef {
        unsafe {
            llvm::LLVMBuildStore(self.ptr, val, ptr)
        }
    }

    pub fn build_load(&mut self, ptr: LLVMValueRef, name: &str) -> LLVMValueRef {
        let c_name = CString::new(name).unwrap();
        unsafe {
            llvm::LLVMBuildLoad(self.ptr, ptr, c_name.as_ptr())
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

    pub fn build_add(&mut self, lhs: LLVMValueRef, rhs: LLVMValueRef, name: &str) -> LLVMValueRef {
        let c_name = CString::new(name).unwrap();
        unsafe {
            llvm::LLVMBuildAdd(self.ptr, lhs, rhs, c_name.as_ptr())
        }
    }

    pub fn build_sub(&mut self, lhs: LLVMValueRef, rhs: LLVMValueRef, name: &str) -> LLVMValueRef {
        let c_name = CString::new(name).unwrap();
        unsafe {
            llvm::LLVMBuildSub(self.ptr, lhs, rhs, c_name.as_ptr())
        }
    }

    pub fn build_mul(&mut self, lhs: LLVMValueRef, rhs: LLVMValueRef, name: &str) -> LLVMValueRef {
        let c_name = CString::new(name).unwrap();
        unsafe {
            llvm::LLVMBuildMul(self.ptr, lhs, rhs, c_name.as_ptr())
        }
    }

    pub fn build_sdiv(&mut self, lhs: LLVMValueRef, rhs: LLVMValueRef,
                      name: &str) -> LLVMValueRef {
        let c_name = CString::new(name).unwrap();
        unsafe {
            llvm::LLVMBuildSDiv(self.ptr, lhs, rhs, c_name.as_ptr())
        }
    }

    pub fn build_icmp(&mut self, op: LLVMIntPredicate, lhs: LLVMValueRef, rhs: LLVMValueRef,
                      name: &str) -> LLVMValueRef {
        let c_name = CString::new(name).unwrap();
        unsafe {
            llvm::LLVMBuildICmp(self.ptr, op, lhs, rhs, c_name.as_ptr())
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

    pub fn build_cond_br(&self, cond: LLVMValueRef, then: LLVMBasicBlockRef,
                         else_: LLVMBasicBlockRef) -> LLVMValueRef {
        unsafe {
            llvm::LLVMBuildCondBr(self.ptr, cond, then, else_)
        }
    }

    pub fn build_br(&self, dest: LLVMBasicBlockRef) -> LLVMValueRef {
        unsafe {
            llvm::LLVMBuildBr(self.ptr, dest)
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

