use memchr::{memchr, memrchr};
use std::path::MAIN_SEPARATOR;

pub const SEP: u8 = MAIN_SEPARATOR as u8;
pub const SEP_BYTES: &[u8] = &[SEP];

// Returns the byte offset of the last byte that equals MAIN_SEPARATOR.
#[inline(always)]
pub fn find_last_sep_pos(bytes: &[u8]) -> Option<usize> {
  memrchr(SEP, bytes)
}

// Returns the byte offset of the last byte that is not MAIN_SEPARATOR.
#[inline(always)]
pub fn find_last_non_sep_pos(bytes: &[u8]) -> Option<usize> {
  bytes.iter().rposition(|&b| b != SEP)
}

// Whether the given byte sequence contains a MAIN_SEPARATOR.
#[inline(always)]
pub fn contains_sep(bytes: &[u8]) -> bool {
  memchr(SEP, bytes).is_some()
}

#[test]
fn it_finds_separators() {
  assert_eq!(find_last_sep_pos(b""), None);
  assert_eq!(find_last_sep_pos(b"a/b/c"), Some(3));
  assert_eq!(find_last_non_sep_pos(b""), None);
  assert_eq!(find_last_non_sep_pos(b"///"), None);
  assert_eq!(find_last_non_sep_pos(b"a///"), Some(0));
  assert_eq!(find_last_non_sep_pos(b"a/b/c/d/e/f/g/h/i/j/k//////////////////////"), Some(20));
  assert!(contains_sep(b"a/b"));
  assert!(!contains_sep(b"ab"));
}
