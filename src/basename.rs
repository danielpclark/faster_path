use std::path::MAIN_SEPARATOR;
use libc::c_char;
use std::ffi::{CStr,CString};
use std::str;

#[allow(dead_code)]
fn rubyish_basename(string: &str, globish_string: &str) -> String {
  let result = if globish_string == ".*" {
    let base = string.rsplit_terminator(MAIN_SEPARATOR).nth(0).unwrap_or("");
    let index = base.rfind(".");
    let (first, _) = base.split_at(index.unwrap());
    first
  } else {
    if &string[string.len()-globish_string.len()..] == globish_string {
      &string[0..string.len()-globish_string.len()]
    } else {
      string
    }.rsplit_terminator(MAIN_SEPARATOR).nth(0).unwrap_or("")
  };
  result.to_string()
}

#[test]
fn it_chomps_strings_correctly(){
  assert_eq!(rubyish_basename("","")                           , "");
  assert_eq!(rubyish_basename("ruby","")                       , "ruby");
  assert_eq!(rubyish_basename("ruby.rb",".rb")                 , "ruby");
  assert_eq!(rubyish_basename("ruby.rb",".*")                  , "ruby");
  assert_eq!(rubyish_basename(".ruby/ruby.rb",".rb")           , "ruby");
  assert_eq!(rubyish_basename(".ruby/ruby.rb.swp",".rb")       , "ruby.rb.swp");
  assert_eq!(rubyish_basename(".ruby/ruby.rb.swp",".swp")      , "ruby.rb");
  assert_eq!(rubyish_basename(".ruby/ruby.rb.swp",".*")        , "ruby.rb");
  assert_eq!(rubyish_basename("asdf/asdf.rb.swp","")           , "asdf.rb.swp"); 
  assert_eq!(rubyish_basename("asdf/asdf.rb.swp", ".*")        , "asdf.rb"); 
  assert_eq!(rubyish_basename("asdf/asdf.rb.swp", "*")         , "asdf.rb.swp"); 
  assert_eq!(rubyish_basename("asdf/asdf.rb.swp", ".")         , "asdf.rb.swp"); 
  assert_eq!(rubyish_basename("asdf/asdf.rb.swp", ".*")        , "asdf.rb"); 
  assert_eq!(rubyish_basename("asdf/asdf.rb.swp", ".rb")       , "asdf.rb.swp"); 
  assert_eq!(rubyish_basename("asdf/asdf.rb.swp", ".swp")      , "asdf.rb"); 
  assert_eq!(rubyish_basename("asdf/asdf.rb.swp", ".sw")       , "asdf.rb.swp"); 
  assert_eq!(rubyish_basename("asdf/asdf.rb.swp", ".sw*")      , "asdf.rb.swp"); 
  assert_eq!(rubyish_basename("asdf/asdf.rb.swp", ".rb.s*")    , "asdf.rb.swp");
  assert_eq!(rubyish_basename("asdf/asdf.rb.swp", ".s*")       , "asdf.rb.swp");
  assert_eq!(rubyish_basename("asdf/asdf.rb.swp", ".s**")      , "asdf.rb.swp");
  assert_eq!(rubyish_basename("asdf/asdf.rb.swp", ".**")       , "asdf.rb.swp");
  assert_eq!(rubyish_basename("asdf/asdf.rb.swp", ".*")        , "asdf.rb");
  assert_eq!(rubyish_basename("asdf/asdf.rb.swp", ".*.*")      , "asdf.rb.swp");
  assert_eq!(rubyish_basename("asdf/asdf.rb.swp", ".rb.swp")   , "asdf");
  assert_eq!(rubyish_basename("asdf/asdf.rb.swp", ".rb.s*p")   , "asdf.rb.swp");
  assert_eq!(rubyish_basename("asdf/asdf.rb.swp", ".r*.s*p")   , "asdf.rb.swp");
  assert_eq!(rubyish_basename("asdf/asdf.rb.swp", ".r*.sw*p")  , "asdf.rb.swp");
  assert_eq!(rubyish_basename("asdf/asdf.rb.swp", ".r*b.sw*p") , "asdf.rb.swp");
  assert_eq!(rubyish_basename("asdf/asdf.rb.swp", "rb.swp")    , "asdf.");
}

#[no_mangle]
pub extern fn basename(str_pth: *const c_char, comp_ext: *const c_char) -> *const c_char {
  let c_str1 = unsafe {
    if str_pth.is_null(){
      return str_pth;
    } 
    CStr::from_ptr(str_pth)
  };
  let c_str2 = unsafe {
    if comp_ext.is_null() {
      return str_pth;
    }
    CStr::from_ptr(comp_ext)
  };
  let string         = str::from_utf8(c_str1.to_bytes()).unwrap();
  let globish_string = str::from_utf8(c_str2.to_bytes()).unwrap();

  let result = if globish_string == ".*" {
    let base = string.rsplit_terminator(MAIN_SEPARATOR).nth(0).unwrap_or("");
    let index = base.rfind(".");
    let (first, _) = base.split_at(index.unwrap());
    first
  } else {
    if &string[string.len()-globish_string.len()..] == globish_string {
      &string[0..string.len()-globish_string.len()]
    } else {
      string
    }.rsplit_terminator(MAIN_SEPARATOR).nth(0).unwrap_or("")
  };
  CString::new(result).unwrap().into_raw()
}
