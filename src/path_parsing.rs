extern crate memchr;

use self::memchr::{memchr, memrchr};
use memrnchr::memrnchr;
use std::path::MAIN_SEPARATOR;
use std::str;
use std::ops::Range;

pub const SEP: u8 = MAIN_SEPARATOR as u8;
lazy_static! {
  pub static ref SEP_STR: &'static str = str::from_utf8(&[SEP]).unwrap();
}

#[allow(dead_code)]
pub fn find_last_word(bytes: &[u8]) -> Range<usize> {
  let len = bytes.len();
  let mut r: Range<usize> = Range { start: len, end: len };
  let mut word = false;
  for (pos, &b) in bytes.iter().enumerate().rev() {
    match b {
      SEP => {
        if !word {
          r.end = pos;
        } else {
          if bytes.get(pos + 1).is_some() {
            r.start = pos + 1;
          }
          break
        }
      },
      _ => {
        word = true;
        r.start = pos;
      }
    }
  }

  if r.start > r.end {
    r.start = r.end
  }

  r
}

#[test]
fn test_find_last_word() {
  let r = find_last_word(&"/asdf/".as_bytes());
  assert_eq!(r, Range { start: 1, end: 5 });

  let r = find_last_word(&"asdf/asdf/asdf".as_bytes());
  assert_eq!(r, Range { start: 10, end: 14});

  let r = find_last_word(&"../asdf/..".as_bytes());
  assert_eq!(r, Range { start: 8, end: 10 });

  let r = find_last_word(&"../asdf/..asdf".as_bytes());
  assert_eq!(r, Range { start: 8, end: 14 });

  let r = find_last_word(&"/////////".as_bytes());
  assert_eq!(r, Range { start: 0, end: 0 });
}

// Returns the byte offset of the last byte that equals MAIN_SEPARATOR.
#[allow(dead_code)]
#[inline(always)]
pub fn find_last_sep_pos(bytes: &[u8]) -> Option<usize> {
  memrchr(SEP, bytes)
}

// Returns the byte offset of the last byte that is not MAIN_SEPARATOR.
#[inline(always)]
pub fn find_last_non_sep_pos(bytes: &[u8]) -> Option<usize> {
  memrnchr(SEP, bytes)
}

// For clarification in use cases
#[test]
fn last_non_sep_pos() {
  assert_eq!(find_last_non_sep_pos(&"..".as_bytes()), Some(1));
}

// Whether the given byte sequence contains a MAIN_SEPARATOR.
#[inline(always)]
pub fn contains_sep(bytes: &[u8]) -> bool {
  memchr(SEP, bytes) != None
}
