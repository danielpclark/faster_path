use path_parsing::{last_non_sep_i, last_sep_i};

pub fn basename<'a>(pth: &'a str, ext: &str) -> &'a str {
  let name_end = (last_non_sep_i(pth) + 1) as usize;
  // Known edge case, all '/'.
  if !pth.is_empty() && name_end == 0 {
    return &pth[..1];
  }

  let mut name = &pth[(last_sep_i(pth, name_end as isize) + 1) as usize..name_end];

  if ext == ".*" {
    if let Some(dot_i) = name.rfind('.') {
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
