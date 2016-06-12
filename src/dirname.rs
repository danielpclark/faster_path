#[no_mangle]
pub extern fn dirname(s: *const c_char) -> *const c_char {
  let r_str = &RubyString::from_ruby(s);
  if r_str.is_empty() {
    return s
  }

  let path = Path::new(r_str).parent().unwrap_or(Path::new(""));

  let out_str = if !path.to_str().unwrap().is_empty() {
    format!("{}{}", path.to_str().unwrap(), MAIN_SEPARATOR)
  } else {
    format!("{}", path.to_str().unwrap())
  };

  RubyString::to_ruby(out_str)
}
