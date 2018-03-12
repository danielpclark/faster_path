extern crate memchr;

use path_parsing::{find_last_sep_pos, find_last_non_sep_pos, find_last_dot_pos};

pub fn basename<'a>(path: &'a str, ext: &str) -> &'a str {
  let bytes: &[u8] = path.as_bytes();
  let mut left: usize = 0;
  let mut right: usize = bytes.len();
  if let Some(last_slash_pos) = find_last_sep_pos(bytes) {
    if last_slash_pos == right - 1 {
      if let Some(pos) = find_last_non_sep_pos(&bytes[..last_slash_pos]) {
        right = pos + 1;
      } else {
        return "/";
      }
      if let Some(pos) = find_last_sep_pos(&bytes[..right]) {
        left = pos + 1;
      }
    } else {
      left = last_slash_pos + 1;
    }
  }
  let ext_bytes = ext.as_bytes();
  if ext_bytes == b".*" {
    if let Some(dot_pos) = find_last_dot_pos(&bytes[left..right]) {
      right = left + dot_pos;
    }
  } else if bytes[left..right].ends_with(ext_bytes) {
    right -= ext_bytes.len();
  }
  &path[left..right]
}

#[test]
fn absolute() {
  assert_eq!(basename("/a/b///c", ""), "c");
}

#[test]
fn trailing_slashes_absolute() {
  assert_eq!(basename("/a/b///c//////", ""), "c");
}

#[test]
fn relative() {
  assert_eq!(basename("b///c", ""), "c");
}

#[test]
fn trailing_slashes_relative() {
  assert_eq!(basename("b/c//", ""), "c");
}

#[test]
fn root() {
  assert_eq!(basename("//c", ""), "c");
}

#[test]
fn trailing_slashes_root() {
  assert_eq!(basename("//c//", ""), "c");
}

#[test]
fn trailing_slashes_relative_root() {
  assert_eq!(basename("c//", ""), "c");
}

#[test]
fn edge_case_all_seps() {
  assert_eq!("/", basename("///", ".*"));
}
