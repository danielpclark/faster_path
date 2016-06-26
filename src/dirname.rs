use std::path::MAIN_SEPARATOR;
use libc::c_char;
use std::ffi::{CStr, CString};
use path_parsing::{last_sep_i, last_non_sep_i, last_non_sep_i_before};

#[no_mangle]
pub extern "C" fn dirname(path: *const c_char) -> *const c_char {
  if path.is_null() {
    return path
  }
  let r_str = unsafe { CStr::from_ptr(path) }.to_str().unwrap();
  if r_str.is_empty() {
    return CString::new(".").unwrap().into_raw();
  }
  let non_sep_i = last_non_sep_i(r_str);
  if non_sep_i == -1 {
    return CString::new(MAIN_SEPARATOR.to_string()).unwrap().into_raw();
  }
  let sep_i = last_sep_i(r_str, non_sep_i);
  if sep_i == -1 {
    return CString::new(".").unwrap().into_raw();
  }
  if sep_i == 0 {
    return CString::new(MAIN_SEPARATOR.to_string()).unwrap().into_raw();
  }
  let non_sep_i2 = last_non_sep_i_before(r_str, sep_i);
  if non_sep_i2 != -1 {
    return CString::new(&r_str[..(non_sep_i2 + 1) as usize]).unwrap().into_raw();
  } else {
    return CString::new(MAIN_SEPARATOR.to_string()).unwrap().into_raw();
  }
}
