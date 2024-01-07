use llvm_sys::core as llvm;
use llvm_sys::prelude::*;
use derive_more::{Deref, DerefMut};

// TODO Documentation
#[derive(Debug, Deref, DerefMut)]
pub struct PassManager {
    pub ptr: LLVMPassManagerRef,
}
configure_wrapper!(PassManager, LLVMPassManagerRef);

// TODO Documentation
impl PassManager {
    pub fn new() -> PassManager {
        PassManager {
            ptr: unsafe { llvm::LLVMCreatePassManager() }
        }
    }
}
