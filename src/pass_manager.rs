use llvm_sys::core as llvm;
use llvm_sys::prelude::*;

#[derive(Debug)]
pub struct PassManager {
    pub ptr: LLVMPassManagerRef,
}
impl_llvm_ref!(PassManager, LLVMPassManagerRef);

impl PassManager {
    pub fn new() -> PassManager {
        PassManager {
            ptr: unsafe { llvm::LLVMCreatePassManager() }
        }
    }
}
