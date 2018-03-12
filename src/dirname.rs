use std::str;
use path_parsing::{find_last_sep_pos, find_last_non_sep_pos};

pub fn dirname(path: &str) -> &str {
  let bytes = path.as_bytes();
  let mut last_slash_pos = match find_last_sep_pos(bytes) {
    Some(pos) => pos,
    _ => return ".",
  };
  // Skip trailing slashes.
  if last_slash_pos == bytes.len() - 1 {
    let last_non_slash_pos = match find_last_non_sep_pos(&bytes[..last_slash_pos]) {
      Some(pos) => pos,
      _ => return "/"
    };
    last_slash_pos = match find_last_sep_pos(&bytes[..last_non_slash_pos]) {
      Some(pos) => pos,
      _ => return "."
    };
  };
  if let Some(end) = find_last_non_sep_pos(&bytes[..last_slash_pos]) {
    &path[..end + 1]
  } else {
    "/"
  }
}

#[test]
fn absolute() {
  assert_eq!(dirname("/a/b///c"), "/a/b");
}

#[test]
fn trailing_slashes_absolute() {
  assert_eq!(dirname("/a/b///c//////"), "/a/b");
}

#[test]
fn relative() {
  assert_eq!(dirname("b///c"), "b");
}

#[test]
fn trailing_slashes_relative() {
  assert_eq!(dirname("b/c//"), "b");
}

#[test]
fn root() {
  assert_eq!(dirname("//c"), "/");
}

#[test]
fn trailing_slashes_root() {
  assert_eq!(dirname("//c//"), "/");
}

#[test]
fn trailing_slashes_relative_root() {
  assert_eq!(dirname("c//"), ".");
}

#[test]
fn returns_dot_for_empty_string() {
  assert_eq!(dirname(""), ".");
}
