use llvm_sys::prelude::*;
use llvm_sys::core as llvm;

pub fn const_int(ty: LLVMTypeRef, val: u64, signed: bool) -> LLVMValueRef {
    unsafe {
        llvm::LLVMConstInt(ty, val, signed as i32)
    }
}

pub fn function_type(ret_ty: LLVMTypeRef,
                     mut param_types: Vec<LLVMTypeRef>,
                     is_var_args: bool) -> LLVMTypeRef {
    unsafe {
        llvm::LLVMFunctionType(ret_ty,
                               param_types.as_mut_ptr(),
                               param_types.len() as u32,
                               is_var_args as i32)
    }
}

pub fn pointer_type(ty: LLVMTypeRef, address_space: u32) -> LLVMTypeRef {
    unsafe {
        llvm::LLVMPointerType(ty, address_space)
    }
}
