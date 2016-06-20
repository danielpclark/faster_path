use std::path::{Path, MAIN_SEPARATOR};
use libc::c_char;
use std::ffi::CStr;
use std::str;

#[no_mangle]
pub extern "C" fn has_trailing_separator(string: *const c_char) -> bool {
    let c_str = unsafe {
        if string.is_null() {
            return false
        }
        CStr::from_ptr(string)
    };

    let r_str = str::from_utf8(c_str.to_bytes()).unwrap_or("");
    let path = Path::new(r_str);
    let last_component = path.iter().last();
    if last_component.is_none() {
        false
    } else {
        let mut parts: Vec<&str> = r_str.rsplit_terminator(MAIN_SEPARATOR).collect();
        parts.retain(|x| !x.is_empty());
        let last_part = parts.first().unwrap_or(&"").chars().last().unwrap_or(MAIN_SEPARATOR);
        let last_char = r_str.chars().last().unwrap();
        (last_part != last_char) && (last_char == MAIN_SEPARATOR)
    }
}
