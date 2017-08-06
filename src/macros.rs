// TODO: This could be named better
macro_rules! impl_llvm_ref {
    ($dest: tt, $ref: ty) => {
        impl From<$ref> for $dest {
            fn from(ptr: $ref) -> Self {
                $dest {
                    ptr: ptr,
                }
            }
        }

        impl From<$dest> for $ref {
            fn from(s: $dest) -> Self {
                s.ptr
            }
        }
    }
}

macro_rules! impl_llvm_type_wrapper {
    ($llvm: ty, $our: ty) => {
        impl<'a> From<$llvm> for &'a $our {
            fn from(ptr: $llvm) -> &'a $our {
                unsafe { ::std::mem::transmute::<$llvm, &$our>(ptr) }
            }
        }

        impl<'a> From<&'a $our> for $llvm {
            fn from(ty: &'a $our) -> $llvm {
                unsafe { ::std::mem::transmute::<&$our, $llvm>(ty) }
            }
        }

        impl<'a> From<$llvm> for &'a mut $our {
            fn from(ptr: $llvm) -> &'a mut $our {
                unsafe { ::std::mem::transmute::<$llvm, &mut $our>(ptr) }
            }
        }

        impl<'a> From<&'a mut $our> for $llvm {
            fn from(ty: &'a mut $our) -> $llvm {
                unsafe { ::std::mem::transmute::<&mut $our, $llvm>(ty) }
            }
        }
    }
}

macro_rules! impl_llvm_type_eq {
    ($llvm: tt, $our: ty) => {
        impl PartialEq for $our {
            fn eq(&self, other: &Self) -> bool {
                $llvm::from(self) == $llvm::from(other)
            }
        }

        impl Eq for $our {}
    }
}

macro_rules! impl_llvm_type_fmt {
    ($our: ty, $to_string: ident) => {
        impl ::std::fmt::Debug for $our {
            fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "llvm::{}({})", stringify!($our), self)
            }
        }

        impl ::std::fmt::Display for $our {
            fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                unsafe {
                    let s_ptr = $to_string(self.into());
                    let r = write!(f, "{}", c_str_to_str!(s_ptr));
                    LLVMDisposeMessage(s_ptr);
                    r
                }
            }
        }
    }
}
