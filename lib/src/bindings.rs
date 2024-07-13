use std::ffi::{CStr, CString};
use std::os::raw::c_char;

#[no_mangle]
pub extern "C" fn hello_world() -> *mut c_char {
    let result = crate::hello_world();
    CString::new(result).unwrap().into_raw()
}

#[no_mangle]
pub extern "C" fn free_c_string(s: *mut c_char) {
    if s.is_null() { return; }
    unsafe { CString::from_raw(s) };
}
