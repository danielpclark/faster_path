use path_parsing::{find_last_non_sep_pos, find_last_sep_pos};
use std::str;

pub fn chop_basename<'a>(input: &'a str) -> Option<(&'a str, &'a str)> {
  let bytes = input.as_bytes();
  let len = find_last_non_sep_pos(&bytes)? + 1;
  let base_start = find_last_sep_pos(&bytes[..len]).map_or(0, |pos| pos + 1);
  if base_start == len {
    return None;
  }
  Some((&input[0..base_start], &input[base_start..len]))
}

#[test]
fn it_chops_the_basename_and_dirname() {
  assert_eq!(chop_basename(""),           None );
  assert_eq!(chop_basename("/"),          None );
  assert_eq!(chop_basename("."),          Some(("", ".")) );
  assert_eq!(chop_basename("asdf/asdf"),  Some(("asdf/",     "asdf")) );
  assert_eq!(chop_basename("asdf.txt"),   Some(("",      "asdf.txt")) );
  assert_eq!(chop_basename("asdf/"),      Some(("",          "asdf")) );
  assert_eq!(chop_basename("/asdf/"),     Some(("/",         "asdf")) );
  assert_eq!(chop_basename("a///b"),      Some(("a///",         "b")) );
  assert_eq!(chop_basename("a///b//"),    Some(("a///",         "b")) );
  assert_eq!(chop_basename("/a///b//"),   Some(("/a///",        "b")) );
  assert_eq!(chop_basename("/a///b//"),   Some(("/a///",        "b")) );

  assert_eq!(chop_basename("./../..///.../..//"), Some(("./../..///.../", "..")));
}

