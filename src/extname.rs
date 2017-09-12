use path_parsing::extract_last_path_segment;

pub fn extname(pth: &str) -> String {
  let name = extract_last_path_segment(pth);

  if let Some(dot_i) = name.rfind('.') {
    if dot_i > 0 && dot_i < name.len() - 1 && name[..dot_i].chars().rev().next().unwrap() != '.' {
      return String::from(&name[dot_i..])
    }
  }

  String::from("")
}
