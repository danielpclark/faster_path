use crate::path_parsing::{find_last_non_sep_pos, find_last_sep_pos};

pub fn chop_basename(input: &[u8]) -> Option<(&[u8], &[u8])> {
  let len = find_last_non_sep_pos(input)? + 1;
  let base_start = find_last_sep_pos(&input[..len]).map_or(0, |pos| pos + 1);
  if base_start == len {
    return None;
  }
  Some((&input[0..base_start], &input[base_start..len]))
}

#[cfg(test)]
fn chop_basename_str(input: &str) -> Option<(&str, &str)> {
  chop_basename(input.as_bytes()).map(|(dirname, basename)| {
    (std::str::from_utf8(dirname).unwrap(), std::str::from_utf8(basename).unwrap())
  })
}

#[test]
fn it_chops_the_basename_and_dirname() {
  assert_eq!(chop_basename_str(""),           None );
  assert_eq!(chop_basename_str("/"),          None );
  assert_eq!(chop_basename_str("."),          Some(("", ".")) );
  assert_eq!(chop_basename_str("asdf/asdf"),  Some(("asdf/",     "asdf")) );
  assert_eq!(chop_basename_str("asdf.txt"),   Some(("",      "asdf.txt")) );
  assert_eq!(chop_basename_str("asdf/"),      Some(("",          "asdf")) );
  assert_eq!(chop_basename_str("/asdf/"),     Some(("/",         "asdf")) );
  assert_eq!(chop_basename_str("a///b"),      Some(("a///",         "b")) );
  assert_eq!(chop_basename_str("a///b//"),    Some(("a///",         "b")) );
  assert_eq!(chop_basename_str("/a///b//"),   Some(("/a///",        "b")) );
  assert_eq!(chop_basename_str("/a///b//"),   Some(("/a///",        "b")) );

  assert_eq!(chop_basename_str("./../..///.../..//"), Some(("./../..///.../", "..")));
}

#[test]
fn it_chops_non_utf8_paths() {
  assert_eq!(chop_basename(b"\xff/\xfe\xfd/"), Some((&b"\xff/"[..], &b"\xfe\xfd"[..])));
}
