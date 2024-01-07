use llvm_sys::execution_engine::*;
use super::*;
use std::mem;

use derive_more::{Deref, DerefMut};

#[derive(Debug, Deref, DerefMut)]
pub struct ExecutionEngine {
    pub ptr: LLVMExecutionEngineRef,
}
configure_wrapper!(ExecutionEngine, LLVMExecutionEngineRef);

impl ExecutionEngine {
    pub fn create_for_module(module: &Module) -> Result<ExecutionEngine, anyhow::Error> {
        unsafe {
            let mut ee = mem::uninitialized();
            let mut out = mem::zeroed();

            let res = LLVMCreateExecutionEngineForModule(&mut ee, module.ptr, &mut out);

            if res == 0 { // no error message was set
                Ok(ExecutionEngine{
                    ptr: ee
                })
            } else {
                Err(anyhow::anyhow!(c_str_to_str!(out as *const i8).to_string()))
            }
        }
    }

    pub fn get_function_address(&self, fname: &str) -> Option<extern "C" fn()> {
        let fname_s = CString::new(fname).unwrap();
        unsafe {
            let addr = LLVMGetFunctionAddress(self.ptr, fname_s.as_ptr());

            if addr == 0 {
                None
            } else {
                Some(mem::transmute(addr))
            }
        }
    }
}

impl Drop for ExecutionEngine {
    fn drop(&mut self) {
        unsafe {
            LLVMDisposeExecutionEngine(self.ptr);
        }
    }
}

pub fn link_in_mcjit() {
    unsafe { LLVMLinkInMCJIT(); }
}
