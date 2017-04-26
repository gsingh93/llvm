use llvm_sys::execution_engine::*;
use super::*;
use std::mem;

pub struct ExecutionEngine {
    pub ptr: LLVMExecutionEngineRef,
}

impl ExecutionEngine {
    pub fn create_for_module(module: Module) -> Result<ExecutionEngine, &'static str> {
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
impl Drop for ExecutionEngine {
    fn drop(&mut self) {
        unsafe {
            LLVMDisposeExecutionEngine(self.ptr);
        }
    }
}
