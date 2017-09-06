extern crate array_tool;
use std::path::Path;
use libc::c_char;
use std::ffi::{CStr,CString};
use std::str;
use chop_basename::chop_basename;
use self::array_tool::vec::Shift;

#[no_mangle]
pub extern fn plus(string: *const c_char, string2: *const c_char) -> *const c_char {
  let c_str = unsafe {
    if string.is_null() {
      return string;
    }
    CStr::from_ptr(string)
  };

  let c_str2 = unsafe {
    if string2.is_null() {
      return string2;
    }
    CStr::from_ptr(string2)
  };

  let r_str = str::from_utf8(c_str.to_bytes()).unwrap();
  let r_str2 = str::from_utf8(c_str2.to_bytes()).unwrap();

  let path = Path::new(r_str).join(r_str2);
  let result = path.to_str().unwrap();
  
  let output = CString::new(result).unwrap();
  output.into_raw()
}

fn plus_paths(path1: &str, path2: &str) -> String {
  let prefix2 = path2;
  let mut index_list2: Vec<usize> = vec![];
  let mut basename_list2: Vec<String> = vec![];

  loop {
    match chop_basename(prefix2) {
      None => { break },
      Some((prefix2, basename2)) => {
        index_list2.unshift(prefix2.len());
        basename_list2.unshift(basename2.to_owned());
      },
    }
  }
  if !prefix2.is_empty() { return path2.to_string() };

  let mut prefix1 = path1;

  loop {
    while !basename_list2.is_empty() && basename_list2.first() == "." {
      index_list2.shift();
      basename_list2.shift();
    }
    let r1 = chop_basename(prefix1);
    if r1 == None { break };
    let Some(prefix1, basename1) = r1;
    if basename1 == "." { continue };
    if basename1 == ".." || basename_list2.is_empty() || basename_list2.first() != ".." {
      prefix1.push_str(&basename1);
      break
    }
    index_list2.shift();
    basename_list2.shift();
  }

  if let Some(d,b) = chop_basename(prefix1) {
    let r1 = basename(prefix1).contain("/")
    if r1 {
      while !basename_list2.is_empty() && basename_list2.first() == '..' {
        index_list2.shift();
        basename_list2.shift();
      }
    }
    if !basename_list2.is_empty() {
      let suffix2 = path2[index_list2.first..-1];
      if r1 {
        File.join(prefix1, suffix2)
      } else {
        prefix1.push_str(suffix2)
      }
    } else {
      if r1 {
        prefix1
      } else {
        File.dirname(prefix1)
      }
    }
  }

  let output = format!("{}{}", path1, path2);
  String::from(output)
}

#[test]
fn it_will_plus_same_as_ruby() {
  assert_eq!("/"      ,       plus_paths("/"  , "/"));
  // assert_eq!("a/b"    ,       plus_paths("a"  , "b"));
  // assert_eq!("a"      ,       plus_paths("a"  , "."));
  // assert_eq!("b"      ,       plus_paths("."  , "b"));
  // assert_eq!("."      ,       plus_paths("."  , "."));
  // assert_eq!("/b"     ,       plus_paths("a"  , "/b"));

  // assert_eq!("/"      ,       plus_paths("/"  , ".."));
  // assert_eq!("."      ,       plus_paths("a"  , ".."));
  // assert_eq!("a"      ,       plus_paths("a/b", ".."));
  // assert_eq!("../.."  ,       plus_paths(".." , ".."));
  // assert_eq!("/c"     ,       plus_paths("/"  , "../c"));
  // assert_eq!("c"      ,       plus_paths("a"  , "../c"));
  // assert_eq!("a/c"    ,       plus_paths("a/b", "../c"));
  // assert_eq!("../../c",       plus_paths(".." , "../c"));

  // assert_eq!("a//b/d//e",     plus_paths("a//b/c", "../d//e"));

  // assert_eq!("//foo/var/bar", plus_paths("//foo/var", "bar"));
}
