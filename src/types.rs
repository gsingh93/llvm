use std::fmt;
use std::mem::transmute;
use std::ops::Deref;

use llvm_sys::prelude::*;
use llvm_sys::core::*;
use llvm_sys::*;

use super::*;

/// Enumeration of all the base types of the LLVM type system.
#[allow(non_camel_case_types)]
#[derive(Debug)]
#[repr(C)]
pub enum Kind {
    Void = 0,
    Half,
    Float,
    Double,
    X86_FP80,
    FP128,
    PPC_FP128,
    Label,
    Integer,
    Function,
    Struct,
    Array,
    Pointer,
    Vector,
    Metadata,
    X86_MMX,
    Token,
}
}

/// Should always be used as `&Type`.
///
/// `Type`s are owned by `Context` instances such that only one instance of a
/// specific `Type` exists per `Context`, e.g. only 1 `Type` for `i64` exists
/// per context. `Type`s are also never mutated and never destroyed, living for
/// the lifetime of the `Context` that owns them.
// TODO: can this be made into an unsized type?
pub struct Type(LLVMType);

impl<'a> Into<&'a Type> for LLVMTypeRef {
    fn into(self) -> &'a Type {
        unsafe { transmute::<LLVMTypeRef, &Type>(self) }
    }
}

impl<'a> From<&'a Type> for LLVMTypeRef {
    fn from(ty: &'a Type) -> LLVMTypeRef {
        unsafe { transmute::<&Type, LLVMTypeRef>(ty) }
    }
}

impl Type {
    pub fn kind(&self) -> Kind {
        unsafe { transmute(LLVMGetTypeKind(self.into())) }
    }

    pub fn is_sized(&self) -> bool {
        unsafe { LLVMTypeIsSized(self.into()) == 1 }
    }
}

// This counts as the llvm::Type::print method from the C++ API, though the C++
// version has more options.
impl fmt::Display for Type {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        unsafe {
            let s_ptr = LLVMPrintTypeToString(self.into());
            let r = write!(f, "{}", c_str_to_str!(s_ptr));
            LLVMDisposeMessage(s_ptr);
            r
        }
    }
}

impl fmt::Debug for Type {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "llvm::Type({})", self)
    }
}

impl PartialEq for Type {
    fn eq(&self, other: &Self) -> bool {
        LLVMTypeRef::from(self) == LLVMTypeRef::from(other)
    }
}

impl Eq for Type {}

macro_rules! impl_type {
    ($t:ty) => {
        impl<'a> Into<&'a $t> for LLVMTypeRef {
            fn into(self) -> &'a $t {
                unsafe { transmute::<LLVMTypeRef, &$t>(self) }
            }
        }

        impl<'a> From<&'a $t> for LLVMTypeRef {
            fn from(ty: &'a $t) -> LLVMTypeRef {
                unsafe { transmute::<&$t, LLVMTypeRef>(ty) }
            }
        }

        impl Deref for $t {
            type Target = Type;
            
            fn deref(&self) -> &Self::Target {
                unsafe { transmute::<&Self, &Self::Target>(self) }
            }
        }
    }
}

// Base types:

pub struct Void(Type);
impl_type!(Void);

pub struct Half(Type);
impl_type!(Half);

pub struct Float(Type);
impl_type!(Float);

pub struct Double(Type);
impl_type!(Double);

#[allow(non_camel_case_types)]
pub struct X86_FP80(Type);
impl_type!(X86_FP80);

#[allow(non_camel_case_types)]
pub struct FP128(Type);
impl_type!(FP128);

#[allow(non_camel_case_types)]
pub struct PPC_FP128(Type);
impl_type!(PPC_FP128);

pub struct Label(Type);
impl_type!(Label);

pub struct Metadata(Type);
impl_type!(Metadata);

#[allow(non_camel_case_types)]
pub struct X86_MMX(Type);
impl_type!(X86_MMX);

pub struct Token(Type);
impl_type!(Token);

/// Integer types are constructed with a size. Construct them with the methods
/// that `Context` provides.
pub struct Integer(Type);
impl_type!(Integer);

impl Integer {
    pub fn width(&self) -> u32 {
        unsafe { LLVMGetIntTypeWidth(self.into()) }
    }
}

/// A function type is a tuple consisting of a return type and an array of
/// parameter types.
pub struct Function(Type);
impl_type!(Function);

impl Function {
    pub fn new<'a>(
        return_type: &'a Type,
        param_types: &[&'a Type],
        is_var_args: bool,
    ) -> &'a Function {
        unsafe {
            LLVMFunctionType(
                return_type.into(),
                transmute::<*const &Type, *mut LLVMTypeRef>(param_types.as_ptr()),
                param_types.len() as u32,
                is_var_args as LLVMBool,
            ).into()
        }
    }
}

pub struct Struct(Type);
impl_type!(Struct);

pub struct Array(Type);
impl_type!(Array);

pub struct Pointer(Type);
impl_type!(Pointer);

pub struct Vector(Type);
impl_type!(Vector);

/// Represents a LLVM Context Type
pub trait ContextType {
    fn get_type_in_context<'a>(context: &'a Context) -> &'a Type;
}

macro_rules! impl_context_type {
    ($t: ty, $to_type_in_context: ident) => {
        impl ContextType for $t {
            fn get_type_in_context<'a>(context: &'a Context) -> &'a Type {
                unsafe {
                    $to_type_in_context(context.ptr).into()
                }
            }
        }
    }
}

impl_context_type!(bool, LLVMInt1TypeInContext);
// This might actually not be true, Not sure
impl_context_type!(char, LLVMInt8TypeInContext);
impl_context_type!(u8, LLVMInt8TypeInContext);
impl_context_type!(u16, LLVMInt16TypeInContext);
impl_context_type!(u32, LLVMInt32TypeInContext);
impl_context_type!(u64, LLVMInt64TypeInContext);
impl_context_type!(i8, LLVMInt8TypeInContext);
impl_context_type!(i16, LLVMInt16TypeInContext);
impl_context_type!(i32, LLVMInt32TypeInContext);
impl_context_type!(i64, LLVMInt64TypeInContext);
impl_context_type!(f32, LLVMFloatTypeInContext);
impl_context_type!(f64, LLVMDoubleTypeInContext);
//TODO: Function Types
//TODO: Structure Types
//TODO: Sequential Types
//TODO: Other Types
