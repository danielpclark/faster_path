extern crate memchr;

use self::memchr::{memchr, memrchr};
use memrnchr::memrnchr;
use std::path::MAIN_SEPARATOR;
use std::str;

pub const SEP: u8 = MAIN_SEPARATOR as u8;
lazy_static! {
  pub static ref SEP_STR: &'static str = str::from_utf8(&[SEP]).unwrap();
}

// Returns the byte offset of the last byte that equals MAIN_SEPARATOR.
#[inline(always)]
pub fn find_last_sep_pos(bytes: &[u8]) -> Option<usize> {
  memrchr(SEP, bytes)
}

// Returns the byte offset of the last byte that is not MAIN_SEPARATOR.
#[inline(always)]
pub fn find_last_non_sep_pos(bytes: &[u8]) -> Option<usize> {
  memrnchr(SEP, bytes)
}

// Whether the given byte sequence contains a MAIN_SEPARATOR.
#[inline(always)]
pub fn contains_sep(bytes: &[u8]) -> bool {
  memchr(SEP, bytes) != None
}
