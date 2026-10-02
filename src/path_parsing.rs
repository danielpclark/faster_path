// The building blocks of Ruby's path handling (`file.c`), for both of the
// sets of rules Ruby is built with:
//
// * Unix: `/` is the only separator.
// * Windows ("DOSISH"): `/` and `\` are separators, a path can start with a
//   drive letter (`C:`) or a UNC prefix (`//server/share`), NTFS ignores
//   trailing dots and spaces and `:stream` names at the end of a file name,
//   and file names compare case-insensitively.
//
// Both are always compiled so each can be tested on any platform;
// `Rules::NATIVE` is the one Ruby uses where this is built.
use memchr::{memchr, memchr2, memrchr, memrchr2};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Rules {
  dosish: bool,
}

// `File::SEPARATOR`, which joins paths on every platform.
pub const SEP_BYTES: &[u8] = b"/";

impl Rules {
  #[cfg_attr(not(test), allow(dead_code))]
  pub const UNIX: Rules = Rules { dosish: false };
  #[cfg_attr(not(test), allow(dead_code))]
  pub const WINDOWS: Rules = Rules { dosish: true };
  pub const NATIVE: Rules = Rules { dosish: cfg!(windows) };

  #[inline(always)]
  pub fn is_dosish(self) -> bool {
    self.dosish
  }

  // Ruby's `isdirsep`
  #[inline(always)]
  pub fn is_sep(self, c: u8) -> bool {
    c == b'/' || (self.dosish && c == b'\\')
  }

  // Byte offset of the last separator.
  #[inline]
  pub fn last_sep_pos(self, bytes: &[u8]) -> Option<usize> {
    if self.dosish { memrchr2(b'/', b'\\', bytes) } else { memrchr(b'/', bytes) }
  }

  // Byte offset of the last byte that isn't a separator.
  #[inline]
  pub fn last_non_sep_pos(self, bytes: &[u8]) -> Option<usize> {
    bytes.iter().rposition(|&c| !self.is_sep(c))
  }

  #[inline]
  pub fn contains_sep(self, bytes: &[u8]) -> bool {
    if self.dosish { memchr2(b'/', b'\\', bytes).is_some() } else { memchr(b'/', bytes).is_some() }
  }

  // `has_drive_letter`: "C:"
  #[inline]
  pub fn has_drive_letter(self, path: &[u8]) -> bool {
    self.dosish && path.len() >= 2 && path[0].is_ascii_alphabetic() && path[1] == b':'
  }

  // `istrailinggarbage`: ignored at the end of NTFS file names.
  #[inline(always)]
  fn is_trailing_garbage(self, c: u8) -> bool {
    self.dosish && (c == b'.' || c == b' ')
  }

  // `isADS`: starts an NTFS alternate data stream name.
  #[inline(always)]
  pub fn is_ads(self, c: u8) -> bool {
    self.dosish && c == b':'
  }

  // `Pathname::SAME_PATHS`, and the comparison `File.basename` uses for
  // extensions (`CASEFOLD_FILESYSTEM`).
  #[inline]
  pub fn same_path(self, a: &[u8], b: &[u8]) -> bool {
    if self.dosish { a.eq_ignore_ascii_case(b) } else { a == b }
  }

  // `skipprefix`: the end of a UNC ("//server/share") or drive letter prefix.
  pub fn skip_prefix(self, path: &[u8]) -> usize {
    if !self.dosish {
      return 0;
    }
    let end = path.len();
    if end >= 2 && self.is_sep(path[0]) && self.is_sep(path[1]) {
      let mut i = 2;
      while i < end && self.is_sep(path[i]) {
        i += 1;
      }
      i = self.next_sep(path, i);
      if i + 1 < end && !self.is_sep(path[i + 1]) {
        i = self.next_sep(path, i + 1);
      }
      return i;
    }
    if self.has_drive_letter(path) {
      return 2;
    }
    0
  }

  // `skiproot`: past a drive letter and the separators after it.
  pub fn skip_root(self, path: &[u8]) -> usize {
    let mut i = if self.has_drive_letter(path) { 2 } else { 0 };
    while i < path.len() && self.is_sep(path[i]) {
      i += 1;
    }
    i
  }

  // `nextdirsep`: the next separator at or after `from`, or the end.
  fn next_sep(self, path: &[u8], from: usize) -> usize {
    let rest = &path[from.min(path.len())..];
    let found = if self.dosish { memchr2(b'/', b'\\', rest) } else { memchr(b'/', rest) };
    found.map_or(path.len(), |pos| from + pos)
  }

  // `strrdirsep`: where the last run of separators that isn't at the end
  // of the path starts.
  pub fn last_separator(self, path: &[u8]) -> Option<usize> {
    let end = self.last_non_sep_pos(path)? + 1;
    let pos = self.last_sep_pos(&path[..end])?;
    Some(path[..pos].iter().rposition(|&c| !self.is_sep(c)).map_or(0, |p| p + 1))
  }

  // `chompdirsep`: where the separators at the end of the path start, or
  // its end.
  #[inline]
  pub fn chomp_dir_sep(self, path: &[u8]) -> usize {
    self.last_non_sep_pos(path).map_or(0, |pos| pos + 1)
  }

