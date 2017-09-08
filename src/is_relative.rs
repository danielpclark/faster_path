use std::path::MAIN_SEPARATOR;
use libc::c_char;
use std::ffi::{CStr};

#[no_mangle]
pub extern "C" fn is_relative(path: *const c_char) -> bool {
  if path.is_null() {
    return false;
  }
  let r_str = unsafe { CStr::from_ptr(path) }.to_str().unwrap();

  match r_str.chars().next() {
    Some(c) => c != MAIN_SEPARATOR,
    None => true
  }
}
