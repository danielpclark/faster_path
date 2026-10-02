use memchr::memmem;

use crate::basename::basename;
use crate::path_parsing::{offset_in, Rules};

// Pathname's `chop_basename`: the path split before its basename, unless
// the basename is empty or a separator.
pub fn chop_basename(rules: Rules, path: &[u8]) -> Option<(&[u8], &[u8])> {
  let base = basename(rules, path, b"");
  if base.is_empty() || (base.len() == 1 && rules.is_sep(base[0])) {
    return None;
  }
  // `path.rindex(base)`. On Unix, nothing after the basename can contain it.
  let start = if rules.is_dosish() { memmem::rfind(path, base)? } else { offset_in(path, base) };
  Some((&path[..start], &path[start..start + base.len()]))
}

// Pathname's `relative?`: whether chopping every basename off leaves nothing.
pub fn is_relative(rules: Rules, path: &[u8]) -> bool {
  if !rules.is_dosish() {
    return path.first() != Some(&b'/');
  }
  let mut prefix = path;
  while let Some((rest, _)) = chop_basename(rules, prefix) {
    prefix = rest;
  }
  prefix.is_empty()
}

#[cfg(test)]
fn chop_basename_str(input: &str) -> Option<(&str, &str)> {
  chop_basename(Rules::UNIX, input.as_bytes()).map(|(dirname, basename)| {
    (std::str::from_utf8(dirname).unwrap(), std::str::from_utf8(basename).unwrap())
  })
}

#[test]
fn it_chops_the_basename_and_dirname() {
  assert_eq!(chop_basename_str(""),           None );
  assert_eq!(chop_basename_str("/"),          None );
  assert_eq!(chop_basename_str("."),          Some(("", ".")) );
  assert_eq!(chop_basename_str("asdf/asdf"),  Some(("asdf/",     "asdf")) );
  assert_eq!(chop_basename_str("asdf.txt"),   Some(("",      "asdf.txt")) );
  assert_eq!(chop_basename_str("asdf/"),      Some(("",          "asdf")) );
  assert_eq!(chop_basename_str("/asdf/"),     Some(("/",         "asdf")) );
  assert_eq!(chop_basename_str("a///b"),      Some(("a///",         "b")) );
  assert_eq!(chop_basename_str("a///b//"),    Some(("a///",         "b")) );
  assert_eq!(chop_basename_str("/a///b//"),   Some(("/a///",        "b")) );
  assert_eq!(chop_basename_str("/a///b//"),   Some(("/a///",        "b")) );

  assert_eq!(chop_basename_str("./../..///.../..//"), Some(("./../..///.../", "..")));
}

#[test]
fn it_chops_non_utf8_paths() {
  assert_eq!(chop_basename(Rules::UNIX, b"\xff/\xfe\xfd/"), Some((&b"\xff/"[..], &b"\xfe\xfd"[..])));
}

#[cfg(test)]
fn windows_chop(input: &str) -> Option<(&str, &str)> {
  chop_basename(Rules::WINDOWS, input.as_bytes()).map(|(dirname, basename)| {
    (std::str::from_utf8(dirname).unwrap(), std::str::from_utf8(basename).unwrap())
  })
}

#[test]
fn it_chops_windows_paths() {
  assert_eq!(windows_chop("a\\b"), Some(("a\\", "b")));
  assert_eq!(windows_chop("a\\b\\"), Some(("a\\", "b")));
  assert_eq!(windows_chop("C:"), None);
  assert_eq!(windows_chop("C:/"), None);
  assert_eq!(windows_chop("C:/a"), Some(("C:/", "a")));
  assert_eq!(windows_chop("C:a"), Some(("C:", "a")));
  assert_eq!(windows_chop("//a/b"), None);
  assert_eq!(windows_chop("//a/b/c"), Some(("//a/b/", "c")));
  assert_eq!(windows_chop("a."), Some(("", "a")));
}

#[test]
fn it_knows_relative_paths() {
  for &rules in &[Rules::UNIX, Rules::WINDOWS] {
    assert!(is_relative(rules, b""));
    assert!(is_relative(rules, b"a"));
    assert!(is_relative(rules, b"a/b"));
    assert!(!is_relative(rules, b"/"));
    assert!(!is_relative(rules, b"/a"));
  }
  assert!(is_relative(Rules::UNIX, b"C:/a"));
  assert!(is_relative(Rules::UNIX, b"\\a"));
  for path in &["A:", "A:/", "A:/a", "//", "//a", "//a/", "//a/b", "//a/b/", "//a/b/c", "\\a"] {
    assert!(!is_relative(Rules::WINDOWS, path.as_bytes()), "{}", path);
  }
}
