#![feature(test)]
extern crate test;
use test::Bencher;

include!("../src/lib.rs");

use basename::basename;
use path_parsing::{find_last_sep_pos, find_last_non_sep_pos, find_last_word};
use std::ops::Range;
use extname::extname;
use memrnchr::memrnchr;
extern crate memchr;

#[bench]
fn basename_version1_benchmark(b: &mut Bencher) {
  let parameters = [
    ("/a/b///c//////", ""),
    ("file.test.", ".*"),
    ("", ""),
    ("///", ""),
    ("..", ""),
    ("..", "."),
    ("..", "...")
  ];

  b.iter(|| {
    parameters.iter().for_each(|&(a,b)| {
      basename(a,b);
    })
  })
}

#[bench]
fn basename_version2_benchmark(b: &mut Bencher) {
  let parameters = [
    ("/a/b///c//////", ""),
    ("file.test.", ".*"),
    ("", ""),
    ("///", ""),
    ("..", ""),
    ("..", "."),
    ("..", "...")
  ];

  b.iter(|| {
    parameters.iter().for_each(|&(a,b)| {
      basename_version2(a,b);
    })
  })
}

pub fn basename_version2<'a>(path: &'a str, ext: &str) -> &'a str {
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
