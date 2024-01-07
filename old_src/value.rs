
use llvm_sys::{*, core::*, prelude::*};
use derive_more::{Deref, DerefMut};

use super::{*, types::ContextType};

// TODO Documentation
#[derive(Deref, DerefMut)]
pub struct Value(LLVMValue); // TODO: mark this as an unsized type
    impl_from!(LLVMValueRef, Value);
    impl_eq!(LLVMValueRef, Value);
    impl_fmt!(Value, LLVMPrintValueToString);

impl Value {
    // TODO Documentation
    // TODO Debug
    pub fn set_name(&mut self, name: &str) {
        let c_name = CString::new(name).unwrap();
        unsafe {
            LLVMSetValueName2(self.into(), c_name.as_ptr(), name.len().into());
        }
    }

    // TODO Documentation
    // TODO Debug
    pub fn name(&self) -> String {
        unsafe {
            let mut length: libc::size_t = 0;
            let name = LLVMGetValueName2(self.into(), &mut length);
            let slice = std::slice::from_raw_parts(name as *const u8, length as usize);
            String::from_utf8_lossy(slice).into_owned()
        }
    }
}

/// Represents a type that can be used as a const in a context
pub trait IntoConstValue: ContextType {
    fn gen_const(self, context: &Context) -> LLVMValueRef;
}

// TODO Documentation
// TODO Debug: May be easier to build fully, then simplify into macros
macro_rules! impl_const_value {
    (UINT: $t: ty) => {
        impl IntoConstValue for $t {
            fn gen_const(self, context: &Context) -> LLVMValueRef {
                unsafe {
                    let type_in_context = <$t as ContextType>::get_type_in_context(context);
                    LLVMConstInt(From::from(type_in_context), self.into(), 0)
                }
            }
        }
    };
    (INT: $t: ty) => {
        impl IntoConstValue for $t {
            fn gen_const(self, context: &Context) -> LLVMValueRef {
                unsafe {
                    LLVMConstInt(<$t>::get_type_in_context(context).into(), self as u64, 1)
                }
            }
        }
    };
    (FLOAT: $t: ty) => {
        impl IntoConstValue for $t {
            fn gen_const(self, context: &Context) -> LLVMValueRef {
                unsafe {
                    LLVMConstReal(<$t>::get_type_in_context(context).into(), self.into())
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


// TODO Reorganize tests
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
        assert_eq!(value.name(), value_name.to_string());
    }
}

