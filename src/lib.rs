// Copyright 2015-2016 Daniel P. Clark & Other FasterPath Developers
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.
#![forbid(unsafe_op_in_unsafe_fn)]

#[macro_use]
mod ruby;
mod helpers;
mod pathname;
mod basename;
mod chop_basename;
mod cleanpath_aggressive;
mod cleanpath_conservative;
mod dirname;
mod extname;
mod plus;
mod prepend_prefix;
pub mod rust_arch_bits;
mod path_parsing;
mod relative_path_from;

use rutie::{Class, Module, Object, RString};

ruby_methods! {
  fn pub_add_trailing_separator(arguments) {
    pathname::pn_add_trailing_separator(arguments)
  }

  fn pub_is_absolute(arguments) {
    pathname::pn_is_absolute(arguments)
  }

  fn pub_basename(arguments) {
    pathname::pn_basename(arguments)
  }

  fn pub_children(arguments) {
    pathname::pn_children(arguments)
  }

  fn pub_children_compat(arguments) {
    pathname::pn_children_compat(arguments)
  }

  fn pub_chop_basename(arguments) {
    pathname::pn_chop_basename(arguments)
  }

  fn pub_cleanpath_aggressive(arguments) {
    pathname::pn_cleanpath_aggressive(arguments)
  }

  fn pub_cleanpath_conservative(arguments) {
    pathname::pn_cleanpath_conservative(arguments)
  }

  fn pub_del_trailing_separator(arguments) {
    pathname::pn_del_trailing_separator(arguments)
  }

  fn pub_is_directory(arguments) {
    pathname::pn_is_directory(arguments)
  }

  fn pub_dirname(arguments) {
    pathname::pn_dirname(arguments)
  }

  // pub_entries returns an array of String objects
  fn pub_entries(arguments) {
    pathname::pn_entries(arguments)
  }

  // pub_entries_compat returns an array of Pathname objects
  fn pub_entries_compat(arguments) {
    pathname::pn_entries_compat(arguments)
  }

  fn pub_extname(arguments) {
    pathname::pn_extname(arguments)
  }

  fn pub_has_trailing_separator(arguments) {
    pathname::pn_has_trailing_separator(arguments)
  }

  fn pub_join(arguments) {
    pathname::pn_join(arguments)
  }

  fn pub_plus(arguments) {
    pathname::pn_plus(arguments)
  }

  fn pub_is_relative(arguments) {
    pathname::pn_is_relative(arguments)
  }

  fn pub_relative_path_from(arguments) {
    pathname::pn_relative_path_from(arguments)
  }
}

#[allow(non_snake_case)]
#[no_mangle]
pub extern "C" fn Init_faster_pathname() {
  let trailing_dot_extname = Class::file().
    protect_send("extname", &[RString::new_utf8("a.").into()]).
    ok().
    and_then(|extname| extname.try_convert_to::<RString>().ok()).
    map_or(false, |extname| extname.to_bytes_unchecked() == b".");
  extname::set_trailing_dot_is_extname(trailing_dot_extname);

  Module::from_existing("FasterPath").define(|itself| {
    itself.def_self("absolute?", pub_is_absolute);
    itself.def_self("add_trailing_separator", pub_add_trailing_separator);
    itself.def_self("del_trailing_separator", pub_del_trailing_separator);
    itself.def_self("chop_basename", pub_chop_basename);
    itself.def_self("cleanpath_aggressive", pub_cleanpath_aggressive);
    itself.def_self("cleanpath_conservative", pub_cleanpath_conservative);
    itself.def_self("directory?", pub_is_directory);
    itself.def_self("dirname", pub_dirname);
    itself.def_self("entries", pub_entries);
    itself.def_self("entries_compat", pub_entries_compat);
    itself.def_self("extname", pub_extname);
    itself.def_self("has_trailing_separator?", pub_has_trailing_separator);
    itself.def_self("join", pub_join);
    itself.def_self("plus", pub_plus);
    itself.def_self("relative?", pub_is_relative);
    itself.def_self("relative_path_from", pub_relative_path_from);
    itself.define_nested_class("Public", None);
  });

  // For methods requiring addition Ruby-side behavior
  Module::from_existing("FasterPath").get_nested_class("Public").define(|itself| {
    itself.def_self("basename", pub_basename);
    itself.def_self("children", pub_children);
    itself.def_self("children_compat", pub_children_compat);
  });
}
