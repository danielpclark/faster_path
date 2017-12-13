extern crate memchr;
use self::memchr::memrchr;
use path_parsing::{SEP, SEP_STR, last_non_sep_i, last_non_sep_i_before};

pub fn dirname(path: &str) -> &str {
  if path.is_empty() { return "."; }
  let non_sep_i = last_non_sep_i(path);
  if non_sep_i == -1 { return SEP_STR; }
  return match memrchr(SEP, &path.as_bytes()[..non_sep_i as usize]) {
    None => ".",
    Some(0) => SEP_STR,
    Some(sep_i) => {
      let non_sep_i2 = last_non_sep_i_before(path, sep_i as isize);
      if non_sep_i2 != -1 {
        &path[..(non_sep_i2 + 1) as usize]
      } else {
        SEP_STR
      }
    }
  }
}

#[test]
fn returns_dot_for_empty_string(){
  assert_eq!(dirname(""), ".".to_string());
}
