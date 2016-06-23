use std::path::MAIN_SEPARATOR;
use libc::c_char;
use std::ffi::{CStr, CString};

#[no_mangle]
pub extern "C" fn extname(string: *const c_char) -> *const c_char {
    if string.is_null() {
        return string
    }
    let r_str = unsafe { CStr::from_ptr(string) }.to_str().unwrap_or("")
      .trim_right_matches(MAIN_SEPARATOR);

    let mut reversed_ext: Vec<u8> = Vec::new();
    let mut dot_index = 0;
    let mut separator_index = 0;
    for (pos, char) in r_str.char_indices().rev() {
        if char != MAIN_SEPARATOR {
            reversed_ext.push(char as u8);
            if char == '.' {
                dot_index = pos;
                break;
            }
        } else {
            separator_index = pos;
            break;
        }
    }

    if separator_index >= dot_index || reversed_ext.len() == 1 {
        return CString::new("").unwrap().into_raw();
    }
    reversed_ext.reverse();
    CString::new(reversed_ext).unwrap().into_raw()
}
