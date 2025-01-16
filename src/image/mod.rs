#[expect(clippy::module_inception)]
mod image;
mod iscache;

pub use image::*;
pub use iscache::*;

use crate::error::{PtError, PtErrorCode};
use std::ffi::{CStr, CString};
use std::os::raw::c_char;

/// Helper function for `pt_image`/`pt_iscache` names
fn name_ptr_to_option_string(name_ptr: *const c_char) -> Option<String> {
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

fn str_to_cstring_pterror(s: &str) -> Result<CString, PtError> {
    CString::new(s).map_err(|_| {
        PtError::new(
            PtErrorCode::Invalid,
            "invalid string: it contains null bytes",
        )
    })
}
