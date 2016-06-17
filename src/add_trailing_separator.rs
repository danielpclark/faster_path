use std::path::{Path, MAIN_SEPARATOR};
use libc::c_char;
use std::ffi::{CStr, CString};
use std::str;

#[no_mangle]
pub extern "C" fn add_trailing_separator(string: *const c_char) -> *const c_char {
    let c_str = unsafe {
        assert!(!string.is_null());

        CStr::from_ptr(string)
    };
    let r_str = str::from_utf8(c_str.to_bytes()).unwrap_or("");

    if r_str.is_empty() {
        return string;
    }

    let path = Path::new(r_str);
    let out_str = if !(path.to_str().unwrap().chars().last().unwrap() == '/') {
        format!("{}{}", path.to_str().unwrap(), MAIN_SEPARATOR)
    } else {
        path.to_str().unwrap().to_string()
    };

    let output = CString::new(out_str).unwrap();
    output.into_raw()
}
