
use llvm_sys::{*, prelude::*};
use llvm_sys::core as llvm;

use derive_more::{Deref, DerefMut};

use super::*;


// TODO Documentation
#[derive(Debug, Deref, DerefMut)]
pub struct Builder {
    pub ptr: LLVMBuilderRef
}
configure_wrapper!(Builder, LLVMBuilderRef);

impl Drop for Builder {
    fn drop(&mut self) {
        unsafe {
            llvm::LLVMDisposeBuilder(self.ptr);
        }
    }
}

// TODO Documentation
impl Builder {
    /// Sets the insertion point to the end of the block
    pub fn position_at_end(&mut self, basic_block: LLVMBasicBlockRef) {
        unsafe {
            llvm::LLVMPositionBuilderAtEnd(self.ptr, basic_block);
        }
    }

    /// Builds a function call
    pub fn build_call(&mut self, func: Function, mut args: Vec<LLVMValueRef>, name: &str) -> LLVMValueRef {
        let c_name = CString::new(name).unwrap();
        unsafe {
            llvm::LLVMBuildCall(
                self.ptr,
                func.ptr,
                args.as_mut_ptr(),
                args.len() as u32,
                c_name.as_ptr()
            )
        }
    }

    // Functions for working with strings
    pub fn build_global_string(&self, s: &str, name: &str) -> LLVMValueRef {
        let c_s = CString::new(s).unwrap();
        let c_name = CString::new(name).unwrap();
        unsafe {
            llvm::LLVMBuildGlobalString(self.ptr, c_s.as_ptr(), c_name.as_ptr())
        }
    }
    pub fn build_global_string_ptr(&self, s: &str, name: &str) -> LLVMValueRef {
        let c_s = CString::new(s).unwrap();
        let c_name = CString::new(name).unwrap();
        unsafe {
            llvm::LLVMBuildGlobalStringPtr(self.ptr, c_s.as_ptr(), c_name.as_ptr())
        }
    }

    // Functions for working with element pointers
    pub fn build_in_bounds_gep(&self, ptr: LLVMValueRef, mut indices: Vec<LLVMValueRef>, name: &str) -> LLVMValueRef {
        let c_name = CString::new(name).unwrap();
        unsafe {
            llvm::LLVMBuildInBoundsGEP(self.ptr, ptr, indices.as_mut_ptr(), indices.len() as u32, c_name.as_ptr())
        }
    }
    pub fn build_gep(&self, ptr: LLVMValueRef, mut indices: Vec<LLVMValueRef>, name: &str) -> LLVMValueRef {
        let c_name = CString::new(name).unwrap();
        unsafe {
            llvm::LLVMBuildGEP(self.ptr, ptr, indices.as_mut_ptr(), indices.len() as u32, c_name.as_ptr())
        }
    }
}


// http://llvm.org/docs/doxygen/html/group__LLVMCCoreInstructionBuilder.html
//TODO: Get/Set Volatile
//TODO: Get/Set Ordering
//TODO: Almost everything from LLVMBuildAdd and upwards
// TODO: LLVMBuildBinOp
// TODO: LLVMBuildAtomicRMW


/// Generates wrapping functions for representing nameable(?) LLVM instructions
macro_rules! build_op_str {
    ($op_name:ident, $fn:path, $($argn:ident: $argv:path),*) => {
        impl Builder {
            pub fn $op_name(&mut self, $($argn: $argv),*, name: &str) -> LLVMValueRef {
                let c_name = CString::new(name).unwrap();
                unsafe {
                    $fn(self.ptr, $($argn),*, c_name.as_ptr())
                }
            }
        }
    }
}

/// Generates wrapping functions for representing LLVM instructions
macro_rules! build_op {
    ($op_name:ident, $fn:path $(, $($argn:ident: $argv:path),*)?) => {
        impl Builder {
            pub fn $op_name(&mut self $(, $($argn: $argv),*)?) -> LLVMValueRef {
                unsafe {
                    $fn(self.ptr $(, $($argn),*)?)
                }
            }
        }
    }
}

// Integer math
build_op_str!(build_neg, llvm::LLVMBuildNeg, val: LLVMValueRef);
build_op_str!(build_add, llvm::LLVMBuildAdd, lhs: LLVMValueRef, rhs: LLVMValueRef);
build_op_str!(build_sub, llvm::LLVMBuildSub, lhs: LLVMValueRef, rhs: LLVMValueRef);
build_op_str!(build_mul, llvm::LLVMBuildMul, lhs: LLVMValueRef, rhs: LLVMValueRef);
build_op_str!(build_sdiv, llvm::LLVMBuildSDiv, lhs: LLVMValueRef, rhs: LLVMValueRef);
build_op_str!(build_srem, llvm::LLVMBuildSRem, lhs: LLVMValueRef, rhs: LLVMValueRef);
build_op_str!(build_exact_sdiv, llvm::LLVMBuildExactSDiv, lhs: LLVMValueRef, rhs: LLVMValueRef);

