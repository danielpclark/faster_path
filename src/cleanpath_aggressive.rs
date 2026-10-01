use std::borrow::Cow;
use crate::prepend_prefix::prepend_prefix;
use crate::basename::basename;
use crate::chop_basename::chop_basename;
use crate::path_parsing::{SEP_BYTES, contains_sep};

pub fn cleanpath_aggressive(path: &[u8]) -> Cow<'_, [u8]> {
  let mut names: Vec<&[u8]> = vec![];
  let mut prefix = path;
  while let Some((p, base)) = chop_basename(prefix) {
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
  names.reverse();
  prepend_prefix(prefix, &names.join(SEP_BYTES))
}

#[cfg(test)]
fn cleanpath_aggressive_str(path: &str) -> String {
  String::from_utf8(cleanpath_aggressive(path.as_bytes()).into_owned()).unwrap()
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

// Future Windows Support
//
// DOSISH = File::ALT_SEPARATOR != nil
// DOSISH_DRIVE_LETTER = File.dirname("A:") == "A:."
// DOSISH_UNC = File.dirname("//") == "//"
//
// if DOSISH_UNC
//   defassert(:cleanpath_aggressive, '//a/b/c', '//a/b/c/')
// else
//   defassert(:cleanpath_aggressive, '/',       '///')
//   defassert(:cleanpath_aggressive, '/a',      '///a')
//   defassert(:cleanpath_aggressive, '/',       '///..')
//   defassert(:cleanpath_aggressive, '/',       '///.')
//   defassert(:cleanpath_aggressive, '/',       '///a/../..')
// end
//
// if DOSISH
//   defassert(:cleanpath_aggressive, 'c:/foo/bar', 'c:\\foo\\bar')
// end
