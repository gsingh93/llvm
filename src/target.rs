use std::ffi::{CString, CStr};
use llvm_sys::target_machine::*;
use llvm_sys::target::*;
use super::*;

pub struct Target {
    ptr: LLVMTargetRef,
}
impl_llvm_ref!(Target, LLVMTargetRef);

impl Target {
    pub fn from_name(name: &str) -> Option<Target> {
        let c_name = CString::new(name).unwrap();
        let res = unsafe {
            LLVMGetTargetFromName(c_name.as_ptr())
        };

        if res.is_null() {
            None
        } else {
            Some(Target {
                ptr: res
            })
        }
    }
    pub fn create_target_machine(&self,
                                 triple: &str,
                                 cpu: &str,
                                 features: &str,
                                 level: LLVMCodeGenOptLevel,
                                 reloc: LLVMRelocMode,
                                 model: LLVMCodeModel) -> TargetMachine {
        TargetMachine::new(self, triple, cpu, features, level, reloc, model)
    }
}

pub struct TargetMachine {
    ptr: LLVMTargetMachineRef,
}

impl TargetMachine {
    // TODO: create wrappers for all these types
    pub fn new(target: &Target,
               triple: &str,
               cpu: &str,
               features: &str,
               level: LLVMCodeGenOptLevel,
               reloc: LLVMRelocMode,
               model: LLVMCodeModel) -> TargetMachine {

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

        TargetMachine {
            ptr: ptr,
        }
    }

    pub fn emit_to_file(&mut self,
                        module: &mut Module,
                        path: &str,
                        file_type: LLVMCodeGenFileType) -> Result<(), &'static str> {
            let c_path = CString::new(path).unwrap();
            let mut em: usize = 0;
            let em_ptr: *mut usize = &mut em;
            unsafe {
                LLVMTargetMachineEmitToFile(self.ptr,
                                            module.ptr,
                                            c_path.as_ptr() as *mut i8,
                                            file_type,
                                            em_ptr as *mut *mut i8);
                if em == 0 { // no error message was set
                    Ok(())
                } else {
                    Err(c_str_to_str!(em as *const i8))
                }
            }
    }
}

pub fn get_default_target_triple<'a>() -> &'a str {
    unsafe {
        c_str_to_str!(LLVMGetDefaultTargetTriple())
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

