pub struct RubyString;

impl RubyString {
  fn from_ruby(s: *const c_char) -> String {
    let c_str = unsafe {
      assert!(!s.is_null());
      CStr::from_ptr(s)
    };
    (*str::from_utf8(c_str.to_bytes()).unwrap_or("")).to_string()
  }

  fn to_ruby<S: Into<String>>(s: S) -> *const c_char {
    let r_str = s.into();
    let s_slice: &str = &r_str[..];
    CString::new(s_slice).unwrap().into_raw()
  }
}
