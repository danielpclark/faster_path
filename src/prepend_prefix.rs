use std::borrow::Cow;
use crate::dirname::dirname;
use crate::path_parsing::{SEP, contains_sep};

pub fn prepend_prefix<'a>(prefix: &'a [u8], relpath: &[u8]) -> Cow<'a, [u8]> {
  if relpath.is_empty() {
    dirname(prefix).into()
  } else if contains_sep(prefix) {
    let prefix_dirname = dirname(prefix);
    match prefix_dirname.last() {
      None => relpath.to_vec().into(),
      Some(&SEP) => [prefix_dirname, relpath].concat().into(),
      _ => [prefix_dirname, &[SEP], relpath].concat().into(),
    }
  } else {
    [prefix, relpath].concat().into()
  }
}
