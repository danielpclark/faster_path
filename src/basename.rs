#[no_mangle]
pub extern fn basename(s: *const c_char) -> *const c_char {
  let r_str = &RubyString::from_ruby(s);
  let part = Path::new(r_str).file_name().unwrap_or(OsStr::new("")).to_str().unwrap();
  RubyString::to_ruby(part)
}
