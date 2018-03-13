use path_parsing::{find_last_non_sep_pos, find_last_word};
use extname::extname;
use memrnchr::memrnchr;

pub fn basename<'a>(path: &'a str, ext: &str) -> &'a str {
  if path.is_empty() {
    return path;
  }

  let bytes: &[u8] = path.as_bytes();

  let mut end = if let Some(v) = find_last_non_sep_pos(bytes) {
    v + 1
  } else {
    return "/"
  };

  if ext.is_empty() || ext.len() > path.len() {
    return &path[find_last_word(bytes)];
  }

  let extension = ext.as_bytes();

  if extension == b".*" {
    let e = extname(&path[..end]);

    end -= if e.is_empty() {
      memrnchr(b'.', &bytes[..end]).map(|v| bytes[..end].len() - v - 1).unwrap_or(0)
    } else {
      e.len()
    };
    return &path[find_last_word(&bytes[..end])];
  }

  let mut y = extension.iter().rev();
  let mut z = bytes[..end].iter().rev();
  let mut pos = 0;
  let mut start = end;
  loop {
    match (y.next(), z.next()) {
      (_, None) | (None, Some(_)) => { 
        start = find_last_word(&bytes[..end]).start;
        break
      },
      (Some(c), Some(d)) if c != d => {
        start = find_last_word(&bytes[..end]).start;
        break
      },
      _ => {
        end -= 1;

        if pos > bytes.len() {
          break
        }
      }
    }

    pos += 1;
  }

  &path[start..end]
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
