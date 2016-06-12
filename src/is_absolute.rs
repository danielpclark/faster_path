#[no_mangle]
pub extern fn is_absolute(s: *const c_char) -> bool {
  let r_str = &RubyString::from_ruby(s);
  r_str.chars().next().unwrap_or("muffins".chars().next().unwrap()) == MAIN_SEPARATOR
}
