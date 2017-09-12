use std::path::MAIN_SEPARATOR;
use std::str;

pub fn chop_basename(input: &str) -> Option<(String,String)> {
  if input.is_empty() {
    return None;
  }

  let mut offset = 0;
  let mut trailing_slashes = input.chars().rev();
  loop {
    match trailing_slashes.next() {
      Some(MAIN_SEPARATOR) => { offset = offset + 1 },
      _                    => { break               },
    }
  }

  let input = &input[0..input.len()-offset];
  let base = input.rsplit_terminator(MAIN_SEPARATOR).nth(0).unwrap_or("");
  let directory = &input[0..input.len()-base.len()];

  if directory.is_empty() && (base.is_empty() || base.chars().next().unwrap() == MAIN_SEPARATOR) {
    return None
  };

  Some((directory.to_string(), base.to_string()))
}

#[test]
fn it_chops_the_basename_and_dirname() {
  assert_eq!(chop_basename(&""[..]), None);
  assert_eq!(chop_basename(&"/"[..]), None);
  assert_eq!(
    chop_basename(&"."[..]),
    Some(("".to_string(), ".".to_string()))
  );
  assert_eq!(
    chop_basename(&"asdf/asdf"[..]),
    Some(("asdf/".to_string(), "asdf".to_string()))
  );
  assert_eq!(
    chop_basename(&"asdf.txt"[..]),
    Some(("".to_string(), "asdf.txt".to_string()))
  );
  assert_eq!(
    chop_basename(&"asdf/"[..]),
    Some(("".to_string(), "asdf".to_string()))
  );
  assert_eq!(
    chop_basename(&"/asdf/"[..]),
    Some(("/".to_string(), "asdf".to_string()))
  );
}

