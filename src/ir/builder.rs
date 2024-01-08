
use std::ffi::CString;

use llvm_sys::{*, prelude::*};
use llvm_sys::core as llvm;

use derive_more::{Deref, DerefMut};

/*
    !! WARNING !!

    Types beginning in "LLVM" are unsafe pointers and should not be trusted.
    TODO Need to create wrapper types and perform checking for specific type kinds

*/


/// TODO Docs
#[derive(Debug, Deref, DerefMut)]
pub struct Builder(LLVMBuilderRef);

impl Drop for Builder {
    /// TODO Docs
    fn drop(&mut self) {
        unsafe { llvm::LLVMDisposeBuilder(self.0) }
    }
}

impl Builder {
    /// TODO Docs
    pub fn new() -> Self {
        let builder = unsafe { llvm::LLVMCreateBuilder() };
        Self(builder)
    }

    /// TODO Docs
    pub fn insert(&self, instruction: LLVMValueRef) {
        unsafe { llvm::LLVMInsertIntoBuilder(self.0, instruction) }
    }

    /// TODO Docs
    pub fn insert_with_name(&self, instruction: LLVMValueRef, name: &str) {
        let c_name = CString::new(name).expect("Convert &str to CString");
        unsafe { llvm::LLVMInsertIntoBuilderWithName(self.0, instruction, c_name.as_ptr()) }
    }

    /// TODO Docs
    pub fn position(&self, block: LLVMBasicBlockRef, instruction: LLVMValueRef) {
        unsafe { llvm::LLVMPositionBuilder(self.0, block, instruction) }
    }

    /// TODO Docs
    pub fn position_at_end(&self, block: LLVMBasicBlockRef) {
        unsafe { llvm::LLVMPositionBuilderAtEnd(self.0, block) }
    }

    /// TODO Docs
    pub fn position_before(&self, instruction: LLVMValueRef) {
        unsafe { llvm::LLVMPositionBuilderBefore(self.0, instruction) }
    }

    /// TODO Docs
    pub fn clear_insertion_position(&self) {
        unsafe { llvm::LLVMClearInsertionPosition(self.0) }
    }

    /// TODO Docs
    pub fn get_default_fp_math_tag(&self) -> LLVMMetadataRef {
        unsafe { llvm::LLVMBuilderGetDefaultFPMathTag(self.0) }
    }

    /// TODO Docs
    pub fn set_default_fp_math_tag(&self, fp_math_tag: LLVMMetadataRef) {
        unsafe { llvm::LLVMBuilderSetDefaultFPMathTag(self.0, fp_math_tag) }
    }
}


/// Generates functions representing LLVM IR build instructions
// TODO MERGE THESE MACROS INTO ONE: build_op!()
// TODO automatically prefix $op_name with "build_"
// TODO dynamically accomodate &str and &mut [...] conversions
// TODO add variable return
macro_rules! op {
    ($op_name:ident, $fn:path $(, $($argn:ident: $argv:path),*)?) => {
        impl Builder {
            /// TODO Dynamically-generated docs, maybe attribute macro?
            pub fn $op_name(&mut self $(, $($argn: $argv),*)?) -> LLVMValueRef {
                unsafe {
                    $fn(self.0 $(, $($argn),*)?)
                }
            }
        }
    }
}

/// Generates functions representing LLVM IR build instructions with a "name" argument
// TODO add variable return
macro_rules! op_with_name {
    ($op_name:ident, $fn:path, $($argn:ident: $argv:path),*) => {
        impl Builder {
            /// TODO Docs
            pub fn $op_name(&mut self, $($argn: $argv),*, name: &str) -> LLVMValueRef {
                let c_name = CString::new(name).expect("Convert &str to CString");
                unsafe {
                    $fn(self.0, $($argn),*, c_name.as_ptr())
                }
            }
        }
    }
}

// Memory allocation
op_with_name!(build_malloc, llvm::LLVMBuildMalloc, typ: LLVMTypeRef);
op_with_name!(build_alloca, llvm::LLVMBuildAlloca, typ: LLVMTypeRef);
op_with_name!(build_array_malloc, llvm::LLVMBuildArrayMalloc, typ: LLVMTypeRef, val: LLVMValueRef);
op_with_name!(build_array_alloca, llvm::LLVMBuildArrayAlloca, typ: LLVMTypeRef, val: LLVMValueRef);

