// Copyright 2015-2016 Daniel P. Clark & Other FasterPath Developers
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.
#[macro_use]
extern crate ruru;

class!(FasterPathname);

mod helpers;
mod pathname;
mod basename;
mod chop_basename;
mod dirname;
mod extname;
mod plus;
pub mod rust_arch_bits;
mod path_parsing;

use ruru::{Class, Object, RString, Boolean, Array};

// r_ methods are on the core class and may evaluate instance variables or self
// pub_ methods must take all values as parameters
methods!(
  FasterPathname,
  _itself,

  fn pub_add_trailing_separator(pth: RString) -> RString {
    pathname::pn_add_trailing_separator(pth)
  }

  fn pub_is_absolute(pth: RString) -> Boolean {
    pathname::pn_is_absolute(pth)
  }

  // fn r_ascend(){}

  fn pub_basename(pth: RString, ext: RString) -> RString {
    pathname::pn_basename(pth, ext)
  }

  fn pub_children(pth: RString, with_dir: Boolean) -> Array {
    pathname::pn_children(pth, with_dir)
  }

  fn pub_children_compat(pth: RString, with_dir: Boolean) -> Array {
    pathname::pn_children_compat(pth, with_dir)
  }

  fn pub_chop_basename(pth: RString) -> Array {
    pathname::pn_chop_basename(pth)
  }

  // fn r_cleanpath(){ pub_cleanpath(r_to_path()) }
  // fn pub_cleanpath(pth: RString){}

  // fn r_cleanpath_aggressive(pth: RString){}

  // fn r_cleanpath_conservative(pth: RString){}

  // fn r_del_trailing_separator(pth: RString){}

  // fn r_descend(){}

  fn pub_is_directory(pth: RString) -> Boolean {
    pathname::pn_is_directory(pth)
  }

  fn pub_dirname(pth: RString) -> RString {
    pathname::pn_dirname(pth)
  }

  // fn r_each_child(){}
  // fn pub_each_child(){}

  // fn pub_each_filename(pth: RString) -> NilClass {
  //   pathname::pn_each_filename(pth)
  // }

  // pub_entries returns an array of String objects
  fn pub_entries(pth: RString) -> Array {
    pathname::pn_entries(pth)
  }

  // pub_entries_compat returns an array of Pathname objects
  fn pub_entries_compat(pth: RString) -> Array {
    pathname::pn_entries_compat(pth)
  }

  fn pub_extname(pth: RString) -> RString {
    pathname::pn_extname(pth)
  }

  // fn r_find(ignore_error: Boolean){}
  // fn pub_find(pth: RString ,ignore_error: Boolean){}
  
  fn pub_has_trailing_separator(pth: RString) -> Boolean {
    pathname::pn_has_trailing_separator(pth)
  }

  // fn r_join(args: Array){}

  // fn pub_mkpath(pth: RString) -> NilClass {
  //   pathname::pn_mkpath(pth)
  // }

  // fn r_is_mountpoint(){ pub_is_mountpount(r_to_path()) }
  // fn pub_is_mountpoint(pth: RString){}

  // fn r_parent(){ pub_parent(r_to_path()) }
  // fn pub_parent(pth: RString){}

  // also need impl +
  fn pub_plus(pth1: RString, pth2: RString) -> RString {
    pathname::pn_plus(pth1, pth2)
  }

  // fn r_prepend_prefix(prefix: RString, relpath: RString){}

  fn pub_is_relative(pth: RString) -> Boolean {
    pathname::pn_is_relative(pth)
  }

  // fn r_root(){ pub_root(r_to_path()) }
  // fn pub_root(pth: RString){}

  // fn r_split_names(pth: RString){}

  // fn r_relative_path_from(){}
  // fn pub_relative_path_from(){}

  // fn pub_rmtree(pth: RString) -> NilClass {
  //   pathname::pn_rmtree(pth)
  // }
);

#[allow(non_snake_case)]
#[no_mangle]
pub extern "C" fn Init_faster_pathname(){
  Class::new("FasterPathname", None).define(|itself| {
    itself.define_nested_class("Public", None);
  });

  // Public methods
  // * methods for refinements, monkeypatching
  // * methods that need all values as parameters
  Class::from_existing("FasterPathname").get_nested_class("Public").define(|itself| {
    itself.def("absolute?", pub_is_absolute);
    itself.def("add_trailing_separator", pub_add_trailing_separator);
    itself.def("basename", pub_basename);
    itself.def("children", pub_children);
    itself.def("children_compat", pub_children_compat);
    itself.def("chop_basename", pub_chop_basename);
    itself.def("directory?", pub_is_directory);
    itself.def("dirname", pub_dirname);
    itself.def("entries", pub_entries);
    itself.def("entries_compat", pub_entries_compat);
    itself.def("extname", pub_extname);
    itself.def("has_trailing_separator?", pub_has_trailing_separator);
    itself.def("plus", pub_plus);
    itself.def("relative?", pub_is_relative);
  });
}
