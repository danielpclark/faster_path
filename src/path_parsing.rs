use std::path::MAIN_SEPARATOR;

static SEP: u8 = MAIN_SEPARATOR as u8;

pub fn extract_last_path_segment(path: &str) -> &str {
  // Works with bytes directly because MAIN_SEPARATOR is always in the ASCII 7-bit range so we can
  // avoid the overhead of full UTF-8 processing.
  // See src/benches/path_parsing.rs for benchmarks of different approaches.
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


// Returns the byte offset of the last byte preceding a MAIN_SEPARATOR.
pub fn last_non_sep_i(path: &str) -> isize {
  last_non_sep_i_before(path, path.len() as isize - 1)
}

// Returns the byte offset of the last byte preceding a MAIN_SEPARATOR before the given end offset.
pub fn last_non_sep_i_before(path: &str, end: isize) -> isize {
  let ptr = path.as_ptr();
  let mut i = end;
  while i >= 0 {
    if unsafe { *ptr.offset(i) } != SEP { break; };
    i -= 1;
  }
  i
}

// Returns the byte offset of the last MAIN_SEPARATOR before the given end offset.
pub fn last_sep_i(path: &str, end: isize) -> isize {
  let ptr = path.as_ptr();
  let mut i = end - 1;
  while i >= 0 {
    if unsafe { *ptr.offset(i) } == SEP {
      return i;
    };
    i -= 1;
  }
  -1
}
