use crate::path_parsing::{find_last_sep_pos, find_last_non_sep_pos, SEP_BYTES};

pub fn dirname(path: &[u8]) -> &[u8] {
  let mut last_slash_pos = match find_last_sep_pos(path) {
    Some(pos) => pos,
    _ => return b".",
  };
  // Skip trailing slashes.
  if last_slash_pos == path.len() - 1 {
    let last_non_slash_pos = match find_last_non_sep_pos(&path[..last_slash_pos]) {
      Some(pos) => pos,
      _ => return SEP_BYTES,
    };
    last_slash_pos = match find_last_sep_pos(&path[..last_non_slash_pos]) {
      Some(pos) => pos,
      _ => return b".",
    };
  };
  if let Some(end) = find_last_non_sep_pos(&path[..last_slash_pos]) {
    &path[..end + 1]
  } else {
    SEP_BYTES
  }
}

#[cfg(test)]
fn dirname_str(path: &str) -> &str {
  std::str::from_utf8(dirname(path.as_bytes())).unwrap()
}

#[test]
fn absolute() {
  assert_eq!(dirname_str("/a/b///c"), "/a/b");
}

#[test]
fn trailing_slashes_absolute() {
  assert_eq!(dirname_str("/a/b///c//////"), "/a/b");
}

#[test]
fn relative() {
  assert_eq!(dirname_str("b///c"), "b");
}

#[test]
fn trailing_slashes_relative() {
  assert_eq!(dirname_str("b/c//"), "b");
}

#[test]
fn root() {
  assert_eq!(dirname_str("//c"), "/");
}

#[test]
fn trailing_slashes_root() {
  assert_eq!(dirname_str("//c//"), "/");
}

#[test]
fn trailing_slashes_relative_root() {
  assert_eq!(dirname_str("c//"), ".");
}

#[test]
fn returns_dot_for_empty_string() {
  assert_eq!(dirname_str(""), ".");
}

#[test]
fn only_separators() {
  assert_eq!(dirname_str("/"), "/");
  assert_eq!(dirname_str("///"), "/");
}

#[test]
fn non_utf8() {
  assert_eq!(dirname(b"\xff\xfe/a"), b"\xff\xfe");
}
