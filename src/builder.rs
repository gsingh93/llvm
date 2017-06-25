use llvm_sys::*;
use llvm_sys::prelude::*;
use llvm_sys::core as llvm;

use super::*;

macro_rules! build_op_str {
    ($op_name: ident, $fn: path, $($argn: ident: $argv: path),*) => {
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

macro_rules! build_op {
    ($op_name: ident, $fn: path, $($argn: ident: $argv: path),*) => {
        impl Builder {
            pub fn $op_name(&mut self, $($argn: $argv),*) -> LLVMValueRef {
                unsafe {
                    $fn(self.ptr, $($argn),*)
                }
            }
        }
    }
}

#[derive(Debug)]
pub struct Builder {
    pub ptr: LLVMBuilderRef
}
impl_llvm_ref!(Builder, LLVMBuilderRef);

// http://llvm.org/docs/doxygen/html/group__LLVMCCoreInstructionBuilder.html
//TODO: Get/Set Volatile
//TODO: Get/Set Ordering
//TODO: Almost everything from LLVMBuildAdd and upwards

build_op_str!(build_add, llvm::LLVMBuildAdd, lhs: LLVMValueRef, rhs: LLVMValueRef);
build_op_str!(build_sub, llvm::LLVMBuildSub, lhs: LLVMValueRef, rhs: LLVMValueRef);
build_op_str!(build_mul, llvm::LLVMBuildMul, lhs: LLVMValueRef, rhs: LLVMValueRef);

build_op_str!(build_fadd, llvm::LLVMBuildFAdd, lhs: LLVMValueRef, rhs: LLVMValueRef);
build_op_str!(build_fsub, llvm::LLVMBuildFSub, lhs: LLVMValueRef, rhs: LLVMValueRef);
build_op_str!(build_fmul, llvm::LLVMBuildFMul, lhs: LLVMValueRef, rhs: LLVMValueRef);
build_op_str!(build_fdiv, llvm::LLVMBuildFDiv, lhs: LLVMValueRef, rhs: LLVMValueRef);

build_op_str!(build_nswadd, llvm::LLVMBuildNSWAdd, lhs: LLVMValueRef, rhs: LLVMValueRef);
build_op_str!(build_nswsub, llvm::LLVMBuildNSWSub, lhs: LLVMValueRef, rhs: LLVMValueRef);
build_op_str!(build_nswmul, llvm::LLVMBuildNSWMul, lhs: LLVMValueRef, rhs: LLVMValueRef);

build_op_str!(build_nuwadd, llvm::LLVMBuildNUWAdd, lhs: LLVMValueRef, rhs: LLVMValueRef);
build_op_str!(build_nuwsub, llvm::LLVMBuildNUWSub, lhs: LLVMValueRef, rhs: LLVMValueRef);
build_op_str!(build_nuwmul, llvm::LLVMBuildNUWMul, lhs: LLVMValueRef, rhs: LLVMValueRef);

build_op_str!(build_udiv, llvm::LLVMBuildUDiv, lhs: LLVMValueRef, rhs: LLVMValueRef);
build_op_str!(build_sdiv, llvm::LLVMBuildSDiv, lhs: LLVMValueRef, rhs: LLVMValueRef);
build_op_str!(build_exact_sdiv, llvm::LLVMBuildExactSDiv, lhs: LLVMValueRef, rhs: LLVMValueRef);

build_op_str!(build_urem, llvm::LLVMBuildURem, lhs: LLVMValueRef, rhs: LLVMValueRef);
build_op_str!(build_srem, llvm::LLVMBuildSRem, lhs: LLVMValueRef, rhs: LLVMValueRef);
build_op_str!(build_frem, llvm::LLVMBuildFRem, lhs: LLVMValueRef, rhs: LLVMValueRef);

build_op_str!(build_shl, llvm::LLVMBuildShl, lhs: LLVMValueRef, rhs: LLVMValueRef);
build_op_str!(build_lshr, llvm::LLVMBuildLShr, lhs: LLVMValueRef, rhs: LLVMValueRef);
build_op_str!(build_ashr, llvm::LLVMBuildAShr, lhs: LLVMValueRef, rhs: LLVMValueRef);

build_op_str!(build_and, llvm::LLVMBuildAnd, lhs: LLVMValueRef, rhs: LLVMValueRef);
build_op_str!(build_or, llvm::LLVMBuildOr, lhs: LLVMValueRef, rhs: LLVMValueRef);
build_op_str!(build_xor, llvm::LLVMBuildXor, lhs: LLVMValueRef, rhs: LLVMValueRef);

// TODO: LLVMBuildBinOp

build_op_str!(build_neg, llvm::LLVMBuildNeg, v: LLVMValueRef);
build_op_str!(build_fneg, llvm::LLVMBuildFNeg, v: LLVMValueRef);
build_op_str!(build_nswneg, llvm::LLVMBuildNSWNeg, v: LLVMValueRef);
build_op_str!(build_nuwneg, llvm::LLVMBuildNUWNeg, v: LLVMValueRef);

build_op_str!(build_not, llvm::LLVMBuildNot, v: LLVMValueRef);

build_op_str!(build_malloc, llvm::LLVMBuildMalloc, typ: LLVMTypeRef);
build_op_str!(build_array_malloc, llvm::LLVMBuildArrayMalloc, typ: LLVMTypeRef,
                                                              val: LLVMValueRef);

build_op_str!(build_alloca, llvm::LLVMBuildAlloca, ty: LLVMTypeRef);
build_op_str!(build_array_alloca, llvm::LLVMBuildArrayAlloca, ty: LLVMTypeRef,
                                                              val: LLVMValueRef);

build_op!(build_free, llvm::LLVMBuildFree, pval: LLVMValueRef);
build_op_str!(build_load, llvm::LLVMBuildLoad, ptr: LLVMValueRef);
build_op!(build_store, llvm::LLVMBuildStore,  val: LLVMValueRef, pval: LLVMValueRef);

build_op_str!(build_trunc, llvm::LLVMBuildTrunc, val: LLVMValueRef, dest_ty: LLVMTypeRef);
build_op_str!(build_zext, llvm::LLVMBuildZExt, val: LLVMValueRef, dest_ty: LLVMTypeRef);
build_op_str!(build_sext, llvm::LLVMBuildSExt, val: LLVMValueRef, dest_ty: LLVMTypeRef);
build_op_str!(build_fp_to_ui, llvm::LLVMBuildFPToUI, val: LLVMValueRef, dest_ty: LLVMTypeRef);
build_op_str!(build_fp_to_si, llvm::LLVMBuildFPToSI, val: LLVMValueRef, dest_ty: LLVMTypeRef);
build_op_str!(build_ui_to_fp, llvm::LLVMBuildUIToFP, val: LLVMValueRef, dest_ty: LLVMTypeRef);
build_op_str!(build_si_to_fp, llvm::LLVMBuildSIToFP, val: LLVMValueRef, dest_ty: LLVMTypeRef);
build_op_str!(build_fp_trunc, llvm::LLVMBuildFPTrunc, val: LLVMValueRef, dest_ty: LLVMTypeRef);
build_op_str!(build_fp_ext, llvm::LLVMBuildFPExt, val: LLVMValueRef, dest_ty: LLVMTypeRef);
build_op_str!(build_ptr_to_int, llvm::LLVMBuildPtrToInt, val: LLVMValueRef, dest_ty: LLVMTypeRef);
build_op_str!(build_int_to_ptr, llvm::LLVMBuildIntToPtr, val: LLVMValueRef, dest_ty: LLVMTypeRef);

build_op_str!(build_bit_cast, llvm::LLVMBuildBitCast, val: LLVMValueRef, dest_ty: LLVMTypeRef);
build_op_str!(build_addr_space_cast, llvm::LLVMBuildAddrSpaceCast, val: LLVMValueRef,
                                                                   dest_ty: LLVMTypeRef);
build_op_str!(build_zext_or_bit_cast, llvm::LLVMBuildZExtOrBitCast, val: LLVMValueRef,
                                                                    dest_ty: LLVMTypeRef);
build_op_str!(build_sext_or_bit_cast, llvm::LLVMBuildSExtOrBitCast, val: LLVMValueRef,
                                                                    dest_ty: LLVMTypeRef);
build_op_str!(build_trunc_or_bit_cast, llvm::LLVMBuildTruncOrBitCast, val: LLVMValueRef,
                                                                      dest_ty: LLVMTypeRef);

// TODO: improve LLVMOpcode
build_op_str!(build_cast, llvm::LLVMBuildCast, op: LLVMOpcode,
                                               val: LLVMValueRef,
                                               dest_ty: LLVMTypeRef);

build_op_str!(build_pointer_cast, llvm::LLVMBuildPointerCast, val: LLVMValueRef, dest_ty: LLVMTypeRef);
build_op_str!(build_int_cast, llvm::LLVMBuildIntCast, val: LLVMValueRef, dest_ty: LLVMTypeRef);
build_op_str!(build_fpcast, llvm::LLVMBuildFPCast, val: LLVMValueRef, dest_ty: LLVMTypeRef);


build_op_str!(build_icmp, llvm::LLVMBuildICmp, op: LLVMIntPredicate,
                                               lhs: LLVMValueRef,
                                               rhs: LLVMValueRef);

build_op_str!(build_fcmp, llvm::LLVMBuildFCmp, op: LLVMRealPredicate,
                                               lhs: LLVMValueRef,
                                               rhs: LLVMValueRef);

build_op_str!(build_phi, llvm::LLVMBuildPhi, ty: LLVMTypeRef);
//build_call is manually defined in impl Builder
build_op_str!(build_select, llvm::LLVMBuildSelect, i: LLVMValueRef,
                                                   the: LLVMValueRef,
                                                   els: LLVMValueRef);

build_op_str!(build_vaarg, llvm::LLVMBuildVAArg, list: LLVMValueRef, ty: LLVMTypeRef);


build_op_str!(build_extract_element, llvm::LLVMBuildExtractElement, vec_val: LLVMValueRef,
                                                                    index: LLVMValueRef);
build_op_str!(build_insert_element, llvm::LLVMBuildInsertElement, vec_val: LLVMValueRef,
                                                                  eltval: LLVMValueRef,
                                                                  index: LLVMValueRef);
build_op_str!(build_shuffle_vector, llvm::LLVMBuildShuffleVector, v1: LLVMValueRef,
                                                                  v2: LLVMValueRef,
                                                                  mask: LLVMValueRef);

// TODO: Both these types use unsigned, change this to libc::unsigned
build_op_str!(build_extract_value, llvm::LLVMBuildExtractValue, aggval: LLVMValueRef,
                                                                index: u32);
build_op_str!(build_insert_value, llvm::LLVMBuildInsertValue, aggval: LLVMValueRef,
                                                              eltval: LLVMValueRef,
                                                              index: u32);

// TODO: LLVMBuildAtomicRMW

build_op_str!(build_is_null, llvm::LLVMBuildIsNull, val: LLVMValueRef);
build_op_str!(build_is_not_null, llvm::LLVMBuildIsNotNull, val: LLVMValueRef);
build_op_str!(build_ptr_diff, llvm::LLVMBuildPtrDiff, lhs: LLVMValueRef, rhs: LLVMValueRef);
build_op_str!(build_fence, llvm::LLVMBuildFence, ordering: LLVMAtomicOrdering,
                                                 singlethread: LLVMBool);




build_op!(build_ret, llvm::LLVMBuildRet, ret_val: LLVMValueRef);
build_op!(build_ret_void, llvm::LLVMBuildRetVoid,); // TODO: Fix the trailing comma
build_op!(build_br, llvm::LLVMBuildBr, dest: LLVMBasicBlockRef);

build_op!(build_cond_br, llvm::LLVMBuildCondBr, cond: LLVMValueRef,
                                                then: LLVMBasicBlockRef,
                                                else_: LLVMBasicBlockRef);




impl Builder {
    pub fn position_at_end(&mut self, basic_block: LLVMBasicBlockRef) {
        unsafe {
            llvm::LLVMPositionBuilderAtEnd(self.ptr, basic_block);
        }
    }

    pub fn build_call(&mut self, func: Function, mut args: Vec<LLVMValueRef>,
                      name: &str) -> LLVMValueRef {
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

    pub fn build_in_bounds_gep(&self, ptr: LLVMValueRef, mut indices: Vec<LLVMValueRef>,
                               name: &str) -> LLVMValueRef {
        let c_name = CString::new(name).unwrap();
        unsafe {
            llvm::LLVMBuildInBoundsGEP(self.ptr, ptr, indices.as_mut_ptr(),
                                       indices.len() as u32, c_name.as_ptr())
        }
    }
    pub fn build_gep(&self, ptr: LLVMValueRef, mut indices: Vec<LLVMValueRef>,
                               name: &str) -> LLVMValueRef {
        let c_name = CString::new(name).unwrap();
        unsafe {
            llvm::LLVMBuildGEP(self.ptr, ptr, indices.as_mut_ptr(),
                                       indices.len() as u32, c_name.as_ptr())
        }
    }
}

impl Drop for Builder {
    fn drop(&mut self) {
        unsafe {
            llvm::LLVMDisposeBuilder(self.ptr);
        }
    }
}

