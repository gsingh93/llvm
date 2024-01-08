
use std::ffi::CString;
use llvm_sys::{error::*, error_handling::*};

use derive_more::{Deref, DerefMut};


///! Note:
///! "LLVMErrorRef" is an alias for "*mut LLVMOpaqueError"
///! "LLVMErrorTypeId" is an alias for "*const c_void"

/// TODO Docs
pub const SUCCESS: libc::c_int = LLVMErrorSuccess; // Just zero with extra steps


/// TODO Docs
#[derive(Debug, Deref, DerefMut)]
pub struct Error(LLVMErrorRef); // Contains raw pointer, field is private for safety

// TODO impl Drop for Error, consume error

impl Error {
    // TODO Docs
    pub fn consume_error(self) {
        unsafe { LLVMConsumeError(self.0) };
        drop(self)
    }

    // TODO Docs
    pub fn create_string_error(message: &str) -> Self {
        let c_msg = CString::new(message).expect("Convert &str to CString");
        let str_err = unsafe { LLVMCreateStringError(c_msg.as_ptr()) };
        Self(str_err)
    }

    // TODO Docs: Removes msg from error
    pub fn dispose_error_message(&mut self) {
        unsafe {
            let msg = LLVMGetErrorMessage(self.0);
            LLVMDisposeErrorMessage(msg)
        }
    }

    // TODO Docs
    pub fn get_error_message(&mut self) -> &str {
        let msg = unsafe { LLVMGetErrorMessage(self.0) };
        c_str_to_str!(msg)
    }

    // TODO Docs, plus, does this need a Rusty return type?
    pub fn get_error_type_id(&self) -> LLVMErrorTypeId {
        unsafe { LLVMGetErrorTypeId(self.0) }
    }

    // TODO Why is this useful exactly?
    pub fn get_string_error_type_id() -> LLVMErrorTypeId {
        unsafe { LLVMGetStringErrorTypeId() }
    }


    // TODO May need to add the unsafe versions of get message back, in case there are
    // other situations where I'd need to dispose of a message

}

///! Note:
///! "LLVMFatalErrorHandler" is a type alias for Option<extern "C" fn(Reason: *const c_char)>"

/// TODO Docs
pub struct FatalErrorHandler(LLVMFatalErrorHandler);

// TODO Revisit this, how can i create a new one safely? Would i ever need to do that?
impl FatalErrorHandler {
    // TODO Docs
    pub fn enable_pretty_stack_trace() {
        unsafe { LLVMEnablePrettyStackTrace() }
    }

    // TODO Docs
    pub fn install_fatal_error_handler(self) {
        unsafe { LLVMInstallFatalErrorHandler(self.0) }
        drop(self)
    }

    // TODO Docs
    pub fn reset_fatal_error_handler() {
        unsafe { LLVMResetFatalErrorHandler() }
    }
}





