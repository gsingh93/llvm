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
pub enum Kind<'a> {
    Void(&'a Void),
    Half(&'a Half),
    Float(&'a Float),
    Double(&'a Double),
    X86_FP80(&'a X86_FP80),
    FP128(&'a FP128),
    PPC_FP128(&'a PPC_FP128),
    Label(&'a Label),
    Integer(&'a Integer),
    Function(&'a Function),
    Struct(&'a Struct),
    Array(&'a Array),
    Pointer(&'a Pointer),
    Vector(&'a Vector),
    Metadata(&'a Metadata),
    X86_MMX(&'a X86_MMX),
    Token(&'a Token),
}

/// A generic LLVM type. Should always be used as `&Type`.
///
/// `Type`s are owned by `Context` instances such that only one instance of a
/// specific `Type` exists per `Context`, e.g. only 1 `Type` for `i64` exists
/// per context. `Type`s are also never mutated and never destroyed, living for
/// the lifetime of the `Context` that owns them.
///
/// In LLVM, there is a heirarchy of types. An `&Type` can be downcast to a
/// subtype, such as `&Integer`, with the `downcast` method. Downcasting makes
/// subtype specific methods, such as `width` on `&Integer` available.
// TODO: mark this as an unsized type
pub struct Type(LLVMType);

impl<'a> From<LLVMTypeRef> for &'a Type {
    fn from(ptr: LLVMTypeRef) -> &'a Type {
        unsafe { transmute::<LLVMTypeRef, &Type>(ptr) }
    }
}

impl<'a> From<&'a Type> for LLVMTypeRef {
    fn from(ty: &'a Type) -> LLVMTypeRef {
        unsafe { transmute::<&Type, LLVMTypeRef>(ty) }
    }
}

macro_rules! try_as_fns {
            pub fn $name<'a>(&'a self) -> Option<&'a $variant> {
                if let Kind::$variant(t) = self.downcast() {
                    Some(t)
                }
                else {
                    None
                }
            }
        )*
    }
}

impl Type {
    pub fn is_sized(&self) -> bool {
        unsafe { LLVMTypeIsSized(self.into()) == 1 }
    }

    pub fn downcast(&self) -> Kind {
        unsafe {
            match LLVMGetTypeKind(self.into()) {
                LLVMTypeKind::LLVMVoidTypeKind => Kind::Void(transmute(self)),
                LLVMTypeKind::LLVMHalfTypeKind => Kind::Half(transmute(self)),
                LLVMTypeKind::LLVMFloatTypeKind => Kind::Float(transmute(self)),
                LLVMTypeKind::LLVMDoubleTypeKind => Kind::Double(transmute(self)),
                LLVMTypeKind::LLVMX86_FP80TypeKind => Kind::X86_FP80(transmute(self)),
                LLVMTypeKind::LLVMFP128TypeKind => Kind::FP128(transmute(self)),
                LLVMTypeKind::LLVMPPC_FP128TypeKind => Kind::PPC_FP128(transmute(self)),
                LLVMTypeKind::LLVMLabelTypeKind => Kind::Label(transmute(self)),
                LLVMTypeKind::LLVMIntegerTypeKind => Kind::Integer(transmute(self)),
                LLVMTypeKind::LLVMFunctionTypeKind => Kind::Function(transmute(self)),
                LLVMTypeKind::LLVMStructTypeKind => Kind::Struct(transmute(self)),
                LLVMTypeKind::LLVMArrayTypeKind => Kind::Array(transmute(self)),
                LLVMTypeKind::LLVMPointerTypeKind => Kind::Pointer(transmute(self)),
                LLVMTypeKind::LLVMVectorTypeKind => Kind::Vector(transmute(self)),
                LLVMTypeKind::LLVMMetadataTypeKind => Kind::Metadata(transmute(self)),
                LLVMTypeKind::LLVMX86_MMXTypeKind => Kind::X86_MMX(transmute(self)),
                LLVMTypeKind::LLVMTokenTypeKind => Kind::Token(transmute(self)),
            }
        }
    }

    try_as_fns! {
        pub fn try_as_void -> Void
        pub fn try_as_half -> Half
        pub fn try_as_float -> Float
        pub fn try_as_double -> Integer
        pub fn try_as_x86_fp80 -> X86_FP80
        pub fn try_as_fp128 -> FP128
        pub fn try_as_ppc_fp128 -> PPC_FP128
        pub fn try_as_label -> Label
        pub fn try_as_integer -> Integer
        pub fn try_as_function -> Function
        pub fn try_as_struct -> Struct
        pub fn try_as_array -> Array
        pub fn try_as_pointer -> Pointer
        pub fn try_as_vector -> Vector
        pub fn try_as_metadata -> Metadata
        pub fn try_as_x86_mmx -> X86_MMX
        pub fn try_as_token -> Token
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
        impl Deref for $t {
            type Target = Type;
            
            fn deref(&self) -> &Self::Target {
                unsafe { transmute::<&Self, &Self::Target>(self) }
            }
        }

        // This would not be needed if the compiler could infer that the
        // From<&Type> for LLVMTypeRef above worked on &SubType.
        impl<'a> From<&'a $t> for LLVMTypeRef {
            fn from(ty: &'a $t) -> LLVMTypeRef {
                unsafe { transmute::<&$t, LLVMTypeRef>(ty) }
            }
        }

        impl<'a> From<LLVMTypeRef> for &'a $t {
            fn from(ptr: LLVMTypeRef) -> &'a $t {
                unsafe { transmute::<LLVMTypeRef, &$t>(ptr) }
            }
        }

        impl fmt::Display for $t {
            fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                self.deref().fmt(f)
            }
}

        impl fmt::Debug for $t {
            fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                write!(f, "llvm::types::{}({})", stringify!($t), self)
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

/// Trait marking Rust types that have LLVM counterparts
pub trait ContextType {
    type LlvmType;

    fn get_type_in_context<'a>(context: &'a Context) -> &'a Self::LlvmType;
}

macro_rules! impl_context_type {
    ($t: ty => $llvm_type: ty, $to_type_in_context: ident) => {
        impl ContextType for $t {
            type LlvmType = $llvm_type;

            fn get_type_in_context<'a>(context: &'a Context) -> &'a Self::LlvmType {
                unsafe {
                    $to_type_in_context(context.ptr).into()
                }
            }
        }
    }
}

impl_context_type!(bool => Integer, LLVMInt1TypeInContext);
// This might actually not be true, Not sure
impl_context_type!(char => Integer, LLVMInt8TypeInContext);
impl_context_type!(u8 => Integer, LLVMInt8TypeInContext);
impl_context_type!(u16 => Integer, LLVMInt16TypeInContext);
impl_context_type!(u32 => Integer, LLVMInt32TypeInContext);
impl_context_type!(u64 => Integer, LLVMInt64TypeInContext);
impl_context_type!(i8 => Integer, LLVMInt8TypeInContext);
impl_context_type!(i16 => Integer, LLVMInt16TypeInContext);
impl_context_type!(i32 => Integer, LLVMInt32TypeInContext);
impl_context_type!(i64 => Integer, LLVMInt64TypeInContext);
impl_context_type!(f32 => Float, LLVMFloatTypeInContext);
impl_context_type!(f64 => Double, LLVMDoubleTypeInContext);
//TODO: Function Types
//TODO: Structure Types
//TODO: Sequential Types
//TODO: Other Types
