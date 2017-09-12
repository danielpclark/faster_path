// Copyright 2015-2016 Daniel P. Clark & Other FasterPath Developers
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.
#[macro_use]
extern crate ruru;

class!(FasterPathname);

mod basename;
mod chop_basename;
mod dirname;
mod extname;
mod plus;
pub mod rust_arch_bits;
mod path_parsing;

use ruru::{Class, Object, RString, Boolean, Array};
use std::path::{MAIN_SEPARATOR,Path};
use std::fs;

methods!(
  FasterPathname,
  _itself,

  fn r_add_trailing_separator(pth: RString) -> RString {
    let p = pth.ok().unwrap();
    let x = format!("{}{}", p.to_str(), "a");
    match x.rsplit_terminator(MAIN_SEPARATOR).next() {
      Some("a") => p,
      _ => RString::new(format!("{}{}", p.to_str(), MAIN_SEPARATOR).as_str())
    }
  }

  fn r_is_absolute(pth: RString) -> Boolean {
    Boolean::new(match pth.ok().unwrap_or(RString::new("")).to_str().chars().next() {
      Some(c) => c == MAIN_SEPARATOR,
      None => false
    })
  }

  fn r_basename(pth: RString, ext: RString) -> RString {
    RString::new(
      &basename::basename(
        pth.ok().unwrap_or(RString::new("")).to_str(),
        ext.ok().unwrap_or(RString::new("")).to_str()
      )[..]
    )
  }

  fn r_chop_basename(pth: RString) -> Array {
    let mut arr = Array::with_capacity(2);
    let results = chop_basename::chop_basename(pth.ok().unwrap_or(RString::new("")).to_str());
    match results {
      Some((dirname, basename)) => {
        arr.push(RString::new(&dirname[..]));
        arr.push(RString::new(&basename[..]));
        arr
      },
      None => arr
    }
  }

  fn r_is_directory(pth: RString) -> Boolean {
    Boolean::new(
      Path::new(
        pth.ok().unwrap_or(RString::new("")).to_str()
      ).is_dir()
    )
  }

  fn r_dirname(pth: RString) -> RString {
    RString::new(
      &dirname::dirname(
        pth.ok().unwrap_or(RString::new("")).to_str()
      )[..]
    )
  }

  fn r_entries(pth: RString) -> Array {
    let files = fs::read_dir(pth.ok().unwrap_or(RString::new("")).to_str()).unwrap();
    let mut arr = Array::new();

    arr.push(RString::new("."));
    arr.push(RString::new(".."));

    for file in files {
      let file_name_str = file.unwrap().file_name().into_string().unwrap();
      arr.push(RString::new(&file_name_str[..]));
    }

    arr
  }

  fn r_extname(pth: RString) -> RString {
    RString::new(
      &extname::extname(pth.ok().unwrap_or(RString::new("")).to_str())[..]
    )
  }

  fn r_has_trailing_separator(pth: RString) -> Boolean {
    let v = pth.ok().unwrap_or(RString::new(""));
    match chop_basename::chop_basename(v.to_str()) {
      Some((a,b)) => {
        Boolean::new(a.len() + b.len() < v.to_str().len())
      },
      _ => Boolean::new(false)
    }
  }

  fn r_plus(pth1: RString, pth2: RString) -> RString {
    RString::new(&plus::plus_paths(pth1.ok().unwrap().to_str(), pth2.ok().unwrap().to_str())[..])
  }

  fn r_is_relative(pth: RString) -> Boolean {
    Boolean::new(
      match pth.ok().unwrap_or(RString::new(&MAIN_SEPARATOR.to_string()[..])).to_str().chars().next() {
        Some(c) => c != MAIN_SEPARATOR,
        None => true
      }
    )
  }
);

#[allow(non_snake_case)]
#[no_mangle]
pub extern "C" fn Init_faster_pathname(){
  Class::new("FasterPathname", None).define(|itself| {
    itself.def("absolute?", r_is_absolute);
    itself.def("add_trailing_separator", r_add_trailing_separator);
    itself.def("basename", r_basename);
    itself.def("chop_basename", r_chop_basename);
    itself.def("directory?", r_is_directory);
    itself.def("dirname", r_dirname);
    itself.def("entries", r_entries);
    itself.def("extname", r_extname);
    itself.def("has_trailing_separator?", r_has_trailing_separator);
    itself.def("plus", r_plus);
    itself.def("relative?", r_is_relative);
  });
}
