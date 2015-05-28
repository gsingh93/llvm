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

// LLVM Wrappers

pub struct Context {
    pub context: LLVMContextRef
}

impl Context {
    pub fn new() -> Self {
        let context = unsafe {
            llvm::LLVMContextCreate()
        };
        Context { context: context }
    }

    pub fn create_builder(&self) -> Builder {
        let builder = unsafe {
            llvm::LLVMCreateBuilderInContext(self.context)
        };
        Builder { builder: builder }
    }

    pub fn module_create_with_name(&self, name: &str) -> Module {
        let c_name = CString::new(name).unwrap();
        let module = unsafe {
            llvm::LLVMModuleCreateWithNameInContext(c_name.as_ptr(), self.context)
        };
        Module { module: module }
    }

    pub fn void_type(&self) -> LLVMTypeRef {
        unsafe {
            llvm::LLVMVoidTypeInContext(self.context)
        }
    }

    pub fn int32_type(&self) -> LLVMTypeRef {
        unsafe {
            llvm::LLVMInt32TypeInContext(self.context)
        }
    }

    pub fn int16_type(&self) -> LLVMTypeRef {
        unsafe {
            llvm::LLVMInt16TypeInContext(self.context)
        }
    }

    pub fn int8_type(&self) -> LLVMTypeRef {
        unsafe {
            llvm::LLVMInt8TypeInContext(self.context)
        }
    }

    pub fn int1_type(&self) -> LLVMTypeRef {
        unsafe {
            llvm::LLVMInt1TypeInContext(self.context)
        }
    }

    pub fn const_bool(&self, val: bool) -> LLVMValueRef {
        let ty = self.int1_type();
        const_int(ty, val as u64, false)
    }

    pub fn append_basic_block(&self, func: LLVMValueRef, name: &str) -> LLVMBasicBlockRef {
        let c_name = CString::new(name).unwrap();
        unsafe {
            llvm::LLVMAppendBasicBlockInContext(self.context, func, c_name.as_ptr())
        }
    }
}

impl Drop for Context {
    fn drop(&mut self) {
        unsafe {
            llvm::LLVMContextDispose(self.context);
        }
    }
}

// No `Drop` impl is needed as this is disposed of when the associated context is disposed
pub struct Module {
    pub module: LLVMModuleRef
}

impl Module {
    pub fn dump(&self) {
        unsafe {
            llvm::LLVMDumpModule(self.module)
        }
    }

    pub fn add_function(&mut self, func_ty: LLVMTypeRef, name: &str) -> LLVMValueRef {
        let c_name = CString::new(name).unwrap();
        unsafe {
            llvm::LLVMAddFunction(self.module, c_name.as_ptr(), func_ty)
        }
    }

    pub fn get_named_function(&mut self, name: &str) -> Option<LLVMValueRef> {
        let c_name = CString::new(name).unwrap();
        let res = unsafe {
            llvm::LLVMGetNamedFunction(self.module, c_name.as_ptr())
        };

        if res.is_null() {
            None
        } else {
            Some(res)
        }
    }
}

pub struct Builder {
    pub builder: LLVMBuilderRef
}

impl Builder {
    pub fn position_at_end(&mut self, basic_block: LLVMBasicBlockRef) {
        unsafe {
            llvm::LLVMPositionBuilderAtEnd(self.builder, basic_block);
        }
    }

    pub fn build_ret(&mut self, ret_val: LLVMValueRef) -> LLVMValueRef {
        unsafe {
            llvm::LLVMBuildRet(self.builder, ret_val)
        }
    }

    pub fn build_ret_void(&self) -> LLVMValueRef {
        unsafe {
            llvm::LLVMBuildRetVoid(self.builder)
        }
    }

    pub fn build_alloca(&mut self, ty: LLVMTypeRef, name: &str) -> LLVMValueRef {
        let c_name = CString::new(name).unwrap();
        unsafe {
            llvm::LLVMBuildAlloca(self.builder, ty, c_name.as_ptr())
        }
    }

    pub fn build_store(&mut self, val: LLVMValueRef, ptr: LLVMValueRef) -> LLVMValueRef {
        unsafe {
            llvm::LLVMBuildStore(self.builder, val, ptr)
        }
    }

    pub fn build_load(&mut self, ptr: LLVMValueRef, name: &str) -> LLVMValueRef {
        let c_name = CString::new(name).unwrap();
        unsafe {
            llvm::LLVMBuildLoad(self.builder, ptr, c_name.as_ptr())
        }
    }

