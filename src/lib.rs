extern crate raygui_sys;

use std::ffi::CString;
use std::os::raw::c_char;
use raygui_sys::*;

trait AsNativeEquivalent {
    type Equivalent;
    fn as_native(&self) -> Self::Equivalent;
}

impl AsNativeEquivalent for &str {
    type Equivalent = *const c_char;
    fn as_native(&self) -> Self::Equivalent {
        CString::new(*self).unwrap().as_ptr()
    }
}

include!(concat!(env!("CARGO_MANIFEST_DIR"), "/autogen/bindings.rs"));
