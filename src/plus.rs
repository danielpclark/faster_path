use std::borrow::Cow;
use std::str;
use std::path::MAIN_SEPARATOR;

use chop_basename::chop_basename;
use path_parsing::SEP;

pub fn plus_paths<'a>(path1: &'a str, path2: &str) -> Cow<'a, str> {
  let mut prefix2 = path2;
  let mut index_list2: Vec<usize> = vec![];
  let mut basename_list2: Vec<&str> = vec![];
  loop {
    match chop_basename(prefix2) {
      None => { break; }
      Some((pfx2, basename2)) => {
        prefix2 = pfx2;
        index_list2.push(pfx2.len());
        basename_list2.push(basename2);
      }
    }
  }
  if !prefix2.is_empty() {
    return path2.to_string().into();
  };

  let result_prefix: Cow<str>;
  let mut prefix1 = path1;
  loop {
    let mut new_len = basename_list2.len() - count_trailing(".", &basename_list2);
    index_list2.truncate(new_len);
    basename_list2.truncate(new_len);
    match chop_basename(prefix1) {
      None => {
        result_prefix = prefix1.into();
        break;
      }
      Some((pfx1, basename1)) => {
        prefix1 = pfx1;
        if basename1 == "." { continue; };
        if basename1 == ".." || basename_list2.last() != Some(&"..") {
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

  if !result_prefix.is_empty() && result_prefix.as_bytes().iter().cloned().all(|b| b == SEP) {
    let new_len = basename_list2.len() - count_trailing("..", &basename_list2);
    index_list2.truncate(new_len);
    basename_list2.truncate(new_len);
  }
  if let Some(last_index2) = index_list2.last() {
    let suffix = &path2[*last_index2..];
    match (result_prefix.as_bytes().last(), suffix.as_bytes().first()) {
      (Some(&SEP), Some(&SEP)) => [&result_prefix, &suffix[1..]].concat().into(),
      (Some(&SEP), Some(_)) | (Some(_), Some(&SEP)) => [&result_prefix, suffix].concat().into(),
      (None, Some(_)) => suffix.to_string().into(),
      _ => format!("{}{}{}", result_prefix.as_ref(), MAIN_SEPARATOR, suffix).into(),
    }
  } else {
    if result_prefix.is_empty() {
      ".".into()
    } else {
      result_prefix
    }
  }
}

#[inline(always)]
fn count_trailing(x: &str, xs: &Vec<&str>) -> usize {
  xs.iter().rev().take_while(|&c| c == &x).count()
}

#[test]
fn it_will_plus_same_as_ruby() {
  assert_eq!("/"      ,       plus_paths("/"  , "/"));
  assert_eq!("a/b"    ,       plus_paths("a"  , "b"));
  assert_eq!("a"      ,       plus_paths("a"  , "."));
  assert_eq!("b"      ,       plus_paths("."  , "b"));
  assert_eq!("."      ,       plus_paths("."  , "."));
  assert_eq!("/b"     ,       plus_paths("a"  , "/b"));

  assert_eq!("/"      ,       plus_paths("/"  , ".."));
  assert_eq!("////"   ,       plus_paths("////", ""));
  assert_eq!("."      ,       plus_paths("a"  , ".."));
  assert_eq!("a"      ,       plus_paths("a/b", ".."));
  assert_eq!("../.."  ,       plus_paths(".." , ".."));
  assert_eq!("/c"     ,       plus_paths("/"  , "../c"));
  assert_eq!("c"      ,       plus_paths("a"  , "../c"));
  assert_eq!("a/c"    ,       plus_paths("a/b", "../c"));
  assert_eq!("../../c",       plus_paths(".." , "../c"));

  assert_eq!("a//b/d//e",     plus_paths("a//b/c", "../d//e"));

  assert_eq!("//foo/var/bar", plus_paths("//foo/var", "bar"));
}
