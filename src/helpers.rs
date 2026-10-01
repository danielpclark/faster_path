#[cfg(windows)]
#[inline]
pub fn is_same_path(a: &[u8], b: &[u8]) -> bool {
  a.eq_ignore_ascii_case(b)
}

#[cfg(not(windows))]
#[inline]
pub fn is_same_path(a: &[u8], b: &[u8]) -> bool {
  a == b
}
