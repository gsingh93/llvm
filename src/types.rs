use llvm_sys::prelude::*;
use llvm_sys::core::*;

use super::*;

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

pub trait Type: ContextType /* + GlobalType*/{}

/// Represents a LLVM Context Type
pub trait ContextType {
    fn get_type_in_context(context: &Context) -> LLVMTypeRef;
}

macro_rules! impl_context_type {
    ($t: ty, $to_type_in_context: ident) => {
        impl ContextType for $t {
            fn get_type_in_context(context: &Context) -> LLVMTypeRef {
                unsafe {
                    $to_type_in_context(context.ptr)
                }
            }
        }
    }
}


impl_context_type!(bool, LLVMInt1TypeInContext);
// This might actually not be true, Not sure
impl_context_type!(char, LLVMInt8TypeInContext);
impl_context_type!(u8, LLVMInt8TypeInContext);
impl_context_type!(u16, LLVMInt16TypeInContext);
impl_context_type!(u32, LLVMInt32TypeInContext);
impl_context_type!(u64, LLVMInt64TypeInContext);
impl_context_type!(i8, LLVMInt8TypeInContext);
impl_context_type!(i16, LLVMInt16TypeInContext);
impl_context_type!(i32, LLVMInt32TypeInContext);
impl_context_type!(i64, LLVMInt64TypeInContext);
impl_context_type!(f32, LLVMFloatTypeInContext);
impl_context_type!(f64, LLVMDoubleTypeInContext);
//TODO: Function Types
//TODO: Structure Types
//TODO: Sequential Types
//TODO: Other Types
