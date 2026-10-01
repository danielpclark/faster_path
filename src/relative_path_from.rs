use crate::helpers::is_same_path;
use crate::path_parsing::SEP_BYTES;
use crate::cleanpath_aggressive::cleanpath_aggressive;
use crate::chop_basename::chop_basename;

#[derive(Debug, PartialEq)]
pub enum RelativePathError {
  // Holds the destination prefix and the cleaned base directory.
  DifferentPrefix(Vec<u8>, Vec<u8>),
  // Holds the cleaned base directory.
  BaseDirectoryHasDotDot(Vec<u8>),
}

pub fn relative_path_from(dest: &[u8], base: &[u8]) -> Result<Vec<u8>, RelativePathError> {
  let dest_directory = cleanpath_aggressive(dest);
  let base_directory = cleanpath_aggressive(base);

  let (dest_prefix, mut dest_names) = to_names(&dest_directory);
  let (base_prefix, mut base_names) = to_names(&base_directory);

  if !is_same_path(dest_prefix, base_prefix) {
    return Err(RelativePathError::DifferentPrefix(dest_prefix.to_vec(), base_directory.to_vec()));
  }

  // Remove the shared leading names (stored last, as the names are collected in reverse)
  {
    let num_same = dest_names.iter().rev().zip(base_names.iter().rev()).
        take_while(|&(dest, base)| is_same_path(dest, base)).count();
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
fn to_names(path: &[u8]) -> (&[u8], Vec<&[u8]>) {
  let mut result: Vec<&[u8]> = vec![];
  let mut prefix = path;
  while let Some((dest, basename)) = chop_basename(prefix) {
    prefix = dest;
    if basename != b"." {
      result.push(basename);
    }
  }
  (prefix, result)
}

#[cfg(test)]
fn relative_str(dest: &str, base: &str) -> Option<String> {
  relative_path_from(dest.as_bytes(), base.as_bytes()).ok().map(|path| String::from_utf8(path).unwrap())
}

#[test]
fn it_finds_relative_paths() {
  assert_eq!(relative_str("a", "b").unwrap(), "../a");
  assert_eq!(relative_str("/a/b/c/d", "/a/b").unwrap(), "c/d");
  assert_eq!(relative_str("/a/b", "/a/b/c/d").unwrap(), "../..");
  assert_eq!(relative_str(".", ".").unwrap(), ".");
  assert_eq!(relative_str("a/b/c", "a/d").unwrap(), "../b/c");
}

#[test]
fn it_rejects_incompatible_paths() {
  assert_eq!(
    relative_path_from(b"/", b"."),
    Err(RelativePathError::DifferentPrefix(b"/".to_vec(), b".".to_vec()))
  );
  assert_eq!(
    relative_path_from(b"a", b".."),
    Err(RelativePathError::BaseDirectoryHasDotDot(b"..".to_vec()))
  );
}
