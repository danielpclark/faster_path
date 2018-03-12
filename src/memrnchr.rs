// The code below is based on the fallback `memrchr` implementation from the respective crate.
//
// We use this mainly to skip repeated `/`. If there is only one slash, `memrnchr` performs the same
// as a naive version (e.g. `rposition`). However, it is much faster in pathological cases.

#[cfg(target_pointer_width = "32")]
const USIZE_BYTES: usize = 4;
#[cfg(target_pointer_width = "64")]
const USIZE_BYTES: usize = 8;

// Returns the byte offset of the last byte that is NOT equal to the given one.
#[inline(always)]
pub fn memrnchr(x: u8, text: &[u8]) -> Option<usize> {
  // Scan for a single byte value by reading two `usize` words at a time.
  //
  // Split `text` in three parts
  // - unaligned tail, after the last word aligned address in text
  // - body, scan by 2 words at a time
  // - the first remaining bytes, < 2 word size
  let len = text.len();
  let ptr = text.as_ptr();

  // search to an aligned boundary
  let end_align = (ptr as usize + len) & (USIZE_BYTES - 1);
  let mut offset;
  if end_align > 0 {
    offset = if end_align >= len { 0 } else { len - end_align };
    let pos = text[offset..].iter().rposition(|elt| *elt != x);
    if let Some(index) = pos {
      return Some(offset + index);
    }
  } else {
    offset = len;
  }

  // search the body of the text
  let repeated_x = repeat_byte(x);

  while offset >= 2 * USIZE_BYTES {
    debug_assert_eq!((ptr as usize + offset) % USIZE_BYTES, 0);
    unsafe {
      let u = *(ptr.offset(offset as isize - 2 * USIZE_BYTES as isize) as *const usize);
      let v = *(ptr.offset(offset as isize - USIZE_BYTES as isize) as *const usize);

      // break if there is no matching byte
      if u & repeated_x != usize::max_value() || v & repeated_x != usize::max_value() {
        break;
      }
    }
    offset -= 2 * USIZE_BYTES;
  }

  // find the byte before the point the body loop stopped
  text[..offset].iter().rposition(|elt| *elt != x)
}

#[cfg(target_pointer_width = "32")]
#[inline]
fn repeat_byte(b: u8) -> usize {
  let mut rep = (b as usize) << 8 | b as usize;
  rep = rep << 16 | rep;
  rep
}

#[cfg(target_pointer_width = "64")]
#[inline]
fn repeat_byte(b: u8) -> usize {
  let mut rep = (b as usize) << 8 | b as usize;
  rep = rep << 16 | rep;
  rep = rep << 32 | rep;
  rep
}
