#![feature(test)]
extern crate libc;
extern crate test;
use std::path::MAIN_SEPARATOR;

static SEP: u8 = MAIN_SEPARATOR as u8;

pub fn extract_last_path_segment_str(path: &str) -> &str {
  path.trim_right_matches(MAIN_SEPARATOR).rsplit(MAIN_SEPARATOR).next().unwrap_or("")
}

pub fn extract_last_path_segment_bytes(mut path: &str) -> &str {
  // Works with bytes directly because MAIN_SEPARATOR is always in the ASCII 7-bit range so we can
  // avoid the overhead of full UTF-8 processing.
  path = match path.bytes().rposition(|b| b != SEP) {
    Some(pos) => &path[..pos + 1],
    _ => &path[..0]
  };
  if let Some(pos) = path.bytes().rposition(|b| b == SEP) {
    path = &path[pos + 1..];
  }
  path
}

pub fn extract_last_path_segment_pointer(path: &str) -> &str {
  // Works with bytes directly because MAIN_SEPARATOR is always in the ASCII 7-bit range so we can
  // avoid the overhead of full UTF-8 processing.
  let ptr = path.as_ptr();
  let mut i = path.len() as isize - 1;
  while i >= 0 {
    let c = unsafe { *ptr.offset(i) };
    if c != SEP { break; };
    i -= 1;
  }
  let end = (i + 1) as usize;
  while i >= 0 {
    let c = unsafe { *ptr.offset(i) };
    if c == SEP {
      return &path[(i + 1) as usize..end];
    };
    i -= 1;
  }
  &path[..end]
}


#[cfg(test)]
mod tests {
  use super::*;
  use test::Bencher;
  use test::black_box;

  static PATH_FOR_BENCH: &'static str = "/hellooooooooo/woλλλλrλd/";

  #[bench]
  fn bench_extract_last_path_segment_bytes(b: &mut Bencher) {
    let path = black_box(PATH_FOR_BENCH);
    b.iter(|| extract_last_path_segment_bytes(path));
  }

  #[bench]
  fn bench_extract_last_path_segment_str(b: &mut Bencher) {
    let path = black_box(PATH_FOR_BENCH);
    b.iter(|| extract_last_path_segment_str(path));
  }

  #[bench]
  fn bench_extract_last_path_segment_pointer(b: &mut Bencher) {
    let path = black_box(PATH_FOR_BENCH);
    b.iter(|| extract_last_path_segment_pointer(path));
  }
}
