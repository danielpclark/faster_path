extern crate array_tool;
use path_parsing::extract_last_path_segment;
use self::array_tool::string::Squeeze;

pub fn basename(pth: &str, ext: &str) -> String {
  // Known edge case
  match &pth.squeeze("/")[..] {
    "/" => { return "/".to_string() }
    _ => {}
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

