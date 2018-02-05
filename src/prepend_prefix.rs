use dirname::dirname;
use basename::basename;
use std::path::MAIN_SEPARATOR as SEP;

pub fn prepend_prefix(prefix: &str, relpath: &str) -> String {
  let mut prefix: String = prefix.to_string();
  if relpath.is_empty() {
    dirname(&prefix).to_string()
  } else if prefix.contains(&SEP.to_string()[..]) {
    prefix = dirname(&prefix).to_string();
    if basename(&format!("{}{}", &prefix, "a"), "") != "a" {
      prefix = format!("{}{}", prefix, &SEP);
    }
    format!("{}{}", prefix, relpath)
  } else {
    format!("{}{}", prefix, relpath)
  }
}