// Floating point math
build_op_str!(build_fneg, llvm::LLVMBuildFNeg, val: LLVMValueRef);
build_op_str!(build_fadd, llvm::LLVMBuildFAdd, lhs: LLVMValueRef, rhs: LLVMValueRef);
build_op_str!(build_fsub, llvm::LLVMBuildFSub, lhs: LLVMValueRef, rhs: LLVMValueRef);
build_op_str!(build_fmul, llvm::LLVMBuildFMul, lhs: LLVMValueRef, rhs: LLVMValueRef);
build_op_str!(build_fdiv, llvm::LLVMBuildFDiv, lhs: LLVMValueRef, rhs: LLVMValueRef);
build_op_str!(build_frem, llvm::LLVMBuildFRem, lhs: LLVMValueRef, rhs: LLVMValueRef);

// No overflow signed wrap math
build_op_str!(build_nswneg, llvm::LLVMBuildNSWNeg, val: LLVMValueRef);
build_op_str!(build_nswadd, llvm::LLVMBuildNSWAdd, lhs: LLVMValueRef, rhs: LLVMValueRef);
build_op_str!(build_nswsub, llvm::LLVMBuildNSWSub, lhs: LLVMValueRef, rhs: LLVMValueRef);
build_op_str!(build_nswmul, llvm::LLVMBuildNSWMul, lhs: LLVMValueRef, rhs: LLVMValueRef);

// No overflow unsigned wrap math
build_op_str!(build_nuwneg, llvm::LLVMBuildNUWNeg, val: LLVMValueRef);
build_op_str!(build_nuwadd, llvm::LLVMBuildNUWAdd, lhs: LLVMValueRef, rhs: LLVMValueRef);
build_op_str!(build_nuwsub, llvm::LLVMBuildNUWSub, lhs: LLVMValueRef, rhs: LLVMValueRef);
build_op_str!(build_nuwmul, llvm::LLVMBuildNUWMul, lhs: LLVMValueRef, rhs: LLVMValueRef);

build_op_str!(build_udiv, llvm::LLVMBuildUDiv, lhs: LLVMValueRef, rhs: LLVMValueRef);
build_op_str!(build_urem, llvm::LLVMBuildURem, lhs: LLVMValueRef, rhs: LLVMValueRef);

// Bitshifting operations
build_op_str!(build_shl, llvm::LLVMBuildShl, lhs: LLVMValueRef, rhs: LLVMValueRef);
build_op_str!(build_lshr, llvm::LLVMBuildLShr, lhs: LLVMValueRef, rhs: LLVMValueRef);
build_op_str!(build_ashr, llvm::LLVMBuildAShr, lhs: LLVMValueRef, rhs: LLVMValueRef);

// Bitwise logical operators
build_op_str!(build_and, llvm::LLVMBuildAnd, lhs: LLVMValueRef, rhs: LLVMValueRef);
build_op_str!(build_or, llvm::LLVMBuildOr, lhs: LLVMValueRef, rhs: LLVMValueRef);
build_op_str!(build_xor, llvm::LLVMBuildXor, lhs: LLVMValueRef, rhs: LLVMValueRef);
build_op_str!(build_not, llvm::LLVMBuildNot, val: LLVMValueRef);

// Memory allocation
build_op_str!(build_malloc, llvm::LLVMBuildMalloc, typ: LLVMTypeRef);
build_op_str!(build_alloca, llvm::LLVMBuildAlloca, typ: LLVMTypeRef);
build_op_str!(build_array_malloc, llvm::LLVMBuildArrayMalloc, typ: LLVMTypeRef, val: LLVMValueRef);
build_op_str!(build_array_alloca, llvm::LLVMBuildArrayAlloca, typ: LLVMTypeRef, val: LLVMValueRef);

// Variable assignment and access
build_op!(build_store, llvm::LLVMBuildStore,  val: LLVMValueRef, ptr_val: LLVMValueRef);
build_op!(build_free, llvm::LLVMBuildFree, ptr_val: LLVMValueRef);
build_op_str!(build_load, llvm::LLVMBuildLoad, ptr: LLVMValueRef);

// Memory resizing instructions
build_op_str!(build_trunc, llvm::LLVMBuildTrunc, val: LLVMValueRef, dest_typ: LLVMTypeRef);
build_op_str!(build_zext, llvm::LLVMBuildZExt, val: LLVMValueRef, dest_typ: LLVMTypeRef);
build_op_str!(build_sext, llvm::LLVMBuildSExt, val: LLVMValueRef, dest_typ: LLVMTypeRef);
build_op_str!(build_fp_trunc, llvm::LLVMBuildFPTrunc, val: LLVMValueRef, dest_typ: LLVMTypeRef);
build_op_str!(build_fp_ext, llvm::LLVMBuildFPExt, val: LLVMValueRef, dest_typ: LLVMTypeRef);

