use std::sync::atomic::{AtomicBool, Ordering};

use crate::path_parsing::Rules;

// Ruby 2.7 changed `File.extname` to return an extension of only a dot
// (`"foo."` on Unix, `"foo./"` on both Unix and Windows, where trailing dots
// aren't part of a file name) instead of `""`. `FasterPath` follows the Ruby
// it is loaded into; see `Init_faster_pathname`.
static TRAILING_DOT_IS_EXTNAME: AtomicBool = AtomicBool::new(false);

pub fn set_trailing_dot_is_extname(value: bool) {
  TRAILING_DOT_IS_EXTNAME.store(value, Ordering::Relaxed);
}

// Ruby's `File.extname`
pub fn extname(rules: Rules, path: &[u8]) -> &[u8] {
  extname_with(rules, path, TRAILING_DOT_IS_EXTNAME.load(Ordering::Relaxed))
}

// Ruby's `ruby_enc_find_extname`
fn extname_with(rules: Rules, path: &[u8], trailing_dot_is_extname: bool) -> &[u8] {
  let end = path.len();
  // The last component
  let mut p = match rules.last_separator(path) {
    Some(pos) => {
      let mut p = pos;
      while p < end && rules.is_sep(path[p]) {
        p += 1;
      }
      p
    }
    None => 0,
  };
  let name = p;

  // Leading dots don't start an extension
  while p < end && path[p] == b'.' {
    p += 1;
  }
  let mut dot = None;
  while p < end {
    let c = path[p];
    if c == b'.' || (rules.is_dosish() && c == b' ') {
      if !rules.is_dosish() {
        dot = Some(p);
      } else {
        // NTFS ignores trailing dots and spaces
        let last = p;
        let mut last_dot = last;
        p += 1;
        while p < end && (path[p] == b'.' || path[p] == b' ') {
          if path[p] == b'.' {
            last_dot = p;
          }
          p += 1;
        }
        if p >= end || rules.is_ads(path[p]) {
          p = last;
          break;
        }
        if c == b'.' || last_dot > last {
          dot = Some(last_dot);
        }
        continue;
      }
    } else if rules.is_ads(c) || rules.is_sep(c) {
      break;
    }
    p += 1;
  }
  match dot {
    Some(dot) if dot != name && dot + 1 == p => if trailing_dot_is_extname { &path[dot..p] } else { b"" },
    Some(dot) if dot != name => &path[dot..p],
    _ => b"",
  }
}

#[cfg(test)]
fn extname_str(path: &str, trailing_dot_is_extname: bool) -> &str {
  std::str::from_utf8(extname_with(Rules::UNIX, path.as_bytes(), trailing_dot_is_extname)).unwrap()
}

#[cfg(test)]
fn windows_extname(path: &str) -> &str {
  std::str::from_utf8(extname_with(Rules::WINDOWS, path.as_bytes(), true)).unwrap()
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
    assert_eq!(extname_str("a.rb ", trailing_dot), ".rb ");
    assert_eq!(extname_str("a\\b.c", trailing_dot), ".c");
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
  assert_eq!(extname_with(Rules::UNIX, b"\xff.\xfe", false), b".\xfe");
}

#[test]
fn windows() {
  assert_eq!(windows_extname("a.rb"), ".rb");
  assert_eq!(windows_extname("a.b\\c"), "");
  assert_eq!(windows_extname("a\\b.c"), ".c");
  assert_eq!(windows_extname("a.rb\\"), ".rb");
  assert_eq!(windows_extname("foo."), "");
  assert_eq!(windows_extname("foo.rb."), ".rb");
  assert_eq!(windows_extname("foo.rb "), ".rb");
  assert_eq!(windows_extname("foo.rb. ."), ".rb");
  assert_eq!(windows_extname("foo.rb::$DATA"), ".rb");
  assert_eq!(windows_extname("foo.rb::$DATA.bar"), ".rb");
  assert_eq!(windows_extname("foo .ext"), ".ext");
  assert_eq!(windows_extname("foo. .ext"), ".ext");
  assert_eq!(windows_extname("foo.ext .ext"), ".ext");
  assert_eq!(windows_extname("foo .ext "), ".ext");
  assert_eq!(windows_extname(".bashrc"), "");
}
