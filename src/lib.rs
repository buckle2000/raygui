extern crate raygui_sys;

use std::ffi::{CString, CStr};
use std::os::raw::c_char;
use raygui_sys::*;

trait AsNativeEquivalent {
    type Equivalent;
    fn as_native(&self) -> Self::Equivalent;
}

impl AsNativeEquivalent for &str {
    type Equivalent = *const c_char;
    #[inline]
    fn as_native(&self) -> Self::Equivalent {
        let cstr = CString::new(*self).unwrap();
        cstr.as_ptr()
    }
}

include!(concat!(env!("CARGO_MANIFEST_DIR"), "/autogen/bindings.rs"));

#[inline]
pub fn text_box(bounds: impl Into<Rectangle>, text: &mut String, max_text_size: i32, edit_mode: bool) -> bool {
    unsafe {
        let mut cstr = vec![0;max_text_size as usize];
        cstr.copy_from_slice(CString::new(text.as_str()).unwrap().to_bytes());
        let result = GuiTextBox(bounds.into(), cstr.as_mut_ptr() as *mut i8, max_text_size, edit_mode);
        text.clear();
        text.push_str(CStr::from_bytes_with_nul(&cstr).unwrap().to_str().unwrap());
        result
    }
}

#[inline]
pub fn text_box_multi(bounds: impl Into<Rectangle>, text: &mut String, max_text_size: i32, edit_mode: bool) -> bool {
    unsafe {
        let mut cstr = vec![0;max_text_size as usize];
        cstr.copy_from_slice(CString::new(text.as_str()).unwrap().to_bytes());
        let result = GuiTextBoxMulti(bounds.into(), cstr.as_mut_ptr() as *mut c_char, max_text_size, edit_mode);
        text.clear();
        text.push_str(CStr::from_bytes_with_nul(&cstr).unwrap().to_str().unwrap());
        result
    }
}

#[inline]
pub fn list_view_ex(bounds: impl Into<Rectangle>, options: &[&str], enabled: &mut i32, active: &mut i32, focus: &mut i32, scroll_index: &mut i32, edit_mode: bool) -> bool {
    unsafe {
        let cstrs: Vec<*const c_char> = options.iter().map(|x| x.as_native()).collect();
        GuiListViewEx(bounds.into(), cstrs.as_ptr() as *mut *mut c_char, cstrs.len() as i32, enabled as *mut i32, active as *mut i32, focus as *mut i32, scroll_index as *mut i32, edit_mode)
    }
}

#[inline]
pub fn load_style_props(props: &[i32]) {
    unsafe {
        GuiLoadStyleProps(props.as_ptr(), props.len() as i32)
    }
}
