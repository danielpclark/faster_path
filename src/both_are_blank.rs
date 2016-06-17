use libc::c_char;
use std::ffi::{CStr};
use std::str;

#[no_mangle]
pub extern fn both_are_blank(s1: *const c_char, s2: *const c_char) -> bool {
  let c_str1 = unsafe {
    assert!(!s1.is_null());
    CStr::from_ptr(s1)
  };
  let c_str2 = unsafe {
    assert!(!s2.is_null());
    CStr::from_ptr(s2)
  };

  str::from_utf8(c_str1.to_bytes()).unwrap().trim().is_empty() &&
    str::from_utf8(c_str2.to_bytes()).unwrap().trim().is_empty()
}
