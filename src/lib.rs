extern crate libc;
extern crate llvm_sys;

use std::ffi::{CString, CStr};

use llvm_sys::*;
use llvm_sys::prelude::*;
use llvm_sys::target_machine::*;
use llvm_sys::target::*;
use llvm_sys::core as llvm;

// This should only be used for static strings
macro_rules! c_str_to_str {
    ($s:expr) => {
        ::std::str::from_utf8(CStr::from_ptr($s).to_bytes()).unwrap()
    }
}

mod context;
mod types;
mod builder;
mod module;

pub use context::*;
pub use types::*;
pub use builder::*;
pub use module::*;

pub fn function_get_param(func: LLVMValueRef, index: u32) -> Option<LLVMValueRef> {
    let p = unsafe {
        llvm::LLVMGetParam(func, index)
    };

    if p.is_null() {
        return None;
    } else  {
        return Some(p);
    }
}

pub fn get_first_param(func: LLVMValueRef) -> Option<LLVMValueRef> {
    let res = unsafe {
        llvm::LLVMGetFirstParam(func)
    };
    // TODO: We should replace this API with a safe iterator in the future
    if res.is_null() {
        None
    } else {
        Some(res)
    }
}

pub fn get_next_param(param: LLVMValueRef) -> Option<LLVMValueRef> {
    let res = unsafe {
        llvm::LLVMGetNextParam(param)
    };
    // TODO: We should replace this API with a safe iterator in the future
    if res.is_null() {
        None
    } else {
        Some(res)
    }
}

pub fn set_value_name(val: LLVMValueRef, name: &str) {
    let c_name = CString::new(name).unwrap();
    unsafe {
        llvm::LLVMSetValueName(val, c_name.as_ptr());
    }
}

pub fn create_pass_manager() -> LLVMPassManagerRef {
    unsafe {
        llvm::LLVMCreatePassManager()
    }
}

pub fn print_module_to_file(module: &Module, path: &str) -> Result<(), &'static str> {
    let c_path = CString::new(path).unwrap();
    let mut em: usize = 0;
    let em_ptr: *mut usize = &mut em;
    unsafe {
        llvm::LLVMPrintModuleToFile(module.module, c_path.as_ptr(), em_ptr as *mut *mut i8);
        if em == 0 { // no error message was set
            Ok(())
        } else {
            Err(c_str_to_str!(em as *const i8))
        }
    }
}

pub fn print_module_to_string<'a>(module: &'a Module) -> String {
    unsafe {
        let c_str = llvm::LLVMPrintModuleToString(module.module);
        let len = libc::strlen(c_str);
        String::from_raw_parts(c_str as *mut u8, (len + 1) as usize, (len + 1) as usize)
    }
}

pub fn get_target_from_name(name: &str) -> Option<LLVMTargetRef> {
    let c_name = CString::new(name).unwrap();
    let res = unsafe {
        LLVMGetTargetFromName(c_name.as_ptr())
    };

    if res.is_null() {
        None
    } else {
        Some(res)
    }
}

pub fn create_target_machine(target: LLVMTargetRef, triple: &str, cpu: &str, features: &str,
                             level: LLVMCodeGenOptLevel, reloc: LLVMRelocMode,
                             model: LLVMCodeModel) -> LLVMTargetMachineRef {
    let c_triple = CString::new(triple).unwrap();
    let c_cpu = CString::new(cpu).unwrap();
    let c_features = CString::new(features).unwrap();
    unsafe {
        LLVMCreateTargetMachine(target, c_triple.as_ptr(), c_cpu.as_ptr(),
                                c_features.as_ptr(), level, reloc, model)
    }
}

pub fn target_machine_emit_to_file(target: LLVMTargetMachineRef, module: &mut Module, path: &str,
                                   file_type: LLVMCodeGenFileType) -> Result<(), &'static str> {
    let c_path = CString::new(path).unwrap();
    let mut em: usize = 0;
    let em_ptr: *mut usize = &mut em;
    unsafe {
        LLVMTargetMachineEmitToFile(target, module.module, c_path.as_ptr() as *mut i8,
                                    file_type, em_ptr as *mut *mut i8);
        if em == 0 { // no error message was set
            Ok(())
        } else {
            Err(c_str_to_str!(em as *const i8))
        }
    }
}

pub fn initialize_native_target() {
    unsafe {
        LLVM_InitializeNativeTarget();
    }
}

pub fn initialize_native_asm_printer() {
    unsafe {
        LLVM_InitializeNativeAsmPrinter();
    }
}

pub fn get_default_target_triple<'a>() -> &'a str {
    unsafe {
        c_str_to_str!(LLVMGetDefaultTargetTriple())
    }
}
