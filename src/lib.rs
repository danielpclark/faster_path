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

include!("is_absolute.rs");
include!("is_relative.rs");
include!("is_blank.rs");
include!("basename.rs");
include!("dirname.rs");
