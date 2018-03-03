use std::path::MAIN_SEPARATOR;
use std::str;

pub fn chop_basename<'a>(input: &'a str) -> Option<(&'a str, &'a str)> {
  if input.is_empty() {
    return None;
  }

  let input = input.trim_right_matches(MAIN_SEPARATOR);
  let end = input.rsplitn(2, MAIN_SEPARATOR).nth(0).unwrap().len();
  let base = &input[input.len()-end..input.len()];
  let directory = &input[0..input.len()-base.len()];

  if directory.is_empty() && (base.is_empty() || base.chars().next().unwrap() == MAIN_SEPARATOR) {
    return None
  };

  Some((directory, base))
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
}

