// TODO: This could be named better: impl from
// TODO Documentation
macro_rules! configure_wrapper {
    ($dest: tt, $ref: ty) => {
        impl From<$ref> for $dest {
            fn from(ptr: $ref) -> Self {
                $dest { ptr }
            }
        }

        impl From<$dest> for $ref {
            fn from(s: $dest) -> Self { s.ptr }
        }
    }
}

// TODO Documentation
// TODO DEBUG
macro_rules! impl_from {
    ($llvm: ty, $wrap: ty) => {
        impl<'a> From<$llvm> for &'a $wrap {
            fn from(ptr: $llvm) -> &'a $wrap {
                unsafe { std::mem::transmute::<$llvm, &$wrap>(ptr) }
            }
        }
        impl<'a> From<&'a $wrap> for $llvm {
            fn from(typ: &'a $wrap) -> $llvm {
                unsafe { std::mem::transmute::<&$wrap, $llvm>(typ) }
            }
        }
        impl<'a> From<$llvm> for &'a mut $wrap {
            fn from(ptr: $llvm) -> &'a mut $wrap {
                unsafe { std::mem::transmute::<$llvm, &mut $wrap>(ptr) }
            }
        }
        impl<'a> From<&'a mut $wrap> for $llvm {
            fn from(typ: &'a mut $wrap) -> $llvm {
                unsafe { std::mem::transmute::<&mut $wrap, $llvm>(typ) }
            }
        }
    }
}

// TODO Documentation
macro_rules! impl_eq {
    ($llvm: tt, $wrap: ty) => {
        impl PartialEq for $wrap {
            fn eq(&self, other: &Self) -> bool {
                $llvm::from(self) == $llvm::from(other)
            }
        }

        impl Eq for $wrap {}
    }
}

// TODO Documentation
macro_rules! impl_fmt {
    ($wrap: ty, $to_string: ident) => {
        impl ::std::fmt::Debug for $wrap {
            fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "llvm::{}({})", stringify!($wrap), self)
            }
        }

        impl ::std::fmt::Display for $wrap {
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