// Working with memory
op!(build_mem_set, llvm::LLVMBuildMemSet, ptr: LLVMValueRef, val: LLVMValueRef, size: LLVMValueRef, align: u32);
op!(build_mem_move, llvm::LLVMBuildMemMove,
    dest: LLVMValueRef, dest_align: u32, src: LLVMValueRef, src_align: u32, size: LLVMValueRef);
op!(build_mem_cpy, llvm::LLVMBuildMemCpy,
    dest: LLVMValueRef, dest_align: u32, src: LLVMValueRef, src_align: u32, size: LLVMValueRef);

// Pointer comparison
op_with_name!(build_ptr_diff, llvm::LLVMBuildPtrDiff, lhs: LLVMValueRef, rhs: LLVMValueRef);

// Check existence
op_with_name!(build_is_null, llvm::LLVMBuildIsNull, val: LLVMValueRef);
op_with_name!(build_is_not_null, llvm::LLVMBuildIsNotNull, val: LLVMValueRef);

// Atomics & Concurrency
op_with_name!(build_fence, llvm::LLVMBuildFence, ordering: LLVMAtomicOrdering, singlethread: LLVMBool);
op!(build_atomic_cmp_xchg, llvm::LLVMBuildAtomicCmpXchg,
    ptr: LLVMValueRef, cmp: LLVMValueRef, new: LLVMValueRef,
    succ_ord: LLVMAtomicOrdering, fail_ord: LLVMAtomicOrdering,
    singlethread: LLVMBool);
op!(build_atomic_rmw, llvm::LLVMBuildAtomicRMW,
    op: LLVMAtomicRMWBinOp, ptr: LLVMValueRef, val: LLVMValueRef, ord: LLVMAtomicOrdering, singlethread: LLVMBool);

// Global strings
impl Builder {
    // TODO Docs
    pub fn build_global_string(&self, string: &str, name: &str) -> LLVMValueRef {
        let c_string = CString::new(string).expect("Convert &str to CString");
        let c_name = CString::new(name).expect("Convert &str to CString");
        unsafe { llvm::LLVMBuildGlobalString(self.0, c_string.as_ptr(), c_name.as_ptr()) }
    }

    // TODO Docs
    pub fn build_global_string_ptr(&self, string: &str, name: &str) -> LLVMValueRef {
        let c_string = CString::new(string).expect("Convert &str to CString");
        let c_name = CString::new(name).expect("Convert &str to CString");
        unsafe { llvm::LLVMBuildGlobalStringPtr(self.0, c_string.as_ptr(), c_name.as_ptr()) }
    }
}

// Variable assignment and access
op!(build_store, llvm::LLVMBuildStore,  val: LLVMValueRef, ptr: LLVMValueRef);
op!(build_free, llvm::LLVMBuildFree, ptr: LLVMValueRef);
op_with_name!(build_load, llvm::LLVMBuildLoad, ptr: LLVMValueRef);
op_with_name!(build_load_2, llvm::LLVMBuildLoad2, typ: LLVMTypeRef, ptr: LLVMValueRef);
op_with_name!(build_freeze, llvm::LLVMBuildFreeze, val: LLVMValueRef);

// Memory resizing instructions
op_with_name!(build_trunc, llvm::LLVMBuildTrunc, val: LLVMValueRef, dest_typ: LLVMTypeRef);
op_with_name!(build_zext, llvm::LLVMBuildZExt, val: LLVMValueRef, dest_typ: LLVMTypeRef);
op_with_name!(build_sext /*teehee*/, llvm::LLVMBuildSExt, val: LLVMValueRef, dest_typ: LLVMTypeRef);
op_with_name!(build_fp_trunc, llvm::LLVMBuildFPTrunc, val: LLVMValueRef, dest_typ: LLVMTypeRef);
op_with_name!(build_fp_ext, llvm::LLVMBuildFPExt, val: LLVMValueRef, dest_typ: LLVMTypeRef);

