use std::path::MAIN_SEPARATOR;
use path_parsing::extract_last_path_segment;

pub fn basename(pth: &str, ext: &str) -> String {
  // Known edge case
  if !pth.is_empty() && pth.bytes().all(|b| b == (MAIN_SEPARATOR as u8)) {
    return MAIN_SEPARATOR.to_string();
  }

  let mut name = extract_last_path_segment(pth);

  if ext == ".*" {
    if let Some(dot_i) = name.rfind('.') {
      name = &name[0..dot_i];
    }
  } else if name.ends_with(ext) {
    name = &name[..name.len() - ext.len()];
  };
  name.to_string()
}

#[test]
fn edge_case_all_seps() {
  assert_eq!("/", basename("///", ".*"));
}
