use llvm_sys::prelude::*;
use llvm_sys::core::*;
use llvm_sys::*;

use super::*;
use types::ContextType;

pub struct Value(LLVMValue); // TODO: mark this as an unsized type
impl_llvm_type_wrapper!(LLVMValueRef, Value);
impl_llvm_type_eq!(LLVMValueRef, Value);
impl_llvm_type_fmt!(Value, LLVMPrintValueToString);

impl Value {
    pub fn set_name(&mut self, name: &str) {
        let c_name = CString::new(name).unwrap();
        unsafe {
            LLVMSetValueName(self.into(), c_name.as_ptr());
        }
    }

    // TODO: This &mut is unecessary
    pub fn name(&mut self) -> String {
        unsafe {
            let c_str = LLVMGetValueName(self.into());
            let len = libc::strlen(c_str);
            String::from_raw_parts(c_str as *mut u8, len + 1, len + 1)
        }
    }
}


#[cfg(test)]
mod tests {
    use super::Context;
    use value::{Value, IntoConstValue};

    #[test]
    fn can_set_and_get_value_name() {
        let value_name = "test_value_name";
        let context = Context::default();
        let value: &mut Value = 30u32.gen_const(&context).into();

        value.set_name(value_name);
        assert_eq!(value.name(), value_name);
    }
}




/// Represents a type that can be inserted as a const in a context
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
                    LLVMConstInt($t::get_type_in_context(context).into(), self.into(), 0)
                }
            }
        }
    };
    (INT: $t: tt) => {
        impl IntoConstValue for $t {
            fn gen_const(self, context: &Context) -> LLVMValueRef {
                unsafe {
                    LLVMConstInt($t::get_type_in_context(context).into(), self as u64, 1)
                }
            }
        }
    };
    (FLOAT: $t: tt) => {
        impl IntoConstValue for $t {
            fn gen_const(self, context: &Context) -> LLVMValueRef {
                unsafe {
                    LLVMConstReal($t::get_type_in_context(context).into(), self.into())
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
