use std::borrow::Cow;

use crate::basename::basename;
use crate::chop_basename::chop_basename;
use crate::dirname::dirname;
use crate::path_parsing::Rules;

// Pathname's `plus`, which `Pathname#+` and `Pathname#join` use.
pub fn plus_paths<'a>(rules: Rules, path1: &'a [u8], path2: &[u8]) -> Cow<'a, [u8]> {
  // The names in path2, and where each starts
  let mut prefix2 = path2;
  let mut index_list2: Vec<usize> = vec![];
  let mut basename_list2: Vec<&[u8]> = vec![];
  while let Some((prefix, basename)) = chop_basename(rules, prefix2) {
    prefix2 = prefix;
    index_list2.push(prefix.len());
    basename_list2.push(basename);
  }
  if !prefix2.is_empty() {
    return Cow::Owned(path2.to_vec());
  }
  index_list2.reverse();
  basename_list2.reverse();
  // The names before `first` have been shifted off
  let mut first = 0;

  let mut prefix1 = path1;
  loop {
    while basename_list2.get(first) == Some(&&b"."[..]) {
      first += 1;
    }
    let (prefix, basename1) = match chop_basename(rules, prefix1) {
      Some(chopped) => chopped,
      None => break,
    };
    prefix1 = prefix;
    if basename1 == b"." {
      continue;
    }
    if basename1 == b".." || basename_list2.get(first) != Some(&&b".."[..]) {
      // `prefix1 + basename1`; the basename follows its prefix
      prefix1 = &path1[..prefix1.len() + basename1.len()];
      break;
    }
    first += 1;
  }

  let mut prefix1_has_name = chop_basename(rules, prefix1).is_some();
  if !prefix1_has_name && rules.contains_sep(basename(rules, prefix1, b"")) {
    // Nothing goes above the root
    prefix1_has_name = true;
    while basename_list2.get(first) == Some(&&b".."[..]) {
      first += 1;
    }
  }
  if first < basename_list2.len() {
    let suffix2 = &path2[index_list2[first]..];
    if prefix1_has_name {
      Cow::Owned(rules.join(prefix1, suffix2))
    } else {
      Cow::Owned([prefix1, suffix2].concat())
    }
  } else if prefix1_has_name {
    Cow::Borrowed(prefix1)
  } else {
    dirname(rules, prefix1)
  }
}

#[cfg(test)]
fn plus_str(path1: &str, path2: &str) -> String {
  String::from_utf8(plus_paths(Rules::UNIX, path1.as_bytes(), path2.as_bytes()).into_owned()).unwrap()
}

#[cfg(test)]
fn windows_plus(path1: &str, path2: &str) -> String {
  String::from_utf8(plus_paths(Rules::WINDOWS, path1.as_bytes(), path2.as_bytes()).into_owned()).unwrap()
}

#[test]
fn it_will_plus_same_as_ruby() {
  assert_eq!("/"      ,       plus_str("/"  , "/"));
  assert_eq!("a/b"    ,       plus_str("a"  , "b"));
  assert_eq!("a"      ,       plus_str("a"  , "."));
  assert_eq!("b"      ,       plus_str("."  , "b"));
  assert_eq!("."      ,       plus_str("."  , "."));
  assert_eq!("/b"     ,       plus_str("a"  , "/b"));

  assert_eq!("/"      ,       plus_str("/"  , ".."));
  assert_eq!("////"   ,       plus_str("////", ""));
  assert_eq!("."      ,       plus_str("a"  , ".."));
  assert_eq!("a"      ,       plus_str("a/b", ".."));
  assert_eq!("../.."  ,       plus_str(".." , ".."));
  assert_eq!("/c"     ,       plus_str("/"  , "../c"));
  assert_eq!("c"      ,       plus_str("a"  , "../c"));
  assert_eq!("a/c"    ,       plus_str("a/b", "../c"));
  assert_eq!("../../c",       plus_str(".." , "../c"));

  assert_eq!("a//b/d//e",     plus_str("a//b/c", "../d//e"));

  assert_eq!("//foo/var/bar", plus_str("//foo/var", "bar"));
}

#[test]
fn it_will_plus_same_as_ruby_on_windows() {
  assert_eq!("a/b",        windows_plus("a", "b"));
  assert_eq!("a/b",        windows_plus("a\\", "b"));
  assert_eq!("/b",         windows_plus("a", "/b"));
  assert_eq!("\\b",        windows_plus("a", "\\b"));
  assert_eq!("C:/b",       windows_plus("a", "C:/b"));
  assert_eq!("C:/a/b",     windows_plus("C:/a", "b"));
  assert_eq!("C:/b",       windows_plus("C:/a", "../b"));
  assert_eq!("C:/b",       windows_plus("C:/", "../b"));
  assert_eq!("C:\\a/b",    windows_plus("C:\\a", "b"));
  assert_eq!("a",          windows_plus("a\\b", ".."));
  assert_eq!("//a/b/c",    windows_plus("//a/b", "c"));
  assert_eq!("//a/b/c",    windows_plus("//a/b", "../c"));
}
