extern crate memchr;
use self::memchr::memrchr;
use std::path::MAIN_SEPARATOR;
use std::str;

pub const SEP: u8 = MAIN_SEPARATOR as u8;
// TODO: Do not hard-code "/". How can we convert MAIN_SEPARATOR char to a static string here?
pub static SEP_STR: &'static str = "/";

pub fn extract_last_path_segment(path: &str) -> &str {
  let end = (last_non_sep_i(path) + 1) as usize;
  &path[memrchr(SEP, &path.as_bytes()[..end]).unwrap_or(0)..end]
}

// Returns the byte offset of the last byte preceding a MAIN_SEPARATOR.
pub fn last_non_sep_i(path: &str) -> isize {
  last_non_sep_i_before(path, path.len() as isize - 1)
}

// Returns the byte offset of the last byte preceding a MAIN_SEPARATOR before the given end offset.
pub fn last_non_sep_i_before(path: &str, end: isize) -> isize {
  // Works with bytes directly because MAIN_SEPARATOR is always in the ASCII 7-bit range so we can
  // avoid the overhead of full UTF-8 processing.
  let ptr = path.as_ptr();
  let mut i = end;
  while i >= 0 {
    if unsafe { *ptr.offset(i) } != SEP { break; };
    i -= 1;
  }
  i
}
