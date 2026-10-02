use std::borrow::Cow;

use crate::basename::basename;
use crate::chop_basename::chop_basename;
use crate::path_parsing::{Rules, SEP_BYTES};
use crate::prepend_prefix::prepend_prefix;

// Pathname's `cleanpath_aggressive`
pub fn cleanpath_aggressive(rules: Rules, path: &[u8]) -> Cow<'_, [u8]> {
  let mut names: Vec<&[u8]> = vec![];
  let mut prefix = path;
  while let Some((p, base)) = chop_basename(rules, prefix) {
    prefix = p;
    match base {
      b"." => {}
      b".." => names.push(base),
      _ => {
        if names.last() == Some(&&b".."[..]) {
          names.pop();
        } else {
          names.push(base);
        }
      }
    }
  }
  let prefix = with_separators(rules, prefix);
  if rules.contains_sep(basename(rules, &prefix, b"")) {
    let len = names.iter().rposition(|&c| c != b"..").map_or(0, |pos| pos + 1);
    names.truncate(len);
  }
  names.reverse();
  let path = prepend_prefix(rules, &prefix, &names.join(SEP_BYTES));
  Cow::Owned(path.into_owned())
}

// `pre.tr!(File::ALT_SEPARATOR, File::SEPARATOR)`
pub fn with_separators(rules: Rules, prefix: &[u8]) -> Cow<'_, [u8]> {
  if rules.is_dosish() && prefix.contains(&b'\\') {
    Cow::Owned(prefix.iter().map(|&c| if c == b'\\' { b'/' } else { c }).collect())
  } else {
    Cow::Borrowed(prefix)
  }
}

#[cfg(test)]
fn cleanpath_aggressive_str(path: &str) -> String {
  String::from_utf8(cleanpath_aggressive(Rules::UNIX, path.as_bytes()).into_owned()).unwrap()
}

#[cfg(test)]
fn windows_cleanpath_aggressive(path: &str) -> String {
  String::from_utf8(cleanpath_aggressive(Rules::WINDOWS, path.as_bytes()).into_owned()).unwrap()
}

#[test]
fn it_aggressively_cleans_the_path() {
  assert_eq!(cleanpath_aggressive_str("/")                     ,       "/");
  assert_eq!(cleanpath_aggressive_str("")                      ,       ".");
  assert_eq!(cleanpath_aggressive_str(".")                     ,       ".");
  assert_eq!(cleanpath_aggressive_str("..")                    ,      "..");
  assert_eq!(cleanpath_aggressive_str("a")                     ,       "a");
  assert_eq!(cleanpath_aggressive_str("/.")                    ,       "/");
  assert_eq!(cleanpath_aggressive_str("/..")                   ,       "/");
  assert_eq!(cleanpath_aggressive_str("/a")                    ,      "/a");
  assert_eq!(cleanpath_aggressive_str("./")                    ,       ".");
  assert_eq!(cleanpath_aggressive_str("../")                   ,      "..");
  assert_eq!(cleanpath_aggressive_str("a/")                    ,       "a");
  assert_eq!(cleanpath_aggressive_str("a//b")                  ,     "a/b");
  assert_eq!(cleanpath_aggressive_str("a/.")                   ,       "a");
  assert_eq!(cleanpath_aggressive_str("a/./")                  ,       "a");
  assert_eq!(cleanpath_aggressive_str("a/..")                  ,       ".");
  assert_eq!(cleanpath_aggressive_str("a/../")                 ,       ".");
  assert_eq!(cleanpath_aggressive_str("/a/.")                  ,      "/a");
  assert_eq!(cleanpath_aggressive_str("./..")                  ,      "..");
  assert_eq!(cleanpath_aggressive_str("../.")                  ,      "..");
  assert_eq!(cleanpath_aggressive_str("./../")                 ,      "..");
  assert_eq!(cleanpath_aggressive_str(".././")                 ,      "..");
  assert_eq!(cleanpath_aggressive_str("/./..")                 ,       "/");
  assert_eq!(cleanpath_aggressive_str("/../.")                 ,       "/");
  assert_eq!(cleanpath_aggressive_str("/./../")                ,       "/");
  assert_eq!(cleanpath_aggressive_str("/.././")                ,       "/");
  assert_eq!(cleanpath_aggressive_str("a/b/c")                 ,   "a/b/c");
  assert_eq!(cleanpath_aggressive_str("./b/c")                 ,     "b/c");
  assert_eq!(cleanpath_aggressive_str("a/./c")                 ,     "a/c");
  assert_eq!(cleanpath_aggressive_str("a/b/.")                 ,     "a/b");
  assert_eq!(cleanpath_aggressive_str("a/../.")                ,       ".");
  assert_eq!(cleanpath_aggressive_str("/../.././../a")         ,      "/a");
  assert_eq!(cleanpath_aggressive_str("a/b/../../../../c/../d"), "../../d");
}

#[test]
fn it_cleans_like_ruby_on_unix() {
  assert_eq!(cleanpath_aggressive_str("///"),        "/");
  assert_eq!(cleanpath_aggressive_str("///a"),       "/a");
  assert_eq!(cleanpath_aggressive_str("///.."),      "/");
  assert_eq!(cleanpath_aggressive_str("///."),       "/");
  assert_eq!(cleanpath_aggressive_str("///a/../.."), "/");
}

#[test]
fn it_cleans_like_ruby_on_windows() {
  assert_eq!(windows_cleanpath_aggressive("//a/b/c/"),     "//a/b/c");
  assert_eq!(windows_cleanpath_aggressive("c:\\foo\\bar"), "c:/foo/bar");
  assert_eq!(windows_cleanpath_aggressive("c:\\foo\\..\\..\\bar"), "c:/bar");
  assert_eq!(windows_cleanpath_aggressive("a\\.\\b"), "a/b");
  assert_eq!(windows_cleanpath_aggressive("C:"), "C:.");
  assert_eq!(windows_cleanpath_aggressive("C:a/../b"), "C:b");
}
