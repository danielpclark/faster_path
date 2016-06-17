use std::path::MAIN_SEPARATOR;
use libc::c_char;
use std::ffi::{CStr,CString};
use std::str;

#[no_mangle]
pub extern fn dirname_for_chop(string: *const c_char) -> *const c_char {
  let c_str = unsafe {
    assert!(!string.is_null());

    CStr::from_ptr(string)
  };

  let r_str = str::from_utf8(c_str.to_bytes()).unwrap();

  if r_str.is_empty() {
    return string
  }

  let mut offset = 0;
  let mut trailing_slashes = r_str.chars().rev();
  loop {
    match trailing_slashes.next() {
      Some(MAIN_SEPARATOR) => { offset = offset + 1 },
      _                    => { break               },
    }
  }
  
  let r_str = &r_str[0..r_str.len()-offset];

  let base = r_str.rsplit_terminator(MAIN_SEPARATOR).nth(0).unwrap_or("");

  let output = CString::new(&r_str[0..r_str.len()-base.len()]).unwrap();
  output.into_raw()
}
