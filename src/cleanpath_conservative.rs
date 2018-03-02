use prepend_prefix::prepend_prefix;
use basename::basename;
use dirname::dirname;
use chop_basename::chop_basename;
extern crate array_tool;
use self::array_tool::vec::Shift;
use std::path::MAIN_SEPARATOR;

pub fn cleanpath_conservative(path: &str) -> String {
  let sep = MAIN_SEPARATOR.to_string();
  let mut names: Vec<String> = vec![];
  let mut pre = path.to_string();
  loop {
    match chop_basename(&pre.clone()) {
      Some((ref p, ref base)) => {
        pre = p.to_string();
        match base.as_ref() {
          "." => {},
          _ => names.unshift(base.to_string()),
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

  if names.is_empty() {
    return dirname(&pre).to_string();
  }

  if names.last() != Some(&"..".to_string()) && basename(&path, "") == &".".to_string() {
    names.push(".".to_string());
  }

  let result = prepend_prefix(&pre, &names.join(&sep)[..]);
  let last = names.last();

  if !(last == Some(&".".to_string()) || last == Some(&"..".to_string())) &&
    chop_basename(path).map(|(a, b)| a.len() + b.len()).unwrap() < path.len() {
    format!("{}{}", last.unwrap(), MAIN_SEPARATOR)
  } else {
    result
  }
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
