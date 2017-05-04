use llvm_sys::execution_engine::*;
use super::*;
use std::mem;

pub struct ExecutionEngine {
    pub ptr: LLVMExecutionEngineRef,
}
impl_llvm_ref!(ExecutionEngine, LLVMExecutionEngineRef);

impl ExecutionEngine {
    pub fn create_for_module(module: &Module) -> Result<ExecutionEngine, &'static str> {
        unsafe {
            let mut ee = mem::uninitialized();
            let mut out = mem::zeroed();

            let res = LLVMCreateExecutionEngineForModule(&mut ee, module.ptr, &mut out);

            if res == 0 { // no error message was set
                Ok(ExecutionEngine{
                    ptr: ee
                })
            } else {
                Err(c_str_to_str!(out as *const i8))
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
    unsafe {
        LLVMLinkInMCJIT();
    }
}
