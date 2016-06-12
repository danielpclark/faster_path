#[no_mangle]
pub extern fn basename(string: *const c_char) -> *const c_char {
  let c_str = unsafe {
    assert!(!string.is_null());

    CStr::from_ptr(string)
  };

  let r_str = str::from_utf8(c_str.to_bytes()).unwrap();

  let part = Path::new(r_str).file_name().unwrap_or(OsStr::new("")).to_str();
  
  let output = CString::new(format!("{}", part.unwrap())).unwrap();
  output.into_raw()
}
