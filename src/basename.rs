extern crate memchr;

use self::memchr::memrchr;

use path_parsing::{find_last_sep_pos, find_last_non_sep_pos};

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
  &path[left..left + ext_end(&bytes[left..right], ext)]
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
  assert_eq!(basename("abc", "b*"), "a");
  assert_eq!(basename("abc", "abc"), "abc");
  assert_eq!(basename("abc", "a*"), "abc");
  assert_eq!(basename("playlist", "l*"), "play");
  // Treated as literal "*":
  assert_eq!(basename("playlist", "yl*"), "playlist");
  assert_eq!(basename("playl*", "yl*"), "pla");
}

#[test]
fn empty() {
  assert_eq!(basename("", ""), "");
  assert_eq!(basename("", ".*"), "");
  assert_eq!(basename("", ".a"), "");
}

#[test]
fn sep() {
  assert_eq!(basename("/", ""), "/");
  assert_eq!(basename("//", ""), "/");
}

#[test]
fn trailing_dot() {
  assert_eq!(basename("file.test.", ""), "file.test.");
  assert_eq!(basename("file.test.", "."), "file.test");
  assert_eq!(basename("file.test.", ".*"), "file.test");
}

#[test]
fn trailing_dot_dot() {
  assert_eq!(basename("a..", ".."), "a");
  assert_eq!(basename("a..", ".*"), "a.");
}

#[test]
fn dot() {
  assert_eq!(basename(".", ""), ".");
  assert_eq!(basename(".", "."), ".");
  assert_eq!(basename(".", ".*"), ".");
}

#[test]
fn dot_dot() {
  assert_eq!(basename("..", ""), "..");
  assert_eq!(basename("..", ".*"), "..");
  assert_eq!(basename("..", ".."), "..");
  assert_eq!(basename("..", "..."), "..");
}

#[test]
fn non_dot_ext() {
  assert_eq!(basename("abc", "bc"), "a");
}

#[test]
fn basename_eq_ext() {
  assert_eq!(basename(".x", ".x"), ".x");
  assert_eq!(basename(".x", ".*"), ".x");
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
fn trailing_slashes_with_file_and_extension() {
  assert_eq!("base", basename("dir//base.c/", ".c"));
  assert_eq!("foo", basename("foo.rb/", ".rb"));
  assert_eq!("base", basename("dir//base.c/", ".*"));
  assert_eq!("bar", basename("bar.rb///", ".*"));
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
