use prepend_prefix::prepend_prefix;
use basename::basename;
use chop_basename::chop_basename;
extern crate array_tool;
use self::array_tool::vec::Shift;
use std::path::MAIN_SEPARATOR;

pub fn cleanpath_aggressive(path: &str) -> String {
  let sep = MAIN_SEPARATOR.to_string();
  let mut names: Vec<String> = vec![];
  let mut pre = path.to_string();
  loop {
    match chop_basename(&pre.clone()) {
      Some((ref p, ref base)) => {
        pre = p.to_string();
        match base.as_ref() {
          "." => {},
          ".." => names.unshift(base.to_string()),
          _ => {
            if names.first() == Some(&"..".to_string()) {
              names.shift();
            } else {
              names.unshift(base.to_string())
            }
          }

        }
      },
      None => break,
    }
  }
  // // Windows Feature
  //
  // ```ruby
  // pre.tr!(File::ALT_SEPARATOR, File::SEPARATOR) if File::ALT_SEPARATOR
  // ```
  //
  if basename(&pre, "").contains(&sep) {
    loop {
      if names.first() == Some(&"..".to_string()) {
        let _ = names.shift();
      } else {
        break
      }
    }
  }
  prepend_prefix(&pre, &names.join(&sep)[..])
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
