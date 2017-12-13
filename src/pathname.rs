use helpers::*;
use basename;
use chop_basename;
use dirname;
use extname;
use plus;

use ruru;
use ruru::{RString, Boolean, Array};
use std::path::{MAIN_SEPARATOR,Path};
use std::fs;

type MaybeString = Result<ruru::RString, ruru::result::Error>;
type MaybeBoolean = Result<ruru::Boolean, ruru::result::Error>;

pub fn pn_add_trailing_separator(pth: MaybeString) -> RString {
  let p = pth.ok().unwrap();
  let x = format!("{}{}", p.to_str(), "a");
  match x.rsplit_terminator(MAIN_SEPARATOR).next() {
    Some("a") => p,
    _ => RString::new(format!("{}{}", p.to_str(), MAIN_SEPARATOR).as_str())
  }
}

pub fn pn_is_absolute(pth: MaybeString) -> Boolean {
  Boolean::new(match pth.ok().unwrap_or(RString::new("")).to_str().chars().next() {
    Some(c) => c == MAIN_SEPARATOR,
    None => false
  })
}

// pub fn pn_ascend(){}

pub fn pn_basename(pth: MaybeString, ext: MaybeString) -> RString {
  RString::new(
    basename::basename(
      pth.ok().unwrap_or(RString::new("")).to_str(),
      ext.ok().unwrap_or(RString::new("")).to_str()
    )
  )
}

pub fn pn_children(pth: MaybeString, with_dir: MaybeBoolean) -> Array {
  let rstring = pth.ok().unwrap_or(RString::new("."));
  let val = rstring.to_str();
  let mut with_directory = with_dir.ok().unwrap_or(Boolean::new(true)).to_bool();
  if val == "." {
    with_directory = false;
  }
  let mut arr = Array::new();

  if let Ok(entries) = fs::read_dir(val) {
    for entry in entries {
      if with_directory {
        match entry {
          Ok(v) => { arr.push(RString::new(v.path().to_str().unwrap())); },
          _ => {}
        };
      } else {
        match entry {
          Ok(v) => { arr.push(RString::new(v.file_name().to_str().unwrap())); },
          _ => {}
        };
      }
    }
  }
  arr
}

pub fn pn_children_compat(pth: MaybeString, with_dir: MaybeBoolean) -> Array {
  let rstring = pth.ok().unwrap_or(RString::new("."));
  let val = rstring.to_str();
  let mut with_directory = with_dir.ok().unwrap_or(Boolean::new(true)).to_bool();
  if val == "." {
    with_directory = false;
  }
  let mut arr = Array::new();

  if let Ok(entries) = fs::read_dir(val) {
    for entry in entries {
      if with_directory {
        match entry {
          Ok(v) => { arr.push(
              class_new("Pathname", vec![str_to_any_obj(v.path().to_str().unwrap())])
            );
          },
          _ => {}
        };
      } else {
        match entry {
          Ok(v) => { arr.push(
              class_new("Pathname", vec![str_to_any_obj(v.file_name().to_str().unwrap())])
            );
          },
          _ => {}
        };
      }
    }
  }
  arr
}

pub fn pn_chop_basename(pth: MaybeString) -> Array {
  let mut arr = Array::with_capacity(2);
  let results = chop_basename::chop_basename(pth.ok().unwrap_or(RString::new("")).to_str());
  match results {
    Some((dirname, basename)) => {
      arr.push(RString::new(&dirname[..]));
      arr.push(RString::new(&basename[..]));
      arr
    },
    None => arr
  }
}

// pub fn pn_cleanpath(pth: MaybeString){}

// pub fn pn_cleanpath_aggressive(pth: MaybeString){}

// pub fn pn_cleanpath_conservative(pth: MaybeString){}

// pub fn pn_del_trailing_separator(pth: MaybeString){}

// pub fn pn_descend(){}

pub fn pn_is_directory(pth: MaybeString) -> Boolean {
  Boolean::new(
    Path::new(
      pth.ok().unwrap_or(RString::new("")).to_str()
    ).is_dir()
  )
}

pub fn pn_dirname(pth: MaybeString) -> RString {
  RString::new(
    &dirname::dirname(
      pth.ok().unwrap_or(RString::new("")).to_str()
    )[..]
  )
}

// pub fn pn_each_child(){}

// pub fn pn_each_filename(pth: MaybeString) -> NilClass {
//   NilClass::new()
// }

pub fn pn_entries(pth: MaybeString) -> Array {
  let files = fs::read_dir(pth.ok().unwrap_or(RString::new("")).to_str()).unwrap();
  let mut arr = Array::new();

  arr.push(RString::new("."));
  arr.push(RString::new(".."));

  for file in files {
    let file_name_str = file.unwrap().file_name().into_string().unwrap();
    arr.push(RString::new(&file_name_str[..]));
  }

  arr
}

pub fn pn_entries_compat(pth: MaybeString) -> Array {
  let files = fs::read_dir(pth.ok().unwrap_or(RString::new("")).to_str()).unwrap();
  let mut arr = Array::new();

  arr.push(class_new("Pathname", vec![str_to_any_obj(&"."[..])]));
  arr.push(class_new("Pathname", vec![str_to_any_obj(&".."[..])]));

  for file in files {
    let file_name_str = file.unwrap().file_name().into_string().unwrap();
    arr.push(class_new("Pathname", vec![str_to_any_obj(&file_name_str[..])]));
  }

  arr
}

pub fn pn_extname(pth: MaybeString) -> RString {
  RString::new(
    extname::extname(pth.ok().unwrap_or(RString::new("")).to_str())
  )
}

// pub fn pn_find(pth: MaybeString ,ignore_error: Boolean){}

pub fn pn_has_trailing_separator(pth: MaybeString) -> Boolean {
  let v = pth.ok().unwrap_or(RString::new(""));
  match chop_basename::chop_basename(v.to_str()) {
    Some((a,b)) => {
      Boolean::new(a.len() + b.len() < v.to_str().len())
    },
    _ => Boolean::new(false)
  }
}

// pub fn pn_join(args: Array){}

// pub fn pn_mkpath(pth: MaybeString) -> NilClass {
//   NilClass::new()
// }

// pub fn pn_is_mountpoint(pth: MaybeString){}

// pub fn pn_parent(pth: MaybeString){}

// also need impl +
pub fn pn_plus(pth1: MaybeString, pth2: MaybeString) -> RString {
  RString::new(&plus::plus_paths(pth1.ok().unwrap().to_str(), pth2.ok().unwrap().to_str())[..])
}

// pub fn pn_prepend_prefix(prefix: MaybeString, relpath: MaybeString){}

pub fn pn_is_relative(pth: MaybeString) -> Boolean {
  Boolean::new(
    match pth.ok().unwrap_or(RString::new(&MAIN_SEPARATOR.to_string()[..])).to_str().chars().next() {
      Some(c) => c != MAIN_SEPARATOR,
      None => true
    }
  )
}

// pub fn pn_root(pth: MaybeString){}

// pub fn pn_split_names(pth: MaybeString){}

// pub fn pn_relative_path_from(){}

// pub fn pn_rmtree(pth: MaybeString) -> NilClass {
//   NilClass::new()
// }
