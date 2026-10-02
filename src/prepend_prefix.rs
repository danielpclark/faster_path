use std::borrow::Cow;

use crate::cleanpath_conservative::add_trailing_separator;
use crate::dirname::dirname;
use crate::path_parsing::Rules;

// Pathname's `prepend_prefix`
pub fn prepend_prefix<'a>(rules: Rules, prefix: &'a [u8], relpath: &[u8]) -> Cow<'a, [u8]> {
  if relpath.is_empty() {
    dirname(rules, prefix)
  } else if rules.contains_sep(prefix) {
    let prefix = add_trailing_separator(rules, dirname(rules, prefix));
    Cow::Owned([&prefix[..], relpath].concat())
  } else {
    Cow::Owned([prefix, relpath].concat())
  }
}
