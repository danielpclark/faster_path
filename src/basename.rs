extern crate array_tool;
use libc::c_char;
use std::ffi::{CStr, CString};
use path_parsing::extract_last_path_segment;
use self::array_tool::string::Squeeze;

#[no_mangle]
pub extern "C" fn basename(c_pth: *const c_char, c_ext: *const c_char) -> *const c_char {
  if c_pth.is_null() || c_ext.is_null() {
    return c_pth;
  }
  let pth = unsafe { CStr::from_ptr(c_pth) }.to_str().unwrap();
  let ext = unsafe { CStr::from_ptr(c_ext) }.to_str().unwrap();

  // Known edge case
  match &pth.squeeze("/")[..] {
    "/" => { return CString::new("/").unwrap().into_raw() }
    _ => {}
  }

  let mut name = extract_last_path_segment(pth);

  if ext == ".*" {
    if let Some(dot_i) = name.rfind('.') {
      name = &name[0..dot_i];
    }
  } else if name.ends_with(ext) {
    name = &name[..name.len() - ext.len()];
  };

  CString::new(name).unwrap().into_raw()
}
