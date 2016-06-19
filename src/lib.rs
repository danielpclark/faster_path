// Copyright 2015-2016 Daniel P. Clark & Other FasterPath Developers
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.
extern crate libc;

pub mod ruby_string;
pub mod ruby_array;
pub mod is_absolute;
pub mod is_directory;
pub mod is_relative;
pub mod is_blank;
pub mod both_are_blank;
pub mod basename;
pub mod dirname;
pub mod basename_for_chop;
pub mod dirname_for_chop;
pub mod add_trailing_separator;
pub mod has_trailing_separator;
pub mod extname;

// EXAMPLE
//
//#[no_mangle]
//pub extern fn one_and_two() -> RubyArray {
//  let mut words = vec![];
//  words.push(RubyString::to_ruby(&"one".to_string()));
//  words.push(RubyString::to_ruby(&"two".to_string()));
//  RubyArray::from_vec(words)
//}
