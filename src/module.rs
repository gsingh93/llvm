use std::fmt;

use libc;

use llvm_sys::prelude::*;
use llvm_sys::core as llvm;

use super::*;

// No `Drop` impl is needed as this is disposed of when the associated context is disposed
pub struct Module {
    pub ptr: LLVMModuleRef
}
impl_llvm_ref!(Module, LLVMModuleRef);

impl Module {
    pub fn dump(&self) {
        unsafe {
            llvm::LLVMDumpModule(self.ptr)
        }
    }

    pub fn add_function(&mut self, func_ty: LLVMTypeRef, name: &str) -> Function {
        let c_name = CString::new(name).unwrap();
        let p = unsafe {
            llvm::LLVMAddFunction(self.ptr, c_name.as_ptr(), func_ty)
        };
        Function {
            ptr: p
        }
    }

    pub fn get_named_function(&mut self, name: &str) -> Option<Function> {
        let c_name = CString::new(name).unwrap();
        let res = unsafe {
            llvm::LLVMGetNamedFunction(self.ptr, c_name.as_ptr())
        };

        if res.is_null() {
            None
        } else {
            Some(Function::from_value_ref(res))
        }
    }

    pub fn print_to_file(&self, path: &str) -> Result<(), &'static str> {
        let c_path = CString::new(path).unwrap();
        let mut em: usize = 0;
        let em_ptr: *mut usize = &mut em;
        unsafe {
            llvm::LLVMPrintModuleToFile(self.ptr, c_path.as_ptr(), em_ptr as *mut *mut i8);
            if em == 0 { // no error message was set
                Ok(())
            } else {
                Err(c_str_to_str!(em as *const i8))
            }
        }
    }
}

impl fmt::Display for Module {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        unsafe {
            let c_str = llvm::LLVMPrintModuleToString(self.ptr);
            let len = libc::strlen(c_str);
            let s = String::from_raw_parts(
                c_str as *mut u8,
                (len + 1) as usize,
                (len + 1) as usize
            );
            write!(f, "{}", s)
        }
    }
}
