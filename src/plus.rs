extern crate array_tool;
use std::path::Path;
use std::str;
use chop_basename::chop_basename;
use basename::basename;
use dirname::dirname;
use self::array_tool::vec::Shift;
use std::ops::Index;

pub fn plus_paths(path1: &str, path2: &str) -> String {
  let mut prefix2 = path2.to_string();
  let mut index_list2: Vec<usize> = vec![];
  let mut basename_list2: Vec<String> = vec![];

  loop {
    match chop_basename(&prefix2.clone()[..]) {
      None => { break },
      Some((pfx2, basename2)) => {
        prefix2 = pfx2.to_string();
        index_list2.unshift(pfx2.len());
        basename_list2.unshift(basename2.to_owned());
      },
    }
  }
  if !prefix2.is_empty() {
    return path2.to_string()
  };

  let mut prefix1 = path1.to_string();

  loop {
    while !basename_list2.is_empty() && basename_list2.first().unwrap() == "." {
      index_list2.shift();
      basename_list2.shift();
    }
    match chop_basename(&prefix1.clone()[..]) {
      None => { break },
      Some((pfx1, basename1)) => {
        prefix1 = pfx1.to_string();
        if basename1 == "." { continue };
        if basename1 == ".." || basename_list2.is_empty() || basename_list2.first().unwrap() != ".." {
          prefix1.push_str(&basename1);
          break
          }
        }
    }
    index_list2.shift();
    basename_list2.shift();
  }

  let result: String;

  let mut r1 = if let Some((_,_)) = chop_basename(&prefix1[..]) {true} else {false};

  if !r1 {
    r1 = basename(&prefix1[..], "").contains("/");
    if r1 {
      while !basename_list2.is_empty() && basename_list2.first().unwrap() == ".." {
        index_list2.shift();
        basename_list2.shift();
      }
    }
  }
  if !basename_list2.is_empty() {
    let suffix2 = path2.index(index_list2.first().unwrap().to_owned()..);
    if r1 {
      result = Path::new(&prefix1).join(Path::new(&suffix2)).to_str().unwrap().to_string();
    } else {
      prefix1.push_str(&suffix2);
      result = prefix1.to_string();
    }
  } else {
    if r1 {
      result = prefix1.to_string();
    } else {
      result = dirname(&prefix1[..]).to_string();
    }
  }

  String::from(result)
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
