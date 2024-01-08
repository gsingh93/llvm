

/// Convert CStr to str safely, for both static and dynamic strings.
/// Panics if ptr is not a CStr
// TODO Mark this as panicking
#[allow(unused_macros)]
macro_rules! c_str_to_str {
    ($s:expr) => {{
        use std::ffi::CStr;
        unsafe { CStr::from_ptr($s) }
            .to_str().expect("Convert CStr to str")
    }}
}













