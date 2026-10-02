use memchr::memrchr;

use crate::path_parsing::{Rules, SEP_BYTES};

// Ruby's `File.basename(path, ext)`.
pub fn basename<'a>(rules: Rules, path: &'a [u8], ext: &[u8]) -> &'a [u8] {
  let (name, base_len) = last_component(rules, path);
  match base_len {
    Some(base_len) => &name[..ext_end(rules, name, base_len, ext)],
    None => name,
  }
}

// Ruby's `ruby_enc_find_basename`: the last component of the path, without
// trailing separators (and on Windows, trailing dots, spaces and `:stream`),
// and where that component ends without its extension. A path that is only
// separators and prefixes has no extension: its "component" is a separator,
// or "" for a drive letter.
pub fn last_component(rules: Rules, path: &[u8]) -> (&[u8], Option<usize>) {
  if path.is_empty() {
    return (path, None);
  }
  let root = rules.skip_prefix(path);
  let mut name = root;
  while name < path.len() && rules.is_sep(path[name]) {
    name += 1;
  }
  if name == path.len() {
    if !rules.is_dosish() || name != root {
      // The last separator
      return (&path[name - 1..name], None);
    }
    if path[name - 1] == b':' {
      // A drive letter
      return (&path[name..], None);
    }
    // A UNC prefix
    return (SEP_BYTES, None);
  }
  let start = match rules.last_separator(&path[name..]) {
    Some(pos) => {
      let mut start = name + pos;
      while rules.is_sep(path[start]) {
        start += 1;
      }
      start
    }
    None => name,
  };
  let rest = &path[start..];
  let len = if rules.is_dosish() { rules.ntfs_tail(rest) } else { rules.chomp_dir_sep(rest) };
  let component = &rest[..len];
  let dots = component.iter().take_while(|&&c| c == b'.').count();
  let base_len = component[dots..].iter().rposition(|&c| c == b'.').map_or(len, |pos| dots + pos);
  (component, Some(base_len))
}

// Ruby's `rmext`: where `component` ends without the extension `ext`, or
// its length if `ext` isn't there. `ext` is an extension to remove, ".*"
// for any extension, or a character followed by `*` to remove everything
// from the last occurrence of that character.
pub fn ext_end(rules: Rules, component: &[u8], base_len: usize, ext: &[u8]) -> usize {
  let len = component.len();
  let end = match *ext {
    [] => 0,
    [b'.', b'*'] => base_len,
    [c, b'*'] => memrchr(c, component).unwrap_or(len),
    _ if len < ext.len() => len,
    _ => {
      let start = len - ext.len();
      if rules.same_path(&component[start..], ext) { start } else { 0 }
    }
  };
  if end == 0 { len } else { end }
}

#[cfg(test)]
fn basename_str<'a>(path: &'a str, ext: &str) -> &'a str {
  std::str::from_utf8(basename(Rules::UNIX, path.as_bytes(), ext.as_bytes())).unwrap()
}

#[cfg(test)]
fn windows_basename<'a>(path: &'a str, ext: &str) -> &'a str {
  std::str::from_utf8(basename(Rules::WINDOWS, path.as_bytes(), ext.as_bytes())).unwrap()
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

#[test]
fn ext_compared_like_ruby() {
  assert_eq!(basename_str("..", "."), ".");
  assert_eq!(basename_str("a.rb", ".RB"), "a.rb");
  assert_eq!(windows_basename("a.rb", ".RB"), "a");
}

#[test]
fn windows_separators() {
  assert_eq!(windows_basename("a\\b", ""), "b");
  assert_eq!(windows_basename("a\\b\\", ""), "b");
  assert_eq!(windows_basename("a/b\\c", ""), "c");
  assert_eq!(windows_basename("\\", ""), "\\");
  assert_eq!(basename_str("a\\b", ""), "a\\b");
}

#[test]
fn windows_prefixes() {
  assert_eq!(windows_basename("C:", ""), "");
  assert_eq!(windows_basename("C:/", ""), "/");
  assert_eq!(windows_basename("C:/a", ""), "a");
  assert_eq!(windows_basename("C:a", ""), "a");
  assert_eq!(windows_basename("//a/b", ""), "/");
  assert_eq!(windows_basename("//a/b/", ""), "/");
  assert_eq!(windows_basename("//a/b/c", ""), "c");
  assert_eq!(basename_str("//a/b", ""), "b");
}

#[test]
fn windows_ntfs_names() {
  assert_eq!(windows_basename("foo.test ", ""), "foo.test");
  assert_eq!(windows_basename("foo.test.", ""), "foo.test");
  assert_eq!(windows_basename("foo.test::$DATA", ""), "foo.test");
  assert_eq!(windows_basename("foo.test ", ".test"), "foo");
  assert_eq!(windows_basename("foo.test.", ".*"), "foo");
  assert_eq!(windows_basename("foo.test::$DATA", ".*"), "foo");
  assert_eq!(windows_basename("...", ""), "...");
}
