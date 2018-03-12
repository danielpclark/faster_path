extern crate memchr;
use self::memchr::memchr;
use dirname::dirname;
use path_parsing::SEP;

pub fn prepend_prefix(prefix: &str, relpath: &str) -> String {
  if relpath.is_empty() {
    dirname(prefix).to_string()
  } else if memchr(SEP, prefix.as_bytes()) != None {
    let prefix_dirname = dirname(prefix);
    match prefix_dirname.as_bytes().last() {
      None => relpath.to_string(),
      Some(&SEP) => format!("{}{}", prefix_dirname, relpath),
      _ => format!("{}{}{}", prefix_dirname, SEP, relpath)
    }
  } else {
    format!("{}{}", prefix, relpath)
  }
}
