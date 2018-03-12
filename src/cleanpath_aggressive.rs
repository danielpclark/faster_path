use std::borrow::Cow;
use prepend_prefix::prepend_prefix;
use basename::basename;
use chop_basename::chop_basename;
use path_parsing::{SEP_STR, contains_sep};

pub fn cleanpath_aggressive(path: &str) -> Cow<str> {
  let mut names: Vec<&str> = vec![];
  let mut prefix = path;
  while let Some((ref p, ref base)) = chop_basename(&prefix) {
    prefix = p;
    match base.as_ref() {
      "." => {}
      ".." => names.push(base),
      _ => {
        if names.last() == Some(&"..") {
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
  if contains_sep(basename(&prefix, "").as_bytes()) {
    let len = names.iter().rposition(|&c| c != "..").map_or(0, |pos| pos + 1);
    names.truncate(len);
  }
  names.reverse();
  prepend_prefix(&prefix, &names.join(&SEP_STR))
}

#[test]
fn it_aggressively_cleans_the_path() {
  assert_eq!(cleanpath_aggressive("/")                     ,       "/");
  assert_eq!(cleanpath_aggressive("")                      ,       ".");
  assert_eq!(cleanpath_aggressive(".")                     ,       ".");
  assert_eq!(cleanpath_aggressive("..")                    ,      "..");
  assert_eq!(cleanpath_aggressive("a")                     ,       "a");
  assert_eq!(cleanpath_aggressive("/.")                    ,       "/");
  assert_eq!(cleanpath_aggressive("/..")                   ,       "/");
  assert_eq!(cleanpath_aggressive("/a")                    ,      "/a");
  assert_eq!(cleanpath_aggressive("./")                    ,       ".");
  assert_eq!(cleanpath_aggressive("../")                   ,      "..");
  assert_eq!(cleanpath_aggressive("a/")                    ,       "a");
  assert_eq!(cleanpath_aggressive("a//b")                  ,     "a/b");
  assert_eq!(cleanpath_aggressive("a/.")                   ,       "a");
  assert_eq!(cleanpath_aggressive("a/./")                  ,       "a");
  assert_eq!(cleanpath_aggressive("a/..")                  ,       ".");
  assert_eq!(cleanpath_aggressive("a/../")                 ,       ".");
  assert_eq!(cleanpath_aggressive("/a/.")                  ,      "/a");
  assert_eq!(cleanpath_aggressive("./..")                  ,      "..");
  assert_eq!(cleanpath_aggressive("../.")                  ,      "..");
  assert_eq!(cleanpath_aggressive("./../")                 ,      "..");
  assert_eq!(cleanpath_aggressive(".././")                 ,      "..");
  assert_eq!(cleanpath_aggressive("/./..")                 ,       "/");
  assert_eq!(cleanpath_aggressive("/../.")                 ,       "/");
  assert_eq!(cleanpath_aggressive("/./../")                ,       "/");
  assert_eq!(cleanpath_aggressive("/.././")                ,       "/");
  assert_eq!(cleanpath_aggressive("a/b/c")                 ,   "a/b/c");
  assert_eq!(cleanpath_aggressive("./b/c")                 ,     "b/c");
  assert_eq!(cleanpath_aggressive("a/./c")                 ,     "a/c");
  assert_eq!(cleanpath_aggressive("a/b/.")                 ,     "a/b");
  assert_eq!(cleanpath_aggressive("a/../.")                ,       ".");
  assert_eq!(cleanpath_aggressive("/../.././../a")         ,      "/a");
  assert_eq!(cleanpath_aggressive("a/b/../../../../c/../d"), "../../d");
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
