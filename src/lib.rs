// Copyright 2015-2016 Daniel P. Clark & Other FasterPath Developers
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.
extern crate libc;
extern crate regex;

use std::path::{Path,MAIN_SEPARATOR};
use libc::c_char;
use std::ffi::{CStr,CString,OsStr};
use std::str;
use std::mem;

fn pathish_chomp_regex(string: &str, globish_string: &str) -> String {
  string.to_string()
}

#[test]
fn it_chomps_strings_correctly(){
  assert_eq!(pathish_chomp_regex("",""), "");
  assert_eq!(pathish_chomp_regex("ruby",""), "ruby");
  assert_eq!(pathish_chomp_regex("ruby.rb",".rb"), "ruby");
  assert_eq!(pathish_chomp_regex("ruby.rb",".*"), "ruby");
  assert_eq!(pathish_chomp_regex(".ruby/ruby.rb",".rb"), ".ruby/ruby");
  assert_eq!(pathish_chomp_regex(".ruby/ruby.rb.swp",".rb"), ".ruby/ruby.rb.swp");
  assert_eq!(pathish_chomp_regex(".ruby/ruby.rb.swp",".swp"), ".ruby/ruby.rb");
  assert_eq!(pathish_chomp_regex(".ruby/ruby.rb.swp",".*"), ".ruby/ruby.rb");
}


include!("ruby_string.rs");
include!("ruby_array.rs");
include!("is_absolute.rs");
include!("is_relative.rs");
include!("is_blank.rs");
include!("both_are_blank.rs");
include!("basename.rs");
include!("dirname.rs");
include!("basename_for_chop.rs");
include!("dirname_for_chop.rs");

// EXAMPLE
//
//#[no_mangle]
//pub extern fn one_and_two() -> RubyArray {
//  let mut words = vec![];
//  words.push(RubyString::to_ruby(&"one".to_string()));
//  words.push(RubyString::to_ruby(&"two".to_string()));
//  RubyArray::from_vec(words)
//}
