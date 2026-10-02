use std::borrow::Cow;

use crate::path_parsing::Rules;

// Ruby's `File.dirname`
pub fn dirname(rules: Rules, path: &[u8]) -> Cow<'_, [u8]> {
  let mut name = 0;
  let mut root = rules.skip_root(path);
  if rules.is_dosish() {
    // Keep a UNC prefix
    if root > 1 && rules.is_sep(path[0]) {
      name = root - 2;
      root = name + rules.skip_prefix(&path[name..]);
    }
  } else if root > 1 {
    // Leading separators become one
    name = root - 1;
  }
  let end = rules.last_separator(&path[root..]).map_or(root, |pos| root + pos);
  if end == name {
    return Cow::Borrowed(b".");
  }
  let path = &path[name..];
  let end = end - name;
  let root = root - name;
  if rules.has_drive_letter(path) {
    if path.len() > 2 && rules.is_sep(path[2]) {
      // "C:" and one separator, then the rest
      let top = 2 + rules.skip_root(&path[2..]);
      return Cow::Owned([&path[..3], &path[top..end.max(top)]].concat());
    }
    if root == 2 && end == 2 {
      return Cow::Owned([&path[..2], b"."].concat());
    }
  }
  Cow::Borrowed(&path[..end])
}

#[cfg(test)]
fn dirname_str(path: &str) -> String {
  String::from_utf8(dirname(Rules::UNIX, path.as_bytes()).into_owned()).unwrap()
}

#[cfg(test)]
fn windows_dirname(path: &str) -> String {
  String::from_utf8(dirname(Rules::WINDOWS, path.as_bytes()).into_owned()).unwrap()
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
  assert_eq!(dirname(Rules::UNIX, b"\xff\xfe/a").as_ref(), b"\xff\xfe");
}

#[test]
fn windows_separators() {
  assert_eq!(windows_dirname("a\\b"), "a");
  assert_eq!(windows_dirname("a\\b\\"), "a");
  assert_eq!(windows_dirname("a/b\\c"), "a/b");
  assert_eq!(dirname_str("a\\b"), ".");
}

#[test]
fn windows_drive_letters() {
  assert_eq!(windows_dirname("C:"), "C:.");
  assert_eq!(windows_dirname("C:a"), "C:.");
  assert_eq!(windows_dirname("C:/"), "C:/");
  assert_eq!(windows_dirname("C:/a"), "C:/");
  assert_eq!(windows_dirname("C://a"), "C:/");
  assert_eq!(windows_dirname("C:/a/b"), "C:/a");
  assert_eq!(windows_dirname("C://a//b"), "C:/a");
  assert_eq!(windows_dirname("C:a/b"), "C:a");
  assert_eq!(dirname_str("C:"), ".");
}

#[test]
fn windows_unc() {
  assert_eq!(windows_dirname("//"), "//");
  assert_eq!(windows_dirname("//a"), "//a");
  assert_eq!(windows_dirname("//a/b"), "//a/b");
  assert_eq!(windows_dirname("//a/b/c"), "//a/b");
  assert_eq!(windows_dirname("//a/b/c/d"), "//a/b/c");
  assert_eq!(windows_dirname("///a"), "//a");
  assert_eq!(dirname_str("//a/b"), "/a");
}
