#[no_mangle]
pub extern fn dirname(string: *const c_char) -> *const c_char {
  let c_str = unsafe {
    assert!(!string.is_null());

    CStr::from_ptr(string)
  };

  let r_str = str::from_utf8(c_str.to_bytes()).unwrap();

  if r_str.is_empty() {
    return string
  }

  let path = Path::new(r_str).parent().unwrap_or(Path::new(""));

  let out_str = if !path.to_str().unwrap().is_empty() {
    format!("{}{}", path.to_str().unwrap(), MAIN_SEPARATOR)
  } else {
    format!("{}", path.to_str().unwrap())
  };

  let output = CString::new(out_str).unwrap();
  output.into_raw()
}