// Conversions
op_with_name!(build_fp_to_ui, llvm::LLVMBuildFPToUI, val: LLVMValueRef, dest_typ: LLVMTypeRef);
op_with_name!(build_fp_to_si, llvm::LLVMBuildFPToSI, val: LLVMValueRef, dest_typ: LLVMTypeRef);
op_with_name!(build_ui_to_fp, llvm::LLVMBuildUIToFP, val: LLVMValueRef, dest_typ: LLVMTypeRef);
op_with_name!(build_si_to_fp, llvm::LLVMBuildSIToFP, val: LLVMValueRef, dest_typ: LLVMTypeRef);
op_with_name!(build_ptr_to_int, llvm::LLVMBuildPtrToInt, val: LLVMValueRef, dest_typ: LLVMTypeRef);
op_with_name!(build_int_to_ptr, llvm::LLVMBuildIntToPtr, val: LLVMValueRef, dest_typ: LLVMTypeRef);

// Casts
op_with_name!(build_cast, llvm::LLVMBuildCast, op: LLVMOpcode, val: LLVMValueRef, dest_typ: LLVMTypeRef);

op_with_name!(build_bit_cast, llvm::LLVMBuildBitCast, val: LLVMValueRef, dest_typ: LLVMTypeRef);
op_with_name!(build_trunc_or_bit_cast, llvm::LLVMBuildTruncOrBitCast, val: LLVMValueRef, dest_typ: LLVMTypeRef);
op_with_name!(build_zext_or_bit_cast, llvm::LLVMBuildZExtOrBitCast, val: LLVMValueRef, dest_typ: LLVMTypeRef);
op_with_name!(build_sext_or_bit_cast, llvm::LLVMBuildSExtOrBitCast, val: LLVMValueRef, dest_typ: LLVMTypeRef);

op_with_name!(build_addr_space_cast, llvm::LLVMBuildAddrSpaceCast, val: LLVMValueRef, dest_typ: LLVMTypeRef);
op_with_name!(build_pointer_cast, llvm::LLVMBuildPointerCast, val: LLVMValueRef, dest_typ: LLVMTypeRef);
op_with_name!(build_int_cast, llvm::LLVMBuildIntCast, val: LLVMValueRef, dest_typ: LLVMTypeRef);
op_with_name!(build_int_cast_2, llvm::LLVMBuildIntCast2, val: LLVMValueRef, dest_typ: LLVMTypeRef, is_signed: LLVMBool);
op_with_name!(build_fp_cast, llvm::LLVMBuildFPCast, val: LLVMValueRef, dest_typ: LLVMTypeRef);

// Comparison operations
op_with_name!(build_icmp, llvm::LLVMBuildICmp, op: LLVMIntPredicate, lhs: LLVMValueRef, rhs: LLVMValueRef);
op_with_name!(build_fcmp, llvm::LLVMBuildFCmp, op: LLVMRealPredicate, lhs: LLVMValueRef, rhs: LLVMValueRef);

// Phi node
op_with_name!(build_phi, llvm::LLVMBuildPhi, typ: LLVMTypeRef);

// Variable argument extractor
op_with_name!(build_vaarg, llvm::LLVMBuildVAArg, list: LLVMValueRef, typ: LLVMTypeRef);

// Working with vectors
op_with_name!(build_extract_element, llvm::LLVMBuildExtractElement, vec: LLVMValueRef, index: LLVMValueRef);
op_with_name!(build_insert_element, llvm::LLVMBuildInsertElement, vec: LLVMValueRef, val: LLVMValueRef, index: LLVMValueRef);
op_with_name!(build_shuffle_vector, llvm::LLVMBuildShuffleVector, v1: LLVMValueRef, v2: LLVMValueRef, mask: LLVMValueRef);

// Working with aggregates
op_with_name!(build_extract_value, llvm::LLVMBuildExtractValue, agg: LLVMValueRef, index: u32);
op_with_name!(build_insert_value, llvm::LLVMBuildInsertValue, agg: LLVMValueRef, val: LLVMValueRef, index: u32);

