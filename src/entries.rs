use libc::c_char;
use std::ffi::{CStr,CString};
use std::str;
use std::fs;
use ruby_array::RubyArray;

#[no_mangle]
pub extern fn entries(string: *const c_char) -> RubyArray {
    let c_str = unsafe {
        assert!(!string.is_null());

        CStr::from_ptr(string)
    };

    let r_str = str::from_utf8(c_str.to_bytes()).unwrap();

    let files = fs::read_dir(r_str).unwrap();
    let mut files_vec = vec![];

    files_vec.push(CString::new(".").unwrap().into_raw());
    files_vec.push(CString::new("..").unwrap().into_raw());

    for file in files {
        let file_name_str = file.unwrap().file_name().into_string().unwrap();
        let file_name_cstr = CString::new(file_name_str).unwrap().into_raw();
        files_vec.push(file_name_cstr);
    }

    RubyArray::from_vec(files_vec)
}
