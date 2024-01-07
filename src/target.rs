
use std::ffi::{CString, CStr};
use llvm_sys::target_machine::*;
use llvm_sys::target::*;
use super::*;

//use anyhow::anyhow;


/// TODO Documentation
#[derive(Debug)]
pub struct Target {
    ptr: LLVMTargetRef,
}
map_to_llvm!(Target, LLVMTargetRef);

impl Target {
    /// TODO Documentation
    pub fn from_name(name: &str) -> Option<Target> {
        let c_name = CString::new(name).unwrap();
        let result = unsafe {
            LLVMGetTargetFromName(c_name.as_ptr())
        };

        if result.is_null() {
            None
        } else {
            Some(Target {
                ptr: result
            })
        }
    }

    /// TODO Documentation
    pub fn create_target_machine(
        &self,
        triple: &str,
        cpu: &str,
        features: &str,
        level: LLVMCodeGenOptLevel,
        reloc: LLVMRelocMode,
        model: LLVMCodeModel
    ) -> TargetMachine {
        TargetMachine::new(self, triple, cpu, features, level, reloc, model)
    }
}

/// TODO Documentation
#[derive(Debug)]
pub struct TargetMachine {
    ptr: LLVMTargetMachineRef,
}

impl TargetMachine {

    /// TODO Documentation
    pub fn new(
        // TODO? Restrictions or wrappers for these args?
        target: &Target,
        triple: &str,
        cpu: &str,
        features: &str,
        // TODO Specific wrappers for these types
        level: LLVMCodeGenOptLevel, // https://docs.rs/llvm-sys/latest/llvm_sys/target_machine/enum.LLVMCodeGenOptLevel.html
        reloc: LLVMRelocMode, // https://docs.rs/llvm-sys/latest/llvm_sys/target_machine/enum.LLVMRelocMode.html
        model: LLVMCodeModel // https://docs.rs/llvm-sys/latest/llvm_sys/target_machine/enum.LLVMCodeModel.html
    ) -> TargetMachine {

        let c_triple = CString::new(triple).unwrap();
        let c_cpu = CString::new(cpu).unwrap();
        let c_features = CString::new(features).unwrap();
        let ptr = unsafe {
            LLVMCreateTargetMachine(
                target.ptr,
                c_triple.as_ptr(),
                c_cpu.as_ptr(),
                c_features.as_ptr(),
                level,
                reloc,
                model
            )
        };

        TargetMachine { ptr }
    }

    /// TODO Documentation
    pub fn emit_to_file(
        &mut self,
        module: &mut Module,
        path: &str,
        file_type: LLVMCodeGenFileType
    ) -> Result<(), anyhow::Error> {

        let c_path = CString::new(path).unwrap();
        let mut err_msg: usize = 0;
        let em_ptr: *mut usize = &mut err_msg;
        unsafe {
            LLVMTargetMachineEmitToFile(
                self.ptr,
                module.ptr,
                c_path.as_ptr() as *mut i8,
                file_type,
                em_ptr as *mut *mut i8
            );
            if err_msg == 0 { // No error message was sent
                Ok(())
            } else {
                Err(anyhow::anyhow!(c_str_to_str!(err_msg as *const i8).to_string()))
                // TODO Develop errors
            }
        }
    }
}

/// TODO Documentation
pub fn get_default_target_triple<'a>() -> &'a str {
    unsafe {
        c_str_to_str!(LLVMGetDefaultTargetTriple())
    }
}

/// TODO Documentation
pub fn initialize_native_target() {
    unsafe {
        LLVM_InitializeNativeTarget();
    }
}

/// TODO Documentation
pub fn initialize_native_asm_printer() {
    unsafe {
        LLVM_InitializeNativeAsmPrinter();
    }
}

