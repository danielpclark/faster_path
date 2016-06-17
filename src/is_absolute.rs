use libc::c_char;
use std::ffi::{CStr};
use std::str;
use std::path::MAIN_SEPARATOR;


#[no_mangle]
pub extern fn is_absolute(string: *const c_char) -> bool {
  let c_str = unsafe {
    assert!(!string.is_null());

    CStr::from_ptr(string)
  };

  let r_str = str::from_utf8(c_str.to_bytes()).unwrap_or("");

  r_str.chars().next().unwrap_or("muffins".chars().next().unwrap()) == MAIN_SEPARATOR
}
