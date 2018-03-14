use path_parsing::{find_last_sep_pos, find_last_non_sep_pos, find_last_word};
use std::ops::Range;
use extname::extname;
use memrnchr::memrnchr;
extern crate memchr;

pub fn basename<'a>(path: &'a str, ext: &str) -> &'a str {
  let bytes = path.as_bytes();
  let range: Range<usize>;

  if ext.is_empty() {
    range = find_last_word(bytes);
  } else {
    let extension = ext.as_bytes();
    let mut end = find_last_non_sep_pos(&bytes).map(|v| v+1).unwrap_or(bytes.len());

    if extension == b".*" {
      let e = extname(&path[..end]);

      end -= if e.is_empty() {
        // trailing dot on basename
        memrnchr(b'.', &bytes[..end]).map(|v| bytes[..end].len() - v - 1).unwrap_or(0)
      } else {
        e.len()
      };

      range = find_last_word(&bytes[..end]);
    } else if ext.len() > path.len() {
      range = find_last_word(&bytes);
    } else {
      let mut y = extension.iter().rev();
      let mut z = bytes[..end].iter().rev();
      loop {
        match (y.next(), z.next()) {
          (_, None) | (None, Some(_)) => { 
            range = find_last_word(&bytes[..end]);
            break
          },
          (Some(c), Some(d)) if c != d => {
            range = find_last_word(&bytes[..end]);
            break
          },
          _ => {
            end -= 1;
          }
        }
      }
    }
  }

  if range.start == range.end {
    if let Some(_) = find_last_sep_pos(bytes) {
      return "/"
    }
  }

  &path[range]
}

#[test]
fn empty() {
  assert_eq!(basename("", ""), "");
}

#[test]
fn sep() {
  assert_eq!(basename("/", ""), "/");
  assert_eq!(basename("//", ""), "/");
}

#[test]
fn trailing_dot() {
  assert_eq!(basename("file.test.", ""), "file.test.");
  assert_eq!(basename("file.test.", ".*"), "file.test");
}

#[test]
fn dots() {
  assert_eq!(".", basename(".", ""));
  assert_eq!("..", basename("..", ""));
  assert_eq!(".", basename("..", "."));
  assert_eq!("..", basename("..", ".*"));
  assert_eq!("..", basename("..", "..."));
}

#[test]
fn some_ruby_spec_trailing_sep_cases() {
  assert_eq!("base", basename("dir//base.c/", ".c"));
  assert_eq!("foo", basename("foo.rb/", ".rb"));
  assert_eq!("base", basename("dir//base.c/", ".*"));
  assert_eq!("bar", basename("bar.rb///", ".*"));
}

#[test]
fn absolute() {
  assert_eq!(basename("/a/b///c", ""), "c");
}

#[test]
fn trailing_slashes_absolute() {
  assert_eq!(basename("/a/b///c//////", ""), "c");
}

#[test]
fn relative() {
  assert_eq!(basename("b///c", ""), "c");
}

#[test]
fn trailing_slashes_relative() {
  assert_eq!(basename("b/c//", ""), "c");
}

#[test]
fn root() {
  assert_eq!(basename("//c", ""), "c");
}

#[test]
fn trailing_slashes_root() {
  assert_eq!(basename("//c//", ""), "c");
}

#[test]
fn trailing_slashes_relative_root() {
  assert_eq!(basename("c//", ""), "c");
}

#[test]
fn edge_case_all_seps() {
  assert_eq!("/", basename("///", ".*"));
}