// Conversions
build_op_str!(build_fp_to_ui, llvm::LLVMBuildFPToUI, val: LLVMValueRef, dest_typ: LLVMTypeRef);
build_op_str!(build_fp_to_si, llvm::LLVMBuildFPToSI, val: LLVMValueRef, dest_typ: LLVMTypeRef);
build_op_str!(build_ui_to_fp, llvm::LLVMBuildUIToFP, val: LLVMValueRef, dest_typ: LLVMTypeRef);
build_op_str!(build_si_to_fp, llvm::LLVMBuildSIToFP, val: LLVMValueRef, dest_typ: LLVMTypeRef);
build_op_str!(build_ptr_to_int, llvm::LLVMBuildPtrToInt, val: LLVMValueRef, dest_typ: LLVMTypeRef);
build_op_str!(build_int_to_ptr, llvm::LLVMBuildIntToPtr, val: LLVMValueRef, dest_typ: LLVMTypeRef);

// Casts
// TODO: improve LLVMOpcode
build_op_str!(build_cast, llvm::LLVMBuildCast, op: LLVMOpcode, val: LLVMValueRef, dest_typ: LLVMTypeRef);

build_op_str!(build_bit_cast, llvm::LLVMBuildBitCast, val: LLVMValueRef, dest_typ: LLVMTypeRef);
build_op_str!(build_trunc_or_bit_cast, llvm::LLVMBuildTruncOrBitCast, val: LLVMValueRef, dest_typ: LLVMTypeRef);
build_op_str!(build_zext_or_bit_cast, llvm::LLVMBuildZExtOrBitCast, val: LLVMValueRef, dest_typ: LLVMTypeRef);
build_op_str!(build_sext_or_bit_cast, llvm::LLVMBuildSExtOrBitCast, val: LLVMValueRef, dest_typ: LLVMTypeRef);

build_op_str!(build_addr_space_cast, llvm::LLVMBuildAddrSpaceCast, val: LLVMValueRef, dest_typ: LLVMTypeRef);
build_op_str!(build_pointer_cast, llvm::LLVMBuildPointerCast, val: LLVMValueRef, dest_typ: LLVMTypeRef);
build_op_str!(build_int_cast, llvm::LLVMBuildIntCast, val: LLVMValueRef, dest_typ: LLVMTypeRef);
build_op_str!(build_fpcast, llvm::LLVMBuildFPCast, val: LLVMValueRef, dest_typ: LLVMTypeRef);

// Comparison operations
build_op_str!(build_icmp, llvm::LLVMBuildICmp, op: LLVMIntPredicate, lhs: LLVMValueRef, rhs: LLVMValueRef);
build_op_str!(build_fcmp, llvm::LLVMBuildFCmp, op: LLVMRealPredicate, lhs: LLVMValueRef, rhs: LLVMValueRef);

// Phi node
build_op_str!(build_phi, llvm::LLVMBuildPhi, typ: LLVMTypeRef);

// Variable argument extractor
build_op_str!(build_vaarg, llvm::LLVMBuildVAArg, list: LLVMValueRef, typ: LLVMTypeRef);

// Working with vectors
build_op_str!(build_extract_element, llvm::LLVMBuildExtractElement, vec: LLVMValueRef, index: LLVMValueRef);
build_op_str!(build_insert_element, llvm::LLVMBuildInsertElement, vec: LLVMValueRef, val: LLVMValueRef, index: LLVMValueRef);
build_op_str!(build_shuffle_vector, llvm::LLVMBuildShuffleVector, v1: LLVMValueRef, v2: LLVMValueRef, mask: LLVMValueRef);

// Working with aggregates
build_op_str!(build_extract_value, llvm::LLVMBuildExtractValue, agg: LLVMValueRef, index: u32);
build_op_str!(build_insert_value, llvm::LLVMBuildInsertValue, agg: LLVMValueRef, val: LLVMValueRef, index: u32);

// Check if exists
build_op_str!(build_is_null, llvm::LLVMBuildIsNull, val: LLVMValueRef);
build_op_str!(build_is_not_null, llvm::LLVMBuildIsNotNull, val: LLVMValueRef);

// Pointer comparison
build_op_str!(build_ptr_diff, llvm::LLVMBuildPtrDiff, lhs: LLVMValueRef, rhs: LLVMValueRef);

// Memory fence for async and parallel threads
build_op_str!(build_fence, llvm::LLVMBuildFence, ordering: LLVMAtomicOrdering, singlethread: LLVMBool);

// Conditionals
build_op_str!(build_select, llvm::LLVMBuildSelect, cond: LLVMValueRef, then: LLVMValueRef, els: LLVMValueRef);
build_op!(build_cond_br, llvm::LLVMBuildCondBr, cond: LLVMValueRef, then: LLVMBasicBlockRef, els: LLVMBasicBlockRef);

// Return statements
build_op!(build_ret_void, llvm::LLVMBuildRetVoid);
build_op!(build_ret, llvm::LLVMBuildRet, ret_val: LLVMValueRef);
build_op!(build_br, llvm::LLVMBuildBr, dest: LLVMBasicBlockRef);


