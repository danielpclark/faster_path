use std::path::MAIN_SEPARATOR;
use libc::c_char;
use std::ffi::{CStr, CString};

#[no_mangle]
pub extern "C" fn extname(c_pth: *const c_char) -> *const c_char {
  if c_pth.is_null() {
    return c_pth
  }

  let name = unsafe { CStr::from_ptr(c_pth) }.to_str().unwrap()
    .trim_right_matches(MAIN_SEPARATOR)
    .rsplit(MAIN_SEPARATOR).next().unwrap_or("");

  if let Some(dot_i) = name.rfind('.') {
    if dot_i > 0 && dot_i < name.len() - 1 && name[..dot_i].chars().rev().next().unwrap() != '.' {
      return CString::new(&name[dot_i..]).unwrap().into_raw()
    }
  }

  CString::new("").unwrap().into_raw()
}
