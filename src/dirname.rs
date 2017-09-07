use libc::c_char;
use std::ffi::{CStr, CString};

#[no_mangle]
pub extern "C" fn dirname(path: *const c_char) -> *const c_char {
  if path.is_null() {
    return path
  }
  let r_str = unsafe { CStr::from_ptr(path) }.to_str().unwrap();

  CString::new(rust::dirname(r_str)).unwrap().into_raw()
}

pub mod rust {
  use std::path::MAIN_SEPARATOR;
  use path_parsing::{last_sep_i, last_non_sep_i, last_non_sep_i_before};

  pub fn dirname(path: &str) -> String {
    let r_str = path;
    if r_str.is_empty() {
      return ".".to_string();
    }
    let non_sep_i = last_non_sep_i(r_str);
    if non_sep_i == -1 {
      return MAIN_SEPARATOR.to_string();
    }
    let sep_i = last_sep_i(r_str, non_sep_i);
    if sep_i == -1 {
      return ".".to_string();
    }
    if sep_i == 0 {
      return MAIN_SEPARATOR.to_string();
    }
    let non_sep_i2 = last_non_sep_i_before(r_str, sep_i);
    if non_sep_i2 != -1 {
      return r_str[..(non_sep_i2 + 1) as usize].to_string();
    } else {
      return MAIN_SEPARATOR.to_string();
    }
  }
}

#[test]
fn returns_dot_for_empty_string(){
  assert_eq!(rust::dirname(""), ".".to_string());
}
