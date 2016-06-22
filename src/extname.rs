use std::path::MAIN_SEPARATOR;
use libc::c_char;
use std::ffi::{CStr, CString};
use std::str;

#[no_mangle]
pub extern "C" fn extname(string: *const c_char) -> *const c_char {
    let c_str = unsafe {
        if string.is_null() {
          return string
        }
        CStr::from_ptr(string)
    };

    let r_str = str::from_utf8(c_str.to_bytes()).unwrap_or("");

    let mut chars_iter = r_str.char_indices().rev();

    let mut reversed_ext: Vec<u8> = Vec::new();
    let mut dot_index = 0;
    let mut separator_index = 0;
    loop {
        let char = chars_iter.next();
        if char.is_some() {
            if char.unwrap().1 != MAIN_SEPARATOR {
                if char.unwrap().1 == '.' {
                    dot_index = char.unwrap().0;
                    reversed_ext.push(char.unwrap().1 as u8);
                    break;
                } else {
                    reversed_ext.push(char.unwrap().1 as u8);
                }
            } else {
                separator_index = char.unwrap().0;
                break;
            }
        } else {
            break;
        }
    }

    if separator_index >= dot_index {
        return CString::new("").unwrap().into_raw();
    }

    if reversed_ext.len() == 1 {
        return CString::new("").unwrap().into_raw();
    }

    reversed_ext.reverse();
    let output = CString::new(reversed_ext).unwrap();
    output.into_raw()
}
