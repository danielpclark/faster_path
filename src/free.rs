use libc::c_char;
use std::ffi::CString;
// use ruby_array::RubyArray;

#[no_mangle]
pub extern "C" fn free_string(s: *mut c_char) {
  unsafe {
    if s.is_null() { return }
    CString::from_raw(s)
  };
}

// #[no_mangle]
// pub extern "C" fn free_array(ra: *mut RubyArray) {
//   if ra.is_null() { return }
//   unsafe { Box::from_raw(ra); }
// }
