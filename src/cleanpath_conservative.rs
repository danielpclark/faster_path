use std::borrow::Cow;
use std::path::MAIN_SEPARATOR;
use prepend_prefix::prepend_prefix;
use basename::basename;
use dirname::dirname;
use chop_basename::chop_basename;
use path_parsing::{SEP_STR, contains_sep};

pub fn cleanpath_conservative(path: &str) -> Cow<str> {
  let mut names: Vec<&str> = vec![];
  let mut prefix = path;
  while let Some((ref p, ref base)) = chop_basename(&prefix) {
    prefix = p;
    if base != &"." {
      names.push(base);
    }
  }
  // // Windows Feature
  //
  // ```ruby
  // pre.tr!(File::ALT_SEPARATOR, File::SEPARATOR) if File::ALT_SEPARATOR
  // ```
  //
  if contains_sep(basename(&prefix.as_bytes(), "")) {
    let len = names.iter().rposition(|&c| c != "..").map_or(0, |pos| pos + 1);
    names.truncate(len);
  }

  let last_name = match names.first() {
    Some(&name) => name,
    None => return dirname(&prefix).into(),
  };

  if last_name != ".." && basename(&path.as_bytes(), "") == ".".as_bytes() {
    names.reverse();
    names.push(".");
  } else if last_name != "." && last_name != ".." &&
      chop_basename(path).map(|(a, b)| a.len() + b.len()).unwrap() < path.len() {
    return format!("{}{}", last_name, MAIN_SEPARATOR).into();
  } else {
    names.reverse();
  }
  prepend_prefix(&prefix, &names.join(&SEP_STR))
}

#[test]
fn it_conservatively_cleans_the_path() {
  assert_eq!(cleanpath_conservative("/"),      "/");
  assert_eq!(cleanpath_conservative(""),      ".");
  assert_eq!(cleanpath_conservative("."),      ".");
  assert_eq!(cleanpath_conservative(".."),     "..");
  assert_eq!(cleanpath_conservative("a"),      "a");
  assert_eq!(cleanpath_conservative("/."),      "/");
  assert_eq!(cleanpath_conservative("/.."),      "/");
  assert_eq!(cleanpath_conservative("/a"),     "/a");
  assert_eq!(cleanpath_conservative("./"),      ".");
  assert_eq!(cleanpath_conservative("../"),     "..");
  assert_eq!(cleanpath_conservative("a/"),     "a/");
  assert_eq!(cleanpath_conservative("a//b"),    "a/b");
  assert_eq!(cleanpath_conservative("a/."),    "a/.");
  assert_eq!(cleanpath_conservative("a/./"),    "a/.");
  assert_eq!(cleanpath_conservative("a/../"),   "a/..");
  assert_eq!(cleanpath_conservative("/a/."),   "/a/.");
  assert_eq!(cleanpath_conservative("./.."),     "..");
  assert_eq!(cleanpath_conservative("../."),     "..");
  assert_eq!(cleanpath_conservative("./../"),     "..");
  assert_eq!(cleanpath_conservative(".././"),     "..");
  assert_eq!(cleanpath_conservative("/./.."),      "/");
  assert_eq!(cleanpath_conservative("/../."),      "/");
  assert_eq!(cleanpath_conservative("/./../"),      "/");
  assert_eq!(cleanpath_conservative("/.././"),      "/");
  assert_eq!(cleanpath_conservative("a/b/c"),  "a/b/c");
  assert_eq!(cleanpath_conservative("./b/c"),    "b/c");
  assert_eq!(cleanpath_conservative("a/./c"),    "a/c");
  assert_eq!(cleanpath_conservative("a/b/."),  "a/b/.");
  assert_eq!(cleanpath_conservative("a/../."),   "a/..");
  assert_eq!(cleanpath_conservative("/../.././../a"),     "/a");
  assert_eq!(cleanpath_conservative("a/b/../../../../c/../d"), "a/b/../../../../c/../d");

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
