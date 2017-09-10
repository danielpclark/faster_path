use libc::c_char;
use std::ffi::{CStr};
use std::str;
use std::path::Path;

#[no_mangle]
pub extern "C" fn is_directory(string: *const c_char) -> bool {
  let c_str = unsafe {
    if string.is_null() {
      return false;
    }
    CStr::from_ptr(string)
  };

  let r_str = str::from_utf8(c_str.to_bytes()).unwrap_or("");

  Path::new(r_str).is_dir()
}
