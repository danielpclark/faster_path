use std::path::MAIN_SEPARATOR;
use libc::c_char;
use std::ffi::{CStr,CString};
use std::str;

#[no_mangle]
pub extern fn plus(string: *const c_char, string2: *const c_char) -> *const c_char {
  let c_str = unsafe {
    if string.is_null() {
      return string;
    }
    CStr::from_ptr(string)
  };

  let c_str2 = unsafe {
    if string2.is_null() {
      return string2;
    }
    CStr::from_ptr(string2)
  };

  let r_str = str::from_utf8(c_str.to_bytes()).unwrap();
  let r_str2 = str::from_utf8(c_str2.to_bytes()).unwrap();

  let mut prefix2 = r_str2.clone();
  let basename2: &str;
  let mut index2_list2: Vec<&str> = vec![];
  let mut basename_list2: Vec<&str> = vec![];
  loop {
    // chop basename
    (prefix2, basename2) = prefix2.split_at(prefix2.rfind('/').unwrap());
    index_list2.insert(0, prefix2.length());
    basename_list2.insert(0, basename2);
  }
  if prefix2.is_empty() { return string2 };
  let mut prefix1 = r_str.clone();
  loop {
    while !basename_list2.is_empty() && basename_list2[0] == "." {
      index_list2.remove(0);
      basename_list2.remove(0);
    }
    match chop_basename(prefix1) {
      None => { break },
      Some(val) => {
        (prefix1, basename1) = val
      },
    }
    if basename1 == "." { continue };
    if basename1 == ".." || basename_list2.is_empty() || basename_list2[0] != ".." {
      prefix1 = prefix1 + basename1;
      break
    }
    index_list2.remove(0);
    basename_list2.remove(0);
  }
  if let Some(r1) = chop_basename(prefix1);
  if !r1 && (r1 = /#{SEPARATOR_PAT}/o =~ File.basename(prefix1)) {
    while !basename_list2.is_empty() && basename_list2[0] == ".." {
      index_list2.remove(0);
      basename_list2.remove(0);
    }
  }
  let result = if !basename_list2.is_empty() {
    suffix2 = path2[index_list2.first..-1];
    if r1 { File.join(prefix1, suffix2) } else { prefix1 + suffix2 }
  } else {
    if r1 { prefix1 } else { File.dirname(prefix1) }
  }

  let output = CString::new(result).unwrap();
  output.into_raw()
}
