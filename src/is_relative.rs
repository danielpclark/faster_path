use std::path::MAIN_SEPARATOR;
use libc::c_char;
use std::ffi::CStr;
use std::str;

#[no_mangle]
pub extern "C" fn is_relative(string: *const c_char) -> bool {
    let c_str = unsafe {
        if string.is_null() {
            return false;
        }
        CStr::from_ptr(string)
    };

    let r_str = str::from_utf8(c_str.to_bytes()).unwrap_or("");

    r_str.chars().next().unwrap_or("muffins".chars().next().unwrap()) != MAIN_SEPARATOR
}
