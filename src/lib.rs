// Copyright 2015-2016 Daniel P. Clark & Other FasterPath Developers
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.
extern crate libc;

#[macro_use]
extern crate ruru;

class!(FasterPathname);

pub mod free;
pub mod ruby_array;
pub mod is_directory;
pub mod is_relative;
pub mod is_blank;
pub mod both_are_blank;
pub mod basename;
pub mod plus;
pub mod dirname;
pub mod chop_basename;
pub mod basename_for_chop;
pub mod dirname_for_chop;
pub mod has_trailing_separator;
pub mod extname;
pub mod entries;
pub mod rust_arch_bits;
mod path_parsing;

use ruru::{Class, Object, RString, Boolean};
use std::path::MAIN_SEPARATOR;

methods!(
  FasterPathname,
  _itself,

  fn plus(pth1: RString, pth2: RString) -> RString {
    RString::new(&plus::plus_paths(pth1.ok().unwrap().to_str(), pth2.ok().unwrap().to_str())[..])
  }

  fn is_absolute(path: RString) -> Boolean {
    Boolean::new(match path.ok().unwrap().to_str().chars().next() {
      Some(c) => c == MAIN_SEPARATOR,
      None => false
    })
  }

  fn add_trailing_separator(path: RString) -> RString {
    let pth = path.ok().unwrap();

    let x = format!("{}{}", pth.to_str(), "a");

    match x.rsplit_terminator(MAIN_SEPARATOR).next() {
      Some("a") => pth,
      _ => RString::new(format!("{}{}", pth.to_str(), MAIN_SEPARATOR).as_str())
    }
  }
);

#[allow(non_snake_case)]
#[no_mangle]
pub extern "C" fn Init_faster_pathname(){
  Class::new("FasterPathname", None).define(|itself| {
    itself.def("plus", plus);
    itself.def("absolute?", is_absolute);
    itself.def("add_trailing_separator", add_trailing_separator);
  });
}