// Referencing elements (Get Element Pointer)
impl Builder {
    // TODO Docs, macro?
    pub fn build_gep(&self, ptr: LLVMValueRef, slice: &mut [LLVMValueRef], name: &str) -> LLVMValueRef {
        let c_name = CString::new(name).expect("Convert &str to CString");
        unsafe {
            llvm::LLVMBuildGEP(self.0, ptr, slice.as_mut_ptr(), slice.len() as u32, c_name.as_ptr())
        }
    }

    // TODO Docs, macro?
    pub fn build_gep_2(&self, typ: LLVMTypeRef, ptr: LLVMValueRef, slice: &mut [LLVMValueRef], name: &str) -> LLVMValueRef {
        let c_name = CString::new(name).expect("Convert &str to CString");
        unsafe {
            llvm::LLVMBuildGEP2(self.0, typ, ptr, slice.as_mut_ptr(), slice.len() as u32, c_name.as_ptr())
        }
    }

    // TODO Docs, macro?
    pub fn build_in_bounds_gep(&self, ptr: LLVMValueRef, slice: &mut [LLVMValueRef], name: &str) -> LLVMValueRef {
        let c_name = CString::new(name).expect("Convert &str to CString");
        unsafe {
            llvm::LLVMBuildInBoundsGEP(self.0, ptr, slice.as_mut_ptr(), slice.len() as u32, c_name.as_ptr())
        }
    }

    // TODO Docs, macro?
    pub fn build_in_bounds_gep_2(&self, typ: LLVMTypeRef, ptr: LLVMValueRef, slice: &mut [LLVMValueRef], name: &str) -> LLVMValueRef {
        let c_name = CString::new(name).expect("Convert &str to CString");
        unsafe {
            llvm::LLVMBuildInBoundsGEP2(self.0, typ, ptr, slice.as_mut_ptr(), slice.len() as u32, c_name.as_ptr())
        }
    }

    // TODO Docs, macro?
    pub fn build_struct_gep(&self, ptr: LLVMValueRef, index: u32, name: &str) -> LLVMValueRef {
        let c_name = CString::new(name).expect("Convert &str to CString");
        unsafe {
            llvm::LLVMBuildStructGEP(self.0, ptr, index, c_name.as_ptr())
        }
    }

    // TODO Docs, macro?
    pub fn build_struct_gep_2(&self, typ: LLVMTypeRef, ptr: LLVMValueRef, index: u32, name: &str) -> LLVMValueRef {
        let c_name = CString::new(name).expect("Convert &str to CString");
        unsafe {
            llvm::LLVMBuildStructGEP2(self.0, typ, ptr, index, c_name.as_ptr())
        }
    }
}

// Binary operation from Opcode
op_with_name!(build_bin_op, llvm::LLVMBuildBinOp, op: LLVMOpcode, lhs: LLVMValueRef, rhs: LLVMValueRef);

// Integer math
op_with_name!(build_neg, llvm::LLVMBuildNeg, val: LLVMValueRef);
op_with_name!(build_add, llvm::LLVMBuildAdd, lhs: LLVMValueRef, rhs: LLVMValueRef);
op_with_name!(build_sub, llvm::LLVMBuildSub, lhs: LLVMValueRef, rhs: LLVMValueRef);
op_with_name!(build_mul, llvm::LLVMBuildMul, lhs: LLVMValueRef, rhs: LLVMValueRef);
op_with_name!(build_sdiv, llvm::LLVMBuildSDiv, lhs: LLVMValueRef, rhs: LLVMValueRef);
op_with_name!(build_srem, llvm::LLVMBuildSRem, lhs: LLVMValueRef, rhs: LLVMValueRef);
op_with_name!(build_exact_sdiv, llvm::LLVMBuildExactSDiv, lhs: LLVMValueRef, rhs: LLVMValueRef);

