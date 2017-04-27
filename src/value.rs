use llvm_sys::prelude::*;
use llvm_sys::core as llvm;
use super::*;

/// Represnts a type that can be inserted as a const in a context
pub trait IntoConstValue: ContextType {
    fn gen_const(self, context: &Context) -> LLVMValueRef;
}

macro_rules! impl_const_value {
    // TODO: this `tt` should be a `ty`, for some reason this causes
    // the macro to fail, we should file a bug report
    (UINT: $t: tt) => {
        impl IntoConstValue for $t {
            fn gen_const(self, context: &Context) -> LLVMValueRef {
                unsafe {
                    llvm::LLVMConstInt($t::get_type_in_context(context), self.into(), 0)
                }
            }
        }
    };
    (INT: $t: tt) => {
        impl IntoConstValue for $t {
            fn gen_const(self, context: &Context) -> LLVMValueRef {
                unsafe {
                    llvm::LLVMConstInt($t::get_type_in_context(context), self as u64, 1)
                }
            }
        }
    };
    (FLOAT: $t: tt) => {
        impl IntoConstValue for $t {
            fn gen_const(self, context: &Context) -> LLVMValueRef {
                unsafe {
                    llvm::LLVMConstReal($t::get_type_in_context(context), self.into())
                }
            }
        }
    }
}

//TODO: Missing OfString and OfStringAndSize variants
impl_const_value!(FLOAT: f64);
impl_const_value!(FLOAT: f32);
impl_const_value!(UINT: u8);
impl_const_value!(UINT: u16);
impl_const_value!(UINT: u32);
impl_const_value!(UINT: u64);
impl_const_value!(INT: i8);
impl_const_value!(INT: i16);
impl_const_value!(INT: i32);
impl_const_value!(INT: i64);
