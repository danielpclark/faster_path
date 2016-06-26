use libc::c_char;
use std::ffi::{CStr, CString};
use path_parsing::extract_last_path_segment;

#[no_mangle]
pub extern "C" fn extname(c_pth: *const c_char) -> *const c_char {
  if c_pth.is_null() {
    return c_pth
  }

  let name = extract_last_path_segment(unsafe { CStr::from_ptr(c_pth) }.to_str().unwrap());

  if let Some(dot_i) = name.rfind('.') {
    if dot_i > 0 && dot_i < name.len() - 1 && name[..dot_i].chars().rev().next().unwrap() != '.' {
      return CString::new(&name[dot_i..]).unwrap().into_raw()
    }
  }

  CString::new("").unwrap().into_raw()
}
