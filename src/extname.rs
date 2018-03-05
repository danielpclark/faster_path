use path_parsing::SEP;
use std::str;

struct ExtnameCoords {
  word: bool,
  pred: bool,
  dot: bool,
  start: usize,
  end: usize,
}

impl ExtnameCoords {
  pub fn dec(&mut self) {
    self.start -= 1;
    if !self.word {
      self.end -= 1;
    }
  }
}

pub fn extname(pth: &str) -> &str {
  let path = pth.as_bytes();
  let mut extname = ExtnameCoords {
    word: false,
    pred: false,
    dot: false,
    start: path.len(),
    end: path.len(),
  };

  for &item in path.iter().rev() {
    if (item == b'.' && !extname.dot) || item == SEP {
      if item == SEP && extname.word {
        return ""
      }

      if !extname.pred {
        extname.dec();
      }

      if extname.word {
        extname.dot = true;
      }
    } else {
      if extname.dot {
        extname.pred = true;
        break;
      } else {
        extname.word = true;
      }

      if !extname.pred {
        extname.dec()
      }
    }
  }

  if !extname.dot || !extname.pred {
    return "";
  }

  str::from_utf8(&path[extname.start..extname.end]).unwrap_or("")
}
