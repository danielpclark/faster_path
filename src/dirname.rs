use std::path::MAIN_SEPARATOR;
use libc::c_char;
use std::ffi::{CStr,CString};
use std::str;

#[no_mangle]
pub extern "C" fn dirname(string: *const c_char) -> *const c_char {
    let c_str = unsafe {
        if string.is_null() {
            return string
        }

        CStr::from_ptr(string)
    };

    let r_str = str::from_utf8(c_str.to_bytes()).unwrap();

    if r_str.is_empty() {

        return CString::new(".").unwrap().into_raw();
    }

    let last_none_slash = r_str.char_indices()
        .rev()
        .find(|&(_, char)| char != MAIN_SEPARATOR);

    if !last_none_slash.is_some() {
        return CString::new("/").unwrap().into_raw();
    }

    let most_right_slash = r_str[0..last_none_slash.unwrap().0]
        .char_indices()
        .rev()
        .find(|&(_, char)| char == MAIN_SEPARATOR);

    if most_right_slash.is_some() {

        if most_right_slash.unwrap().0 == 0 {

            return CString::new("/").unwrap().into_raw();
        }

        let path = &r_str[0..most_right_slash.unwrap().0];

        if path == &MAIN_SEPARATOR.to_string() {

            return CString::new("/").unwrap().into_raw();
        }

        if !path.ends_with(&MAIN_SEPARATOR.to_string()) {

            return CString::new(path).unwrap().into_raw();
        }

        let path_last_none_slash = path.char_indices()
            .rev()
            .find(|&(_, char)| char != MAIN_SEPARATOR);

        if !path_last_none_slash.is_some() {

            return CString::new("/").unwrap().into_raw();
        }


        return CString::new(&path[0..(path_last_none_slash.unwrap().0 + 1)]).unwrap().into_raw();
    } else {

        return CString::new(".").unwrap().into_raw();
    }
}
