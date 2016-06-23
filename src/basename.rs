use std::path::MAIN_SEPARATOR;
use libc::c_char;
use std::ffi::{CStr, CString};

#[no_mangle]
pub extern "C" fn basename(c_pth: *const c_char, c_ext: *const c_char) -> *const c_char {
  // TODO: rb_raise on type or encoding errors
  // TODO: support objects that respond to `to_path`
  if c_pth.is_null() || c_ext.is_null() {
    return c_pth;
  }
  let pth = unsafe { CStr::from_ptr(c_pth) }.to_str().unwrap();
  let ext = unsafe { CStr::from_ptr(c_ext) }.to_str().unwrap();

  let mut name = pth.trim_right_matches(MAIN_SEPARATOR).rsplit(MAIN_SEPARATOR).next().unwrap_or("");

  if ext == ".*" {
    if let Some(dot_i) = name.rfind('.') {
      name = &name[0..dot_i];
    }
  } else if name.ends_with(ext) {
    name = &name[..name.len() - ext.len()];
  };

  CString::new(name).unwrap().into_raw()
}
