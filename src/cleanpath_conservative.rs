use std::borrow::Cow;

use crate::basename::basename;
use crate::chop_basename::chop_basename;
use crate::cleanpath_aggressive::with_separators;
use crate::dirname::dirname;
use crate::path_parsing::{Rules, SEP_BYTES};
use crate::prepend_prefix::prepend_prefix;

// Pathname's `cleanpath_conservative`
pub fn cleanpath_conservative(rules: Rules, path: &[u8]) -> Cow<'_, [u8]> {
  let mut names: Vec<&[u8]> = vec![];
  let mut prefix = path;
  while let Some((p, base)) = chop_basename(rules, prefix) {
    prefix = p;
    if base != b"." {
      names.push(base);
    }
  }
  let prefix = with_separators(rules, prefix);
  if rules.contains_sep(basename(rules, &prefix, b"")) {
    let len = names.iter().rposition(|&c| c != b"..").map_or(0, |pos| pos + 1);
    names.truncate(len);
  }

  if names.is_empty() {
    return Cow::Owned(dirname(rules, &prefix).into_owned());
  }
  names.reverse();
  if names.last() != Some(&&b".."[..]) && basename(rules, path, b"") == b"." {
    names.push(b".");
  }
  let result = prepend_prefix(rules, &prefix, &names.join(SEP_BYTES)).into_owned();
  match names.last() {
    Some(&b".") | Some(&b"..") => Cow::Owned(result),
    _ if has_trailing_separator(rules, path) => Cow::Owned(add_trailing_separator(rules, Cow::Owned(result)).into_owned()),
    _ => Cow::Owned(result),
  }
}

// Pathname's `has_trailing_separator?`
pub fn has_trailing_separator(rules: Rules, path: &[u8]) -> bool {
  match chop_basename(rules, path) {
    Some((dirname, basename)) => dirname.len() + basename.len() < path.len(),
    None => false,
  }
}

// Pathname's `add_trailing_separator`:
// `File.basename(path + 'a') == 'a' ? path : File.join(path, '')`
pub fn add_trailing_separator(rules: Rules, path: Cow<'_, [u8]>) -> Cow<'_, [u8]> {
  let ends_a_path = if rules.is_dosish() {
    basename(rules, &[&path[..], b"a"].concat(), b"") == b"a"
  } else {
    path.last().map_or(true, |&c| rules.is_sep(c))
  };
  if ends_a_path {
    path
  } else {
    Cow::Owned(rules.join(&path, b""))
  }
}

// Pathname's `del_trailing_separator`
pub fn del_trailing_separator(rules: Rules, path: &[u8]) -> Cow<'_, [u8]> {
  if let Some((dirname, basename)) = chop_basename(rules, path) {
    // The basename follows the dirname
    return Cow::Borrowed(&path[..dirname.len() + basename.len()]);
  }
  match rules.last_non_sep_pos(path) {
    // No trailing separators
    _ if path.last().map_or(true, |&c| !rules.is_sep(c)) => Cow::Borrowed(path),
    // `$` + File.dirname(path)[/#{SEPARATOR_PAT}*\z/]`
    last => {
      let before = last.map_or(0, |pos| pos + 1);
      let dir = dirname(rules, path);
      let separators = dir.len() - dir.iter().rev().take_while(|&&c| rules.is_sep(c)).count();
      Cow::Owned([&path[..before], &dir[separators..]].concat())
    }
  }
}

#[cfg(test)]
fn cleanpath_conservative_str(path: &str) -> String {
  String::from_utf8(cleanpath_conservative(Rules::UNIX, path.as_bytes()).into_owned()).unwrap()
}

#[cfg(test)]
fn windows_cleanpath_conservative(path: &str) -> String {
  String::from_utf8(cleanpath_conservative(Rules::WINDOWS, path.as_bytes()).into_owned()).unwrap()
}

#[cfg(test)]
fn del_str(rules: Rules, path: &str) -> String {
  String::from_utf8(del_trailing_separator(rules, path.as_bytes()).into_owned()).unwrap()
}

#[test]
fn it_keeps_the_whole_path_with_a_trailing_separator() {
  assert_eq!(cleanpath_conservative_str("a/b/"),     "a/b/");
  assert_eq!(cleanpath_conservative_str("/a/b//"),   "/a/b/");
  assert_eq!(cleanpath_conservative_str("a/./b/"),   "a/b/");
  assert_eq!(cleanpath_conservative_str("a/b/../"),  "a/b/..");
}

#[test]
fn it_adds_trailing_separators() {
  assert_eq!(add_trailing_separator(Rules::UNIX, Cow::Borrowed(&b""[..])), &b""[..]);
  assert_eq!(add_trailing_separator(Rules::UNIX, Cow::Borrowed(&b"/"[..])), &b"/"[..]);
  assert_eq!(add_trailing_separator(Rules::UNIX, Cow::Borrowed(&b"a"[..])), &b"a/"[..]);
  assert_eq!(add_trailing_separator(Rules::UNIX, Cow::Borrowed(&b"a/"[..])), &b"a/"[..]);
}

