use crate::chop_basename::chop_basename;
use crate::cleanpath_aggressive::cleanpath_aggressive;
use crate::path_parsing::{Rules, SEP_BYTES};

#[derive(Debug, PartialEq)]
pub enum RelativePathError {
  // Holds the destination prefix and the cleaned base directory.
  DifferentPrefix(Vec<u8>, Vec<u8>),
  // Holds the cleaned base directory.
  BaseDirectoryHasDotDot(Vec<u8>),
}

// Pathname's `relative_path_from`
pub fn relative_path_from(rules: Rules, dest: &[u8], base: &[u8]) -> Result<Vec<u8>, RelativePathError> {
  let dest_directory = cleanpath_aggressive(rules, dest);
  let base_directory = cleanpath_aggressive(rules, base);

  let (dest_prefix, mut dest_names) = to_names(rules, &dest_directory);
  let (base_prefix, mut base_names) = to_names(rules, &base_directory);

  if !rules.same_path(dest_prefix, base_prefix) {
    return Err(RelativePathError::DifferentPrefix(dest_prefix.to_vec(), base_directory.to_vec()));
  }

  // Remove the shared leading names (stored last, as the names are collected in reverse)
  {
    let num_same = dest_names.iter().rev().zip(base_names.iter().rev()).
        take_while(|&(dest, base)| rules.same_path(dest, base)).count();
    dest_names.truncate(dest_names.len() - num_same);
    base_names.truncate(base_names.len() - num_same);
  };

  if base_names.contains(&&b".."[..]) {
    return Err(RelativePathError::BaseDirectoryHasDotDot(base_directory.to_vec()));
  }

  if base_names.is_empty() && dest_names.is_empty() {
    Ok(b".".to_vec())
  } else {
    let names: Vec<&[u8]> = std::iter::repeat(&b".."[..]).take(base_names.len()).
      chain(dest_names.into_iter().rev()).collect();
    Ok(names.join(SEP_BYTES))
  }
}

#[inline(always)]
fn to_names(rules: Rules, path: &[u8]) -> (&[u8], Vec<&[u8]>) {
  let mut result: Vec<&[u8]> = vec![];
  let mut prefix = path;
  while let Some((dest, basename)) = chop_basename(rules, prefix) {
    prefix = dest;
    if basename != b"." {
      result.push(basename);
    }
  }
  (prefix, result)
}

#[cfg(test)]
fn relative_str(rules: Rules, dest: &str, base: &str) -> Option<String> {
  relative_path_from(rules, dest.as_bytes(), base.as_bytes()).ok().map(|path| String::from_utf8(path).unwrap())
}

#[test]
fn it_finds_relative_paths() {
  let u = Rules::UNIX;
  assert_eq!(relative_str(u, "a", "b").unwrap(), "../a");
  assert_eq!(relative_str(u, "/a/b/c/d", "/a/b").unwrap(), "c/d");
  assert_eq!(relative_str(u, "/a/b", "/a/b/c/d").unwrap(), "../..");
  assert_eq!(relative_str(u, ".", ".").unwrap(), ".");
  assert_eq!(relative_str(u, "a/b/c", "a/d").unwrap(), "../b/c");
}

#[test]
fn it_rejects_incompatible_paths() {
  assert_eq!(
    relative_path_from(Rules::UNIX, b"/", b"."),
    Err(RelativePathError::DifferentPrefix(b"/".to_vec(), b".".to_vec()))
  );
  assert_eq!(
    relative_path_from(Rules::UNIX, b"a", b".."),
    Err(RelativePathError::BaseDirectoryHasDotDot(b"..".to_vec()))
  );
}

#[test]
fn it_finds_relative_paths_on_windows() {
  let w = Rules::WINDOWS;
  assert_eq!(relative_str(w, "C:\\a\\b", "c:/a").unwrap(), "b");
  assert_eq!(relative_str(w, "C:/A/b", "c:/a/c").unwrap(), "../b");
  assert_eq!(relative_str(w, "//a/b/c/d", "//a/b/c").unwrap(), "d");
  assert_eq!(relative_str(w, "C:/a", "D:/a"), None);
  assert_eq!(relative_str(Rules::UNIX, "a/B", "a/b").unwrap(), "../B");
}
