use std::ffi::CStr;
use std::os::raw::c_char;

/// Helper function for pt_image/pt_iscache names
pub(crate) fn name_ptr_to_option_string(name_ptr: *const c_char) -> Option<String> {
    if name_ptr.is_null() {
        None
    } else {
        Some(
            unsafe { CStr::from_ptr(name_ptr) }
                .to_str()
                .expect("Failed to convert C into Rust String, invalid UTF-8.")
                .to_owned(),
        )
    }
}
