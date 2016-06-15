#[no_mangle]
pub extern fn basename(string: *const c_char, comp_ext: *const c_char) -> *const c_char {
  let c_str1 = unsafe {
    assert!(!string.is_null());
    CStr::from_ptr(string)
  };
  let c_str2 = unsafe {
    assert!(!comp_ext.is_null());
    CStr::from_ptr(comp_ext)
  };
  let r_str = str::from_utf8(c_str1.to_bytes()).unwrap();
  let r_str_chomp = str::from_utf8(c_str2.to_bytes()).unwrap();

  let r_str = if !r_str_chomp.is_empty() {
    chomp_pathish_regex(r_str, r_str_chomp)
  } else { r_str.to_string() };

  let part = Path::new(&r_str[..]).file_name().unwrap_or(OsStr::new("")).to_str();
  
  let output = CString::new(format!("{}", part.unwrap())).unwrap();
  output.into_raw()
}
