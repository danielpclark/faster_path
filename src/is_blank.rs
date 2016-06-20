use libc::c_char;
use std::ffi::CStr;
use std::str;

#[no_mangle]
pub extern "C" fn is_blank(string: *const c_char) -> bool {
    let c_str = unsafe {
        if string.is_null() {
            return true;
        }
        CStr::from_ptr(string)
    };

    str::from_utf8(c_str.to_bytes()).unwrap().trim().is_empty()
}
