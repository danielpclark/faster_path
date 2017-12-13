extern crate memchr;
use self::memchr::memrchr;
use path_parsing::{SEP, last_non_sep_i};

pub fn basename<'a>(pth: &'a str, ext: &str) -> &'a str {
  let name_end = (last_non_sep_i(pth) + 1) as usize;
  // Known edge case, all '/'.
  if !pth.is_empty() && name_end == 0 {
    return &pth[..1];
  }
  let name_start = memrchr(SEP, &pth.as_bytes()[..name_end]).unwrap_or(0);
  let mut name = &pth[name_start..name_end];
  if ext == ".*" {
    if let Some(dot_i) = memrchr('.' as u8, name.as_bytes()) {
      name = &name[..dot_i];
    }
  } else if name.ends_with(ext) {
    name = &name[..name.len() - ext.len()];
  };
  name
}

#[test]
fn edge_case_all_seps() {
  assert_eq!("/", basename("///", ".*"));
}