  // `ntfs_tail`: the end of an NTFS file name without trailing dots,
  // spaces, separators or `:stream`.
  pub fn ntfs_tail(self, path: &[u8]) -> usize {
    let end = path.len();
    let mut i = 0;
    while i < end && path[i] == b'.' {
      i += 1;
    }
    while i < end && !self.is_ads(path[i]) {
      if self.is_trailing_garbage(path[i]) {
        let last = i;
        i += 1;
        while i < end && self.is_trailing_garbage(path[i]) {
          i += 1;
        }
        if i >= end || self.is_ads(path[i]) {
          return last;
        }
      } else if self.is_sep(path[i]) {
        let last = i;
        i += 1;
        while i < end && self.is_sep(path[i]) {
          i += 1;
        }
        if i >= end {
          return last;
        }
        if self.is_ads(path[i]) {
          i += 1;
        }
      } else {
        i += 1;
      }
    }
    i
  }

  // `File.join(a, b)`
  pub fn join(self, a: &[u8], b: &[u8]) -> Vec<u8> {
    let tail = self.chomp_dir_sep(a);
    if b.first().map_or(false, |&c| self.is_sep(c)) {
      [&a[..tail], b].concat()
    } else if tail == a.len() {
      [a, SEP_BYTES, b].concat()
    } else {
      [a, b].concat()
    }
  }
}

// Where `part`, a slice of `whole`, starts in it.
#[inline]
pub fn offset_in(whole: &[u8], part: &[u8]) -> usize {
  part.as_ptr() as usize - whole.as_ptr() as usize
}

#[cfg(test)]
mod tests {
  use super::*;

  const U: Rules = Rules::UNIX;
  const W: Rules = Rules::WINDOWS;

  #[test]
  fn it_finds_separators() {
    assert_eq!(U.last_sep_pos(b""), None);
    assert_eq!(U.last_sep_pos(b"a/b/c"), Some(3));
    assert_eq!(U.last_sep_pos(b"a/b\\c"), Some(1));
    assert_eq!(W.last_sep_pos(b"a/b\\c"), Some(3));
    assert_eq!(U.last_non_sep_pos(b"///"), None);
    assert_eq!(U.last_non_sep_pos(b"a///"), Some(0));
    assert_eq!(W.last_non_sep_pos(b"a/\\/"), Some(0));
    assert!(U.contains_sep(b"a/b"));
    assert!(!U.contains_sep(b"a\\b"));
    assert!(W.contains_sep(b"a\\b"));
  }

  #[test]
  fn it_skips_prefixes() {
    assert_eq!(U.skip_prefix(b"//a/b/c"), 0);
    assert_eq!(W.skip_prefix(b"//a/b/c"), 5);
    assert_eq!(W.skip_prefix(b"\\\\a\\b\\c"), 5);
    assert_eq!(W.skip_prefix(b"//a/"), 3);
    assert_eq!(W.skip_prefix(b"//a"), 3);
    assert_eq!(W.skip_prefix(b"//"), 2);
    assert_eq!(W.skip_prefix(b"C:/a"), 2);
    assert_eq!(W.skip_prefix(b"/a"), 0);
    assert_eq!(W.skip_root(b"C://a"), 4);
    assert_eq!(U.skip_root(b"C://a"), 0);
    assert_eq!(U.skip_root(b"//a"), 2);
  }

  #[test]
  fn it_finds_the_last_separator() {
    assert_eq!(U.last_separator(b"a//b//"), Some(1));
    assert_eq!(U.last_separator(b"//b"), Some(0));
    assert_eq!(U.last_separator(b"b//"), None);
    assert_eq!(W.last_separator(b"a\\/b"), Some(1));
    assert_eq!(U.chomp_dir_sep(b"a//"), 1);
    assert_eq!(U.chomp_dir_sep(b"///"), 0);
    assert_eq!(U.chomp_dir_sep(b"a"), 1);
  }

  #[test]
  fn it_finds_ntfs_tails() {
    assert_eq!(W.ntfs_tail(b"foo."), 3);
    assert_eq!(W.ntfs_tail(b"foo. ."), 3);
    assert_eq!(W.ntfs_tail(b"foo::$DATA"), 3);
    assert_eq!(W.ntfs_tail(b"foo.bar"), 7);
    assert_eq!(W.ntfs_tail(b"..."), 3);
    assert_eq!(W.ntfs_tail(b"foo/"), 3);
  }

  #[test]
  fn it_joins_like_file_join() {
    assert_eq!(U.join(b"a", b"b"), b"a/b");
    assert_eq!(U.join(b"a/", b"b"), b"a/b");
    assert_eq!(U.join(b"a//", b"b"), b"a//b");
    assert_eq!(U.join(b"a/", b"/b"), b"a/b");
    assert_eq!(U.join(b"a", b""), b"a/");
    assert_eq!(U.join(b"/", b""), b"/");
    assert_eq!(U.join(b"", b"b"), b"/b");
    assert_eq!(W.join(b"a\\", b"b"), b"a\\b");
    assert_eq!(W.join(b"a", b"\\b"), b"a\\b");
    assert_eq!(U.join(b"a", b"\\b"), b"a/\\b");
  }
}
