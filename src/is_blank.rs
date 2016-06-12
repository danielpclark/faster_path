#[no_mangle]
pub extern fn is_blank(s: *const c_char) -> bool {
  RubyString::from_ruby(s).trim().is_empty()
}
