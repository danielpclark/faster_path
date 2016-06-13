pub struct RubyString;

// Coercing strs into Strings has some loss of performance
// You may use these methods temporarily but it would be much better
// to write all of this code out in each method using just str.
//
// Using these methods you will still get you 10X (= 900%) performance
// gain regardless.  But really consider not using String.  If you do
// as I've intstructed the performance gains will go from 900% to 1250%.
impl RubyString {
  // FOR QUICK IMPLEMENTATION.  NOT FOR PRODUCTION.
  // SEE BENCHMARKS FOR SANCTIONED IMPLEMENTATION.
  fn from_ruby(s: *const c_char) -> String {
    let c_str = unsafe {
      assert!(!s.is_null());
      CStr::from_ptr(s)
    };
    (*str::from_utf8(c_str.to_bytes()).unwrap_or("")).to_string()
  }

  // FOR QUICK IMPLEMENTATION.  NOT FOR PRODUCTION.
  // SEE BENCHMARKS FOR SANCTIONED IMPLEMENTATION.
  fn to_ruby<S: Into<String>>(s: S) -> *const c_char {
    let r_str = s.into();
    let s_slice: &str = &r_str[..];
    CString::new(s_slice).unwrap().into_raw()
  }
}
