use std::borrow::Cow;
use dirname::dirname;
use path_parsing::{SEP, contains_sep};
use std::path::MAIN_SEPARATOR;

pub fn prepend_prefix<'a>(prefix: &'a str, relpath: &str) -> Cow<'a, str> {
  if relpath.is_empty() {
    dirname(prefix).into()
  } else if contains_sep(prefix.as_bytes()) {
    let prefix_dirname = dirname(prefix);
    match prefix_dirname.as_bytes().last() {
      None => relpath.to_string().into(),
      Some(&SEP) => format!("{}{}", prefix_dirname, relpath).into(),
      _ => format!("{}{}{}", prefix_dirname, MAIN_SEPARATOR, relpath).into()
    }
  } else {
    format!("{}{}", prefix, relpath).into()
  }
}
