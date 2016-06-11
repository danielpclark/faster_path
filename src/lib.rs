// Copyright 2015-2016 Daniel P. Clark & Other Combinatorics Developers
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.
extern crate libc;

use std::path::{Path,MAIN_SEPARATOR};
use libc::c_char;
use std::ffi::{CStr,CString,OsStr};
use std::str;

#[no_mangle]
pub extern fn is_absolute(string: *const c_char) -> bool {
  let c_str = unsafe {
    assert!(!string.is_null());

    CStr::from_ptr(string)
  };

  let r_str = str::from_utf8(c_str.to_bytes()).unwrap_or("");

  r_str.chars().next().unwrap_or("muffins".chars().next().unwrap()) == MAIN_SEPARATOR
}

#[no_mangle]
pub extern fn is_relative(string: *const c_char) -> bool {
  let c_str = unsafe {
    assert!(!string.is_null());

    CStr::from_ptr(string)
  };

  let r_str = str::from_utf8(c_str.to_bytes()).unwrap_or("");

  r_str.chars().next().unwrap_or("muffins".chars().next().unwrap()) != MAIN_SEPARATOR
}

#[no_mangle]
pub extern fn is_blank(string: *const c_char) -> bool {
  let c_str = unsafe {
    assert!(!string.is_null());

    CStr::from_ptr(string)
  };

  str::from_utf8(c_str.to_bytes()).unwrap().trim().is_empty()
}

#[no_mangle]
pub extern fn basename(string: *const c_char) -> *const c_char {
  let c_str = unsafe {
    assert!(!string.is_null());

    CStr::from_ptr(string)
  };

  let r_str = str::from_utf8(c_str.to_bytes()).unwrap();

  let part = Path::new(r_str).file_name().unwrap_or(OsStr::new("")).to_str();
  
  let output = CString::new(format!("{}", part.unwrap())).unwrap();
  output.into_raw()
}

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
