use libc::c_char;
use std::ffi::{CStr,CString};
use std::{str,ptr,fs,mem};
use ruby_array::RubyArray;

#[no_mangle]
pub extern "C" fn entries(string: *const c_char, r_ptr: *mut u8) -> usize {
  let c_str = unsafe {
    assert!(!string.is_null());

    CStr::from_ptr(string)
  };

  let r_str = str::from_utf8(c_str.to_bytes()).unwrap();

  let files = fs::read_dir(r_str).unwrap();
  let mut files_vec = vec![];

  files_vec.push(CString::new(".").unwrap().into_raw());
  files_vec.push(CString::new("..").unwrap().into_raw());

  for file in files {
    let file_name_str = file.unwrap().file_name().into_string().unwrap();
    let file_name_cstr = CString::new(file_name_str).unwrap().into_raw();
    files_vec.push(file_name_cstr);
  }

  let box_x = Box::new(RubyArray::from_vec(files_vec));
  let size_x = mem::size_of_val(&box_x) as usize;
  let ptr_x = Box::into_raw(box_x) as *mut RubyArray;
  unsafe { ptr::write(r_ptr, ptr_x as u8); }
  size_x
}