// Floating point math
op_with_name!(build_fneg, llvm::LLVMBuildFNeg, val: LLVMValueRef);
op_with_name!(build_fadd, llvm::LLVMBuildFAdd, lhs: LLVMValueRef, rhs: LLVMValueRef);
op_with_name!(build_fsub, llvm::LLVMBuildFSub, lhs: LLVMValueRef, rhs: LLVMValueRef);
op_with_name!(build_fmul, llvm::LLVMBuildFMul, lhs: LLVMValueRef, rhs: LLVMValueRef);
op_with_name!(build_fdiv, llvm::LLVMBuildFDiv, lhs: LLVMValueRef, rhs: LLVMValueRef);
op_with_name!(build_frem, llvm::LLVMBuildFRem, lhs: LLVMValueRef, rhs: LLVMValueRef);

// No overflow signed wrapping math
op_with_name!(build_nswneg, llvm::LLVMBuildNSWNeg, val: LLVMValueRef);
op_with_name!(build_nswadd, llvm::LLVMBuildNSWAdd, lhs: LLVMValueRef, rhs: LLVMValueRef);
op_with_name!(build_nswsub, llvm::LLVMBuildNSWSub, lhs: LLVMValueRef, rhs: LLVMValueRef);
op_with_name!(build_nswmul, llvm::LLVMBuildNSWMul, lhs: LLVMValueRef, rhs: LLVMValueRef);

// No overflow unsigned wrapping math
op_with_name!(build_nuwneg, llvm::LLVMBuildNUWNeg, val: LLVMValueRef);
op_with_name!(build_nuwadd, llvm::LLVMBuildNUWAdd, lhs: LLVMValueRef, rhs: LLVMValueRef);
op_with_name!(build_nuwsub, llvm::LLVMBuildNUWSub, lhs: LLVMValueRef, rhs: LLVMValueRef);
op_with_name!(build_nuwmul, llvm::LLVMBuildNUWMul, lhs: LLVMValueRef, rhs: LLVMValueRef);

op_with_name!(build_udiv, llvm::LLVMBuildUDiv, lhs: LLVMValueRef, rhs: LLVMValueRef);
op_with_name!(build_urem, llvm::LLVMBuildURem, lhs: LLVMValueRef, rhs: LLVMValueRef);
op_with_name!(build_exact_udiv, llvm::LLVMBuildExactUDiv, lhs: LLVMValueRef, rhs: LLVMValueRef);

// Bitshifting operations
op_with_name!(build_shl, llvm::LLVMBuildShl, lhs: LLVMValueRef, rhs: LLVMValueRef);
op_with_name!(build_lshr, llvm::LLVMBuildLShr, lhs: LLVMValueRef, rhs: LLVMValueRef);
op_with_name!(build_ashr, llvm::LLVMBuildAShr, lhs: LLVMValueRef, rhs: LLVMValueRef);

// Bitwise logical operators
op_with_name!(build_and, llvm::LLVMBuildAnd, lhs: LLVMValueRef, rhs: LLVMValueRef);
op_with_name!(build_or, llvm::LLVMBuildOr, lhs: LLVMValueRef, rhs: LLVMValueRef);
op_with_name!(build_xor, llvm::LLVMBuildXor, lhs: LLVMValueRef, rhs: LLVMValueRef);
op_with_name!(build_not, llvm::LLVMBuildNot, val: LLVMValueRef);

// Flow control & branching
op_with_name!(build_select, llvm::LLVMBuildSelect, cond: LLVMValueRef, then: LLVMValueRef, els: LLVMValueRef);
op!(build_cond_br, llvm::LLVMBuildCondBr, cond: LLVMValueRef, then: LLVMBasicBlockRef, els: LLVMBasicBlockRef);
op!(build_switch, llvm::LLVMBuildSwitch, val: LLVMValueRef, els: LLVMBasicBlockRef, num_cases: u32);
op!(build_br, llvm::LLVMBuildBr, dest: LLVMBasicBlockRef);
op!(build_indirect_br, llvm::LLVMBuildIndirectBr, addr: LLVMValueRef, num_dests: u32);
op!(build_unreachable, llvm::LLVMBuildUnreachable);

// Functions
impl Builder {
    // TODO Docs, macro?
    pub fn build_call(&self, function: LLVMValueRef, args: &mut [LLVMValueRef], name: &str) -> LLVMValueRef {
        let c_name = CString::new(name).expect("Convert &str to CString");
        unsafe {
            llvm::LLVMBuildCall(self.0, function, args.as_mut_ptr(), args.len() as u32, c_name.as_ptr())
        }
    }