    pub fn build_call(&mut self, func: LLVMValueRef, mut args: Vec<LLVMValueRef>,
                      name: &str) -> LLVMValueRef {
        let c_name = CString::new(name).unwrap();
        unsafe {
            llvm::LLVMBuildCall(self.builder, func, args.as_mut_ptr(), args.len() as u32,
                                c_name.as_ptr())
        }
    }

    pub fn build_add(&mut self, lhs: LLVMValueRef, rhs: LLVMValueRef, name: &str) -> LLVMValueRef {
        let c_name = CString::new(name).unwrap();
        unsafe {
            llvm::LLVMBuildAdd(self.builder, lhs, rhs, c_name.as_ptr())
        }
    }

    pub fn build_sub(&mut self, lhs: LLVMValueRef, rhs: LLVMValueRef, name: &str) -> LLVMValueRef {
        let c_name = CString::new(name).unwrap();
        unsafe {
            llvm::LLVMBuildSub(self.builder, lhs, rhs, c_name.as_ptr())
        }
    }

    pub fn build_mul(&mut self, lhs: LLVMValueRef, rhs: LLVMValueRef, name: &str) -> LLVMValueRef {
        let c_name = CString::new(name).unwrap();
        unsafe {
            llvm::LLVMBuildMul(self.builder, lhs, rhs, c_name.as_ptr())
        }
    }

    pub fn build_sdiv(&mut self, lhs: LLVMValueRef, rhs: LLVMValueRef,
                      name: &str) -> LLVMValueRef {
        let c_name = CString::new(name).unwrap();
        unsafe {
            llvm::LLVMBuildSDiv(self.builder, lhs, rhs, c_name.as_ptr())
        }
    }

    pub fn build_icmp(&mut self, op: LLVMIntPredicate, lhs: LLVMValueRef, rhs: LLVMValueRef,
                      name: &str) -> LLVMValueRef {
        let c_name = CString::new(name).unwrap();
        unsafe {
            llvm::LLVMBuildICmp(self.builder, op, lhs, rhs, c_name.as_ptr())
        }
    }

    pub fn build_global_string(&self, s: &str, name: &str) -> LLVMValueRef {
        let c_s = CString::new(s).unwrap();
        let c_name = CString::new(name).unwrap();
        unsafe {
            llvm::LLVMBuildGlobalString(self.builder, c_s.as_ptr(), c_name.as_ptr())
        }
    }

    pub fn build_in_bounds_gep(&self, ptr: LLVMValueRef, mut indices: Vec<LLVMValueRef>,
                               name: &str) -> LLVMValueRef {
        let c_name = CString::new(name).unwrap();
        unsafe {
            llvm::LLVMBuildInBoundsGEP(self.builder, ptr, indices.as_mut_ptr(),
                                       indices.len() as u32, c_name.as_ptr())
        }
    }

    pub fn build_cond_br(&self, cond: LLVMValueRef, then: LLVMBasicBlockRef,
                         else_: LLVMBasicBlockRef) -> LLVMValueRef {
        unsafe {
            llvm::LLVMBuildCondBr(self.builder, cond, then, else_)
        }
    }

    pub fn build_br(&self, dest: LLVMBasicBlockRef) -> LLVMValueRef {
        unsafe {
            llvm::LLVMBuildBr(self.builder, dest)
        }
    }

}

impl Drop for Builder {
    fn drop(&mut self) {
        unsafe {
            llvm::LLVMDisposeBuilder(self.builder);
        }
    }
}

pub fn const_int(ty: LLVMTypeRef, val: u64, signed: bool) -> LLVMValueRef {
    unsafe {
        llvm::LLVMConstInt(ty, val, signed as i32)
    }
}

pub fn function_type(ret_ty: LLVMTypeRef, mut param_types: Vec<LLVMTypeRef>,
                     is_var_args: bool) -> LLVMTypeRef {
    unsafe {
        llvm::LLVMFunctionType(ret_ty, param_types.as_mut_ptr(), param_types.len() as u32,
                               is_var_args as i32)
    }
}

pub fn pointer_type(ty: LLVMTypeRef, address_space: u32) -> LLVMTypeRef {
    unsafe {
        llvm::LLVMPointerType(ty, address_space)
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
