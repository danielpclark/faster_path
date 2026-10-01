use std::sync::atomic::{AtomicBool, Ordering};

use crate::path_parsing::{find_last_non_sep_pos, find_last_sep_pos};

// Ruby 2.7 changed `File.extname("foo.")` from `""` to `"."`. `FasterPath`
// follows the Ruby it is loaded into; see `Init_faster_pathname`.
static TRAILING_DOT_IS_EXTNAME: AtomicBool = AtomicBool::new(false);

pub fn set_trailing_dot_is_extname(value: bool) {
  TRAILING_DOT_IS_EXTNAME.store(value, Ordering::Relaxed);
}

pub fn extname(path: &[u8]) -> &[u8] {
  extname_with(path, TRAILING_DOT_IS_EXTNAME.load(Ordering::Relaxed))
}

// Ruby's `ruby_enc_find_extname` for non-Windows platforms.
fn extname_with(path: &[u8], trailing_dot_is_extname: bool) -> &[u8] {
  // The last component, without trailing separators
  let end = match find_last_non_sep_pos(path) {
    Some(pos) => pos + 1,
    None => return b"",
  };
  let start = find_last_sep_pos(&path[..end]).map_or(0, |pos| pos + 1);
  let name = &path[start..end];

  // Leading dots don't start an extension
  let leading_dots = name.iter().take_while(|&&c| c == b'.').count();
  match name[leading_dots..].iter().rposition(|&c| c == b'.') {
    None => b"",
    Some(pos) => {
      let dot = leading_dots + pos;
      if dot + 1 == name.len() && !trailing_dot_is_extname {
        b""
      } else {
        &name[dot..]
      }
    }
  }
}

#[cfg(test)]
fn extname_str(path: &str, trailing_dot_is_extname: bool) -> &str {
  std::str::from_utf8(extname_with(path.as_bytes(), trailing_dot_is_extname)).unwrap()
}

#[test]
fn it_finds_the_extension() {
  for &trailing_dot in &[false, true] {
    assert_eq!(extname_str("", trailing_dot), "");
    assert_eq!(extname_str("/", trailing_dot), "");
    assert_eq!(extname_str("a.rb", trailing_dot), ".rb");
    assert_eq!(extname_str("a.rb/", trailing_dot), ".rb");
    assert_eq!(extname_str("a.rb//", trailing_dot), ".rb");
    assert_eq!(extname_str("a/b.c.d", trailing_dot), ".d");
    assert_eq!(extname_str("a..b", trailing_dot), ".b");
    assert_eq!(extname_str(".a.b", trailing_dot), ".b");
    assert_eq!(extname_str("a/.rb", trailing_dot), "");
    assert_eq!(extname_str("a.b/c", trailing_dot), "");
    assert_eq!(extname_str(".bashrc", trailing_dot), "");
    assert_eq!(extname_str("..foo", trailing_dot), "");
    assert_eq!(extname_str("a/..b", trailing_dot), "");
    assert_eq!(extname_str("...a", trailing_dot), "");
  }
}

#[test]
fn only_dots() {
  for &trailing_dot in &[false, true] {
    assert_eq!(extname_str(".", trailing_dot), "");
    assert_eq!(extname_str("..", trailing_dot), "");
    assert_eq!(extname_str("....", trailing_dot), "");
    assert_eq!(extname_str("/.", trailing_dot), "");
    assert_eq!(extname_str(".a/", trailing_dot), "");
  }
}

#[test]
fn trailing_dot() {
  assert_eq!(extname_str("foo.", false), "");
  assert_eq!(extname_str(".foo.", false), "");
  assert_eq!(extname_str("foo..", false), "");
  assert_eq!(extname_str("a./", false), "");
  assert_eq!(extname_str("foo.", true), ".");
  assert_eq!(extname_str(".foo.", true), ".");
  assert_eq!(extname_str("foo..", true), ".");
  assert_eq!(extname_str("a./", true), ".");
}

#[test]
fn non_utf8() {
  assert_eq!(extname_with(b"\xff.\xfe", false), b".\xfe");
}
