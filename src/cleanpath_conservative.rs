use std::borrow::Cow;
use crate::prepend_prefix::prepend_prefix;
use crate::basename::basename;
use crate::dirname::dirname;
use crate::chop_basename::chop_basename;
use crate::path_parsing::{SEP, SEP_BYTES, contains_sep};

pub fn cleanpath_conservative(path: &[u8]) -> Cow<'_, [u8]> {
  let mut names: Vec<&[u8]> = vec![];
  let mut prefix = path;
  while let Some((p, base)) = chop_basename(prefix) {
    prefix = p;
    if base != b"." {
      names.push(base);
    }
  }
  // // Windows Feature
  //
  // ```ruby
  // pre.tr!(File::ALT_SEPARATOR, File::SEPARATOR) if File::ALT_SEPARATOR
  // ```
  //
  if contains_sep(basename(prefix, b"")) {
    let len = names.iter().rposition(|&c| c != b"..").map_or(0, |pos| pos + 1);
    names.truncate(len);
  }

  if names.is_empty() {
    return dirname(prefix).into();
  }
  names.reverse();
  if names.last() != Some(&&b".."[..]) && basename(path, b"") == b"." {
    names.push(b".");
  }
  let result = prepend_prefix(prefix, &names.join(SEP_BYTES));
  match names.last() {
    Some(&b".") | Some(&b"..") => result,
    _ if has_trailing_separator(path) => add_trailing_separator(result),
    _ => result,
  }
}

pub fn has_trailing_separator(path: &[u8]) -> bool {
  match chop_basename(path) {
    Some((dirname, basename)) => dirname.len() + basename.len() < path.len(),
    None => false,
  }
}

// Ruby: `File.basename(path + 'a') == 'a' ? path : File.join(path, '')`
pub fn add_trailing_separator(path: Cow<'_, [u8]>) -> Cow<'_, [u8]> {
  if path.is_empty() || path.last() == Some(&SEP) {
    path
  } else {
    let mut path = path.into_owned();
    path.push(SEP);
    path.into()
  }
}

#[cfg(test)]
fn cleanpath_conservative_str(path: &str) -> String {
  String::from_utf8(cleanpath_conservative(path.as_bytes()).into_owned()).unwrap()
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
  assert_eq!(add_trailing_separator(Cow::Borrowed(&b""[..])), &b""[..]);
  assert_eq!(add_trailing_separator(Cow::Borrowed(&b"/"[..])), &b"/"[..]);
  assert_eq!(add_trailing_separator(Cow::Borrowed(&b"a"[..])), &b"a/"[..]);
  assert_eq!(add_trailing_separator(Cow::Borrowed(&b"a/"[..])), &b"a/"[..]);
}

#[test]
fn it_detects_trailing_separators() {
  assert!(!has_trailing_separator(b""));
  assert!(!has_trailing_separator(b"/"));
  assert!(!has_trailing_separator(b"a"));
  assert!(has_trailing_separator(b"a/"));
  assert!(has_trailing_separator(b"/a//"));
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

// Future Windows Support
//
// DOSISH = File::ALT_SEPARATOR != nil
// DOSISH_DRIVE_LETTER = File.dirname("A:") == "A:."
// DOSISH_UNC = File.dirname("//") == "//"
//
//
//   if DOSISH
//     assert_eq!(cleanpath_conservative, 'c:/foo/bar', 'c:\\foo\\bar')
//   end
//
//   if DOSISH_UNC
//     assert_eq!(cleanpath_conservative, '//',     '//')
//   else
//     assert_eq!(cleanpath_conservative, '/',      '//')
//   end
}
