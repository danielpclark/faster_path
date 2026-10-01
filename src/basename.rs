use memchr::memrchr;

use crate::path_parsing::{find_last_sep_pos, find_last_non_sep_pos, SEP_BYTES};

pub fn basename<'a>(path: &'a [u8], ext: &[u8]) -> &'a [u8] {
  let base = last_component(path);
  &base[..ext_end(base, ext)]
}

// The last component of the path, without trailing separators; "/" when
// there are only separators.
pub fn last_component(path: &[u8]) -> &[u8] {
  let mut left: usize = 0;
  let mut right: usize = path.len();
  if let Some(last_slash_pos) = find_last_sep_pos(path) {
    if last_slash_pos == right - 1 {
      if let Some(pos) = find_last_non_sep_pos(&path[..last_slash_pos]) {
        right = pos + 1;
      } else {
        return SEP_BYTES;
      }
      if let Some(pos) = find_last_sep_pos(&path[..right]) {
        left = pos + 1;
      }
    } else {
      left = last_slash_pos + 1;
    }
  }
  &path[left..right]
}

// Where `slice` ends without the extension `ext`. `ext` is either an
// extension to remove, or a byte followed by `*` to remove everything from
// the last occurrence of that byte, such as ".*".
pub fn ext_end(slice: &[u8], ext: &[u8]) -> usize {
  if ext.len() >= slice.len() || slice == b"." || slice == b".." {
    return slice.len();
  }
  if let [first, b'*'] = *ext {
    match memrchr(first, slice) {
      Some(end) if end != 0 => return end,
      _ => {}
    };
  } else if slice.ends_with(ext) {
    return slice.len() - ext.len();
  }
  slice.len()
}

#[cfg(test)]
fn basename_str<'a>(path: &'a str, ext: &str) -> &'a str {
  std::str::from_utf8(basename(path.as_bytes(), ext.as_bytes())).unwrap()
}

#[test]
fn non_dot_asterisk_ext() {
  // This is undocumented Ruby functionality. We match it in case some code out there relies on it.
  assert_eq!(basename_str("abc", "b*"), "a");
  assert_eq!(basename_str("abc", "abc"), "abc");
  assert_eq!(basename_str("abc", "a*"), "abc");
  assert_eq!(basename_str("playlist", "l*"), "play");
  // Treated as literal "*":
  assert_eq!(basename_str("playlist", "yl*"), "playlist");
  assert_eq!(basename_str("playl*", "yl*"), "pla");
}

#[test]
fn empty() {
  assert_eq!(basename_str("", ""), "");
  assert_eq!(basename_str("", ".*"), "");
  assert_eq!(basename_str("", ".a"), "");
}

#[test]
fn sep() {
  assert_eq!(basename_str("/", ""), "/");
  assert_eq!(basename_str("//", ""), "/");
}

#[test]
fn trailing_dot() {
  assert_eq!(basename_str("file.test.", ""), "file.test.");
  assert_eq!(basename_str("file.test.", "."), "file.test");
  assert_eq!(basename_str("file.test.", ".*"), "file.test");
}

#[test]
fn trailing_dot_dot() {
  assert_eq!(basename_str("a..", ".."), "a");
  assert_eq!(basename_str("a..", ".*"), "a.");
}

#[test]
fn dot() {
  assert_eq!(basename_str(".", ""), ".");
  assert_eq!(basename_str(".", "."), ".");
  assert_eq!(basename_str(".", ".*"), ".");
}

#[test]
fn dot_dot() {
  assert_eq!(basename_str("..", ""), "..");
  assert_eq!(basename_str("..", ".*"), "..");
  assert_eq!(basename_str("..", ".."), "..");
  assert_eq!(basename_str("..", "..."), "..");
}

#[test]
fn non_dot_ext() {
  assert_eq!(basename_str("abc", "bc"), "a");
}

#[test]
fn basename_eq_ext() {
  assert_eq!(basename_str(".x", ".x"), ".x");
  assert_eq!(basename_str(".x", ".*"), ".x");
}

#[test]
fn absolute() {
  assert_eq!(basename_str("/a/b///c", ""), "c");
}

#[test]
fn trailing_slashes_absolute() {
  assert_eq!(basename_str("/a/b///c//////", ""), "c");
}

#[test]
fn relative() {
  assert_eq!(basename_str("b///c", ""), "c");
}

#[test]
fn trailing_slashes_relative() {
  assert_eq!(basename_str("b/c//", ""), "c");
}

#[test]
fn root() {
  assert_eq!(basename_str("//c", ""), "c");
}

#[test]
fn trailing_slashes_root() {
  assert_eq!(basename_str("//c//", ""), "c");
}

#[test]
fn trailing_slashes_relative_root() {
  assert_eq!(basename_str("c//", ""), "c");
}

#[test]
fn edge_case_all_seps() {
  assert_eq!("/", basename_str("///", ".*"));
}