    // TODO Docs, macro?
    pub fn build_call_2(&self, typ: LLVMTypeRef, function: LLVMValueRef, args: &mut [LLVMValueRef], name: &str) -> LLVMValueRef {
        let c_name = CString::new(name).expect("Convert &str to CString");
        unsafe {
            llvm::LLVMBuildCall2(self.0, typ, function, args.as_mut_ptr(), args.len() as u32, c_name.as_ptr())
        }
    }
}

// Return statements
op!(build_ret_void, llvm::LLVMBuildRetVoid);
op!(build_ret, llvm::LLVMBuildRet, ret_val: LLVMValueRef);
impl Builder {
    // TODO Docs, macro?
    pub fn build_aggregate_ret(&self, ret_aggr: Vec<LLVMValueRef>) -> LLVMValueRef {
        let mut slice: Vec<_> = ret_aggr.into(); // TODO Revisit, should this use &mut[LLVMValueRef] instead?
        unsafe {
            llvm::LLVMBuildAggregateRet(self.0, slice.as_mut_ptr(), slice.len() as u32)
        }
    }
}

// Function components involving exceptions
op!(build_resume, llvm::LLVMBuildResume, exception: LLVMValueRef);
op!(build_catch_ret, llvm::LLVMBuildCatchRet, pad: LLVMValueRef, block: LLVMBasicBlockRef);
op!(build_cleanup_ret, llvm::LLVMBuildCleanupRet, pad: LLVMValueRef, block: LLVMBasicBlockRef);
op_with_name!(build_landing_pad, llvm::LLVMBuildLandingPad, typ: LLVMTypeRef, pers_fun: LLVMValueRef, num_clauses: u32);
op_with_name!(build_catch_switch, llvm::LLVMBuildCatchSwitch, pad: LLVMValueRef, unwind: LLVMBasicBlockRef, num_handler: u32);
impl Builder {
    // TODO Docs, macro?
    pub fn build_invoke(
        &self,
        function: LLVMValueRef,
        args: &mut [LLVMValueRef],
        then: LLVMBasicBlockRef,
        catch: LLVMBasicBlockRef,
        name: &str
    ) -> LLVMValueRef {
        let c_name = CString::new(name).expect("Convert &str to CString");
        unsafe {
            llvm::LLVMBuildInvoke(
                self.0,
                function,
                args.as_mut_ptr(),
                args.len() as u32,
                then,
                catch,
                c_name.as_ptr()
            )
        }
    }

    // TODO Docs, macro?
    pub fn build_invoke_2(
        &self,
        typ: LLVMTypeRef,
        function: LLVMValueRef,
        args: &mut [LLVMValueRef],
        then: LLVMBasicBlockRef,
        catch: LLVMBasicBlockRef,
        name: &str
    ) -> LLVMValueRef {
        let c_name = CString::new(name).expect("Convert &str to CString");
        unsafe {
            llvm::LLVMBuildInvoke2(
                self.0,
                typ,
                function,
                args.as_mut_ptr(),
                args.len() as u32,
                then,
                catch,
                c_name.as_ptr()
            )
        }
    }

    // TODO Docs, macro?
    pub fn build_catch_pad(&self, pad: LLVMValueRef, args: &mut [LLVMValueRef], name: &str) -> LLVMValueRef {
        let c_name = CString::new(name).expect("Convert &str to CString");
        unsafe {
            llvm::LLVMBuildCatchPad(self.0, pad, args.as_mut_ptr(), args.len() as u32, c_name.as_ptr())
        }
    }
    // TODO Docs, macro?
    pub fn build_cleanup_pad(&self, pad: LLVMValueRef, args: &mut [LLVMValueRef], name: &str) -> LLVMValueRef {
        let c_name = CString::new(name).expect("Convert &str to CString");
        unsafe {
            llvm::LLVMBuildCleanupPad(self.0, pad, args.as_mut_ptr(), args.len() as u32, c_name.as_ptr())
        }
    }
}















