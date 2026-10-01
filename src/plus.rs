use std::borrow::Cow;

use crate::chop_basename::chop_basename;
use crate::path_parsing::SEP;

pub fn plus_paths<'a>(path1: &'a [u8], path2: &[u8]) -> Cow<'a, [u8]> {
  let mut prefix2 = path2;
  let mut index_list2: Vec<usize> = vec![];
  let mut basename_list2: Vec<&[u8]> = vec![];
  while let Some((pfx2, basename2)) = chop_basename(prefix2) {
    prefix2 = pfx2;
    index_list2.push(pfx2.len());
    basename_list2.push(basename2);
  }
  if !prefix2.is_empty() {
    return path2.to_vec().into();
  };

  let result_prefix: Cow<'a, [u8]>;
  let mut prefix1 = path1;
  loop {
    let mut new_len = basename_list2.len() - count_trailing(b".", &basename_list2);
    index_list2.truncate(new_len);
    basename_list2.truncate(new_len);
    match chop_basename(prefix1) {
      None => {
        result_prefix = prefix1.into();
        break;
      }
      Some((pfx1, basename1)) => {
        prefix1 = pfx1;
        if basename1 == b"." { continue; };
        if basename1 == b".." || basename_list2.last() != Some(&&b".."[..]) {
          result_prefix = [prefix1, basename1].concat().into();
          break;
        }
      }
    }
    if new_len > 0 {
      new_len -= 1;
      index_list2.truncate(new_len);
      basename_list2.truncate(new_len);
    }
  }

  if !result_prefix.is_empty() && result_prefix.iter().all(|&b| b == SEP) {
    let new_len = basename_list2.len() - count_trailing(b"..", &basename_list2);
    index_list2.truncate(new_len);
    basename_list2.truncate(new_len);
  }
  if let Some(&last_index2) = index_list2.last() {
    let suffix = &path2[last_index2..];
    match (result_prefix.last(), suffix.first()) {
      (Some(&SEP), Some(&SEP)) => [&result_prefix[..], &suffix[1..]].concat().into(),
      (Some(&SEP), Some(_)) | (Some(_), Some(&SEP)) => [&result_prefix[..], suffix].concat().into(),
      (None, Some(_)) => suffix.to_vec().into(),
      _ => [&result_prefix[..], &[SEP], suffix].concat().into(),
    }
  } else if result_prefix.is_empty() {
    Cow::Borrowed(b".")
  } else {
    result_prefix
  }
}

#[inline(always)]
fn count_trailing(x: &[u8], xs: &[&[u8]]) -> usize {
  xs.iter().rev().take_while(|&&c| c == x).count()
}

#[cfg(test)]
fn plus_str(path1: &str, path2: &str) -> String {
  String::from_utf8(plus_paths(path1.as_bytes(), path2.as_bytes()).into_owned()).unwrap()
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
