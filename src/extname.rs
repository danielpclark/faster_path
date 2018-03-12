use std::str;
use path_parsing::{SEP, find_last_non_sep_pos};

pub fn extname(path: &str) -> &str {
  let end = match find_last_non_sep_pos(path.as_bytes()) {
    Some(pos) => pos + 1,
    _ => return "",
  };
  let bytes = &path.as_bytes()[..end];
  for (pos, c) in bytes.iter().enumerate().rev() {
    match *c {
      b'.' => {
        let prev = bytes.get(pos - 1);
        if pos == end - 1 || prev == None || prev == Some(&SEP) {
          return "";
        } else {
          return &path[pos..end]
        };
      }
      SEP => return "",
      _ => {}
    }
  };
  ""
}
