use std::path::MAIN_SEPARATOR;
use libc::c_char;
use std::ffi::{CStr, CString};
use std::str;

#[no_mangle]
pub extern "C" fn basename_for_chop(string: *const c_char) -> *const c_char {
    let c_str = unsafe {
        if string.is_null() {
            return string;
        }
        CStr::from_ptr(string)
    };

    let r_str = str::from_utf8(c_str.to_bytes()).unwrap();

    let mut offset = 0;
    let mut trailing_slashes = r_str.chars().rev();
    while let Some(MAIN_SEPARATOR) = trailing_slashes.next() {
        offset += 1
    }

    let r_str = &r_str[0..r_str.len() - offset];
    let part = r_str.rsplit_terminator(MAIN_SEPARATOR).nth(0).unwrap_or("");

    let output = CString::new(part).unwrap();
    output.into_raw()
}
