use std::fmt;

use libc;

use llvm_sys::prelude::*;
use llvm_sys::core as llvm;

use super::*;

use std::path::Path;


// No `Drop` impl is needed as this is disposed of when the associated context is disposed
// TODO Documentation
#[derive(Debug)]
pub struct Module {
    pub ptr: LLVMModuleRef
}
map_to_llvm!(Module, LLVMModuleRef);

impl Module {
    // TODO Documentation
    pub fn dump(&self) {
        unsafe {
            llvm::LLVMDumpModule(self.ptr)
        }
    }

    // TODO Documentation
    pub fn add_function(&mut self, func_ty: &types::Function, name: &str) -> Function {
        let c_name = CString::new(name).unwrap();
        let p = unsafe {
            llvm::LLVMAddFunction(self.ptr, c_name.as_ptr(), func_ty.into())
        };
        Function {
            ptr: p
        }
    }

    // TODO Documentation
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

    /// Prints a module to a file
    ///
    /// ```rust
    /// use llvm::Context;
    ///
    /// let context = Context::global();
    /// let module = context.module_create_with_name("name");
    /// let path = "./module_file";
    ///
    /// module.print_to_file(path).unwrap();
    ///
    /// assert!(std::path::Path::new(path).exists());
    /// std::fs::remove_file(path).unwrap()
    /// ```
    pub fn print_to_file<P: AsRef<Path>>(&self, path: P) -> Result<(), anyhow::Error> {
        let str_path = path.as_ref().to_str().expect("Failed to convert path to unicode");
        let c_path = CString::new(str_path).unwrap();
        let mut em: usize = 0;
        let em_ptr: *mut usize = &mut em;
        unsafe {
            llvm::LLVMPrintModuleToFile(self.ptr, c_path.as_ptr(), em_ptr as *mut *mut i8);
            if em == 0 { // no error message was set
                Ok(())
            } else {
                Err(anyhow::anyhow!(c_str_to_str!(em as *const i8).to_string()))
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
                len + 1,
                len + 1
            );
            write!(f, "{}", s)
        }
    }
}
