#![feature(test)]
// PLAYGROUND FOR PERFORMANCE TESTS
//
extern crate libc;
extern crate test;
use test::Bencher;

use libc::c_char;
use std::ffi::{CStr,CString};
use std::str;

#[bench]
fn rust_to_ruby_c_char_convenient(b: &mut Bencher){
  let s: String = "hello".to_string();

  // (185ns)
  b.iter(|| {
    let s = s.clone(); //To imitate .into()
    let s_slice: &str = &s[..];
    CString::new(s_slice).unwrap().into_raw()
  })
}

// SANCTIONED USAGE
#[bench]
fn rust_to_ruby_c_char(b: &mut Bencher){
  let s: String = "hello".to_string();
  let s = s.as_str();

  // USE THIS METHOD!!! (153ns)
  b.iter(|| {
    CString::new(s).unwrap().into_raw()
  })
}


#[bench]
fn ruby_to_rust_string_convenient(b: &mut Bencher){
  let s: *const c_char = CString::new("hello").unwrap().into_raw();

  // (46ns)
  b.iter(|| {
    let c_str = unsafe {
      assert!(!s.is_null());
      CStr::from_ptr(s)
    };
    (*str::from_utf8(c_str.to_bytes()).unwrap_or("")).to_string()
  })
}


// SANCTIONED USAGE
#[bench]
fn ruby_to_rust_from_c_char_from_utf8(b: &mut Bencher){
  let s: *const c_char = CString::new("hello").unwrap().into_raw();

  // USE THIS METHOD!!! (19ns)
  b.iter(|| {
    let c_str = unsafe {
      if s.is_null() {
        return "";
      }
      CStr::from_ptr(s)
    };
    str::from_utf8(c_str.to_bytes()).unwrap_or("")
  })
}


#[bench]
fn ruby_to_rust_from_c_char_to_str(b: &mut Bencher) {
  let s: *const c_char = CString::new("hello").unwrap().into_raw();
  b.iter(|| {
    if s.is_null() {
      return "";
    }
    unsafe { CStr::from_ptr(s) }.to_str().unwrap_or("")
  });
}
