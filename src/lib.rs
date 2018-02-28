// Copyright 2015-2016 Daniel P. Clark & Other FasterPath Developers
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.
#[macro_use]
extern crate ruru;

#[macro_use]
extern crate lazy_static;

module!(FasterPath);

mod helpers;
mod pathname;
mod basename;
mod chop_basename;
mod cleanpath_aggressive;
mod cleanpath_conservative;
mod dirname;
mod extname;
mod pathname_sys;
mod plus;
mod prepend_prefix;
pub mod rust_arch_bits;
mod path_parsing;

use ruru::{Module, Object, RString, Boolean, Array, AnyObject};

use pathname_sys::*;

methods!(
  FasterPath,
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

  fn pub_children(pth: RString, with_dir: Boolean) -> AnyObject {
    pathname::pn_children(pth, with_dir)
  }

  fn pub_children_compat(pth: RString, with_dir: Boolean) -> AnyObject {
    pathname::pn_children_compat(pth, with_dir)
  }

  fn pub_chop_basename(pth: RString) -> Array {
    pathname::pn_chop_basename(pth)
  }

  // fn r_cleanpath(){ pub_cleanpath(r_to_path()) }
  // fn pub_cleanpath(pth: RString){}

  fn pub_cleanpath_aggressive(pth: RString) -> RString {
    pathname::pn_cleanpath_aggressive(pth)
  }

  fn pub_cleanpath_conservative(pth: RString) -> RString {
    pathname::pn_cleanpath_conservative(pth)
  }

  fn pub_del_trailing_separator(pth: RString) -> RString {
    pathname::pn_del_trailing_separator(pth)
  }

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
  fn pub_entries(pth: RString) -> AnyObject {
    pathname::pn_entries(pth)
  }

  // pub_entries_compat returns an array of Pathname objects
  fn pub_entries_compat(pth: RString) -> AnyObject {
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
  Module::from_existing("FasterPath").define(|itself| {
    itself.def_self("absolute?", pub_is_absolute);
    itself.def_self("add_trailing_separator", pub_add_trailing_separator);
    itself.def_self("del_trailing_separator", pub_del_trailing_separator);
    itself.def_self("cleanpath_aggressive", pub_cleanpath_aggressive);
    itself.def_self("cleanpath_conservative", pub_cleanpath_conservative);
    itself.def_self("directory?", pub_is_directory);
    itself.def_self("dirname", pub_dirname);
    itself.def_self("extname", pub_extname);
    itself.def_self("has_trailing_separator?", pub_has_trailing_separator);
    //itself.def_self("join", pub_join);
    pathname_sys::define_singleton_method(itself.value(), "join", pub_join);
    itself.def_self("plus", pub_plus);
    itself.def_self("relative?", pub_is_relative);
    itself.define_nested_class("Public", None);
  });

  // For methods requiring addition Ruby-side behavior
  Module::from_existing("FasterPath").get_nested_class("Public").define(|itself| {
    itself.def_self("basename", pub_basename);
    itself.def_self("children", pub_children);
    itself.def_self("children_compat", pub_children_compat);
    itself.def_self("chop_basename", pub_chop_basename);
    itself.def_self("entries", pub_entries);
    itself.def_self("entries_compat", pub_entries_compat);
  });
}
