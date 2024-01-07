use llvm_sys::core as llvm;
use llvm_sys::prelude::*;

// TODO Documentation
#[derive(Debug)]
pub struct PassManager {
    pub ptr: LLVMPassManagerRef,
}
map_to_llvm!(PassManager, LLVMPassManagerRef);

// TODO Documentation
impl PassManager {
    pub fn new() -> PassManager {
        PassManager {
            ptr: unsafe { llvm::LLVMCreatePassManager() }
        }
    }
}