#[test]
fn it_detects_trailing_separators() {
  assert!(!has_trailing_separator(Rules::UNIX, b""));
  assert!(!has_trailing_separator(Rules::UNIX, b"/"));
  assert!(!has_trailing_separator(Rules::UNIX, b"a"));
  assert!(has_trailing_separator(Rules::UNIX, b"a/"));
  assert!(has_trailing_separator(Rules::UNIX, b"/a//"));
}

#[test]
fn it_conservatively_cleans_the_path() {
  assert_eq!(cleanpath_conservative_str("/"),      "/");
  assert_eq!(cleanpath_conservative_str(""),      ".");
  assert_eq!(cleanpath_conservative_str("."),      ".");
  assert_eq!(cleanpath_conservative_str(".."),     "..");
  assert_eq!(cleanpath_conservative_str("a"),      "a");
  assert_eq!(cleanpath_conservative_str("/."),      "/");
  assert_eq!(cleanpath_conservative_str("/.."),      "/");
  assert_eq!(cleanpath_conservative_str("/a"),     "/a");
  assert_eq!(cleanpath_conservative_str("./"),      ".");
  assert_eq!(cleanpath_conservative_str("../"),     "..");
  assert_eq!(cleanpath_conservative_str("a/"),     "a/");
  assert_eq!(cleanpath_conservative_str("a//b"),    "a/b");
  assert_eq!(cleanpath_conservative_str("a/."),    "a/.");
  assert_eq!(cleanpath_conservative_str("a/./"),    "a/.");
  assert_eq!(cleanpath_conservative_str("a/../"),   "a/..");
  assert_eq!(cleanpath_conservative_str("/a/."),   "/a/.");
  assert_eq!(cleanpath_conservative_str("./.."),     "..");
  assert_eq!(cleanpath_conservative_str("../."),     "..");
  assert_eq!(cleanpath_conservative_str("./../"),     "..");
  assert_eq!(cleanpath_conservative_str(".././"),     "..");
  assert_eq!(cleanpath_conservative_str("/./.."),      "/");
  assert_eq!(cleanpath_conservative_str("/../."),      "/");
  assert_eq!(cleanpath_conservative_str("/./../"),      "/");
  assert_eq!(cleanpath_conservative_str("/.././"),      "/");
  assert_eq!(cleanpath_conservative_str("a/b/c"),  "a/b/c");
  assert_eq!(cleanpath_conservative_str("./b/c"),    "b/c");
  assert_eq!(cleanpath_conservative_str("a/./c"),    "a/c");
  assert_eq!(cleanpath_conservative_str("a/b/."),  "a/b/.");
  assert_eq!(cleanpath_conservative_str("a/../."),   "a/..");
  assert_eq!(cleanpath_conservative_str("/../.././../a"),     "/a");
  assert_eq!(cleanpath_conservative_str("a/b/../../../../c/../d"), "a/b/../../../../c/../d");

  assert_eq!(cleanpath_conservative_str("//"), "/");
}

#[test]
fn it_conservatively_cleans_windows_paths() {
  assert_eq!(windows_cleanpath_conservative("c:\\foo\\bar"), "c:/foo/bar");
  assert_eq!(windows_cleanpath_conservative("//"), "//");
  assert_eq!(windows_cleanpath_conservative("a\\b\\"), "a/b/");
  assert_eq!(windows_cleanpath_conservative("C:/a/../b/"), "C:/a/../b/");
}

#[test]
fn it_adds_trailing_separators_on_windows() {
  let add = |path: &'static str| add_trailing_separator(Rules::WINDOWS, Cow::Borrowed(path.as_bytes())).into_owned();
  assert_eq!(add("a"), b"a/");
  assert_eq!(add("a\\"), b"a\\");
  assert_eq!(add("C:"), b"C:");
  assert_eq!(add("C:/"), b"C:/");
  assert_eq!(add("//a/b"), b"//a/b/");
  assert!(has_trailing_separator(Rules::WINDOWS, b"a\\"));
  assert!(!has_trailing_separator(Rules::UNIX, b"a\\"));
}

#[test]
fn it_deletes_trailing_separators() {
  for &(path, expected) in &[("", ""), ("/", "/"), ("/a", "/a"), ("/a/", "/a"), ("/a//", "/a"),
                             (".", "."), ("./", "."), (".//", "."), ("///", "/"), ("///a/", "///a")] {
    assert_eq!(del_str(Rules::UNIX, path), expected, "{:?}", path);
  }
  for &(path, expected) in &[("A:", "A:"), ("A:/", "A:/"), ("A://", "A:/"), ("A:.", "A:."), ("A:./", "A:."),
                             ("A:.//", "A:."), ("//", "//"), ("//a", "//a"), ("//a/", "//a"), ("//a//", "//a"),
                             ("//a/b", "//a/b"), ("//a/b/", "//a/b"), ("//a/b//", "//a/b"), ("//a/b/c", "//a/b/c"),
                             ("//a/b/c/", "//a/b/c"), ("//a/b/c//", "//a/b/c"), ("a\\", "a")] {
    assert_eq!(del_str(Rules::WINDOWS, path), expected, "{:?}", path);
  }
}
