extern crate memchr;

use self::memchr::memrchr;

use path_parsing::{find_last_sep_pos, find_last_non_sep_pos};

pub fn basename<'a>(path: &'a [u8], ext: &str) -> &'a [u8] {
  let mut left: usize = 0;
  let mut right: usize = path.len();
  if let Some(last_slash_pos) = find_last_sep_pos(path) {
    if last_slash_pos == right - 1 {
      if let Some(pos) = find_last_non_sep_pos(&path[..last_slash_pos]) {
        right = pos + 1;
      } else {
        return "/".as_bytes();
      }
      if let Some(pos) = find_last_sep_pos(&path[..right]) {
        left = pos + 1;
      }
    } else {
      left = last_slash_pos + 1;
    }
  }
  &path[left..left + ext_end(&path[left..right], ext)]
}

fn ext_end(slice: &[u8], ext: &str) -> usize {
  if ext.len() >= slice.len() || slice == b"." || slice == b".." {
    return slice.len();
  }
  let ext_bytes = ext.as_bytes();
  if ext_bytes.len() == 2 && *ext_bytes.get(1).unwrap() == b'*' {
    match memrchr(*ext_bytes.get(0).unwrap(), slice) {
      Some(end) if end != 0 => return end,
      _ => {}
    };
  } else if slice.ends_with(ext_bytes) {
    return slice.len() - ext_bytes.len();
  }
  slice.len()
}

#[test]
fn non_dot_asterisk_ext() {
  // This is undocumented Ruby functionality. We match it in case some code out there relies on it.
  assert_eq!(basename("abc".as_bytes(), "b*"), b"a");
  assert_eq!(basename("abc".as_bytes(), "abc"), b"abc");
  assert_eq!(basename("abc".as_bytes(), "a*"), b"abc");
  assert_eq!(basename("playlist".as_bytes(), "l*"), b"play");
  // Treated as literal "*":
  assert_eq!(basename("playlist".as_bytes(), "yl*"), b"playlist");
  assert_eq!(basename("playl*".as_bytes(), "yl*"), b"pla");
}

#[test]
fn empty() {
  assert_eq!(basename("".as_bytes(), ""), b"");
  assert_eq!(basename("".as_bytes(), ".*"), b"");
  assert_eq!(basename("".as_bytes(), ".a"), b"");
}

#[test]
fn sep() {
  assert_eq!(basename("/".as_bytes(), ""), b"/");
  assert_eq!(basename("//".as_bytes(), ""), b"/");
}

#[test]
fn trailing_dot() {
  assert_eq!(basename("file.test.".as_bytes(), ""), b"file.test.");
  assert_eq!(basename("file.test.".as_bytes(), "."), b"file.test");
  assert_eq!(basename("file.test.".as_bytes(), ".*"), b"file.test");
}

#[test]
fn trailing_dot_dot() {
  assert_eq!(basename("a..".as_bytes(), ".."), b"a");
  assert_eq!(basename("a..".as_bytes(), ".*"), b"a.");
}

#[test]
fn dot() {
  assert_eq!(basename(".".as_bytes(), ""), b".");
  assert_eq!(basename(".".as_bytes(), "."), b".");
  assert_eq!(basename(".".as_bytes(), ".*"), b".");
}

#[test]
fn dot_dot() {
  assert_eq!(basename("..".as_bytes(), ""), b"..");
  assert_eq!(basename("..".as_bytes(), ".*"), b"..");
  assert_eq!(basename("..".as_bytes(), ".."), b"..");
  assert_eq!(basename("..".as_bytes(), "..."), b"..");
}

#[test]
fn non_dot_ext() {
  assert_eq!(basename("abc".as_bytes(), "bc"), b"a");
}

#[test]
fn basename_eq_ext() {
  assert_eq!(basename(".x".as_bytes(), ".x"), b".x");
  assert_eq!(basename(".x".as_bytes(), ".*"), b".x");
}

#[test]
fn absolute() {
  assert_eq!(basename("/a/b///c".as_bytes(), ""), b"c");
}

#[test]
fn trailing_slashes_absolute() {
  assert_eq!(basename("/a/b///c//////".as_bytes(), ""), b"c");
}

#[test]
fn relative() {
  assert_eq!(basename("b///c".as_bytes(), ""), b"c");
}

#[test]
fn trailing_slashes_relative() {
  assert_eq!(basename("b/c//".as_bytes(), ""), b"c");
}

#[test]
fn root() {
  assert_eq!(basename("//c".as_bytes(), ""), b"c");
}

#[test]
fn trailing_slashes_root() {
  assert_eq!(basename("//c//".as_bytes(), ""), b"c");
}

#[test]
fn trailing_slashes_relative_root() {
  assert_eq!(basename("c//".as_bytes(), ""), b"c");
}

#[test]
fn edge_case_all_seps() {
  assert_eq!(b"/", basename("///".as_bytes(), ".*"));
}
