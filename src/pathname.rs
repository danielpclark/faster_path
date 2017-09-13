use basename;
use chop_basename;
use dirname;
use extname;
use plus;

use ruru;
use ruru::{RString, Boolean, Array};
use std::path::{MAIN_SEPARATOR,Path};
use std::fs;

// TOPATH = :to_path

// SAME_PATHS = if File::FNM_SYSCASE.nonzero?
//   # Avoid #zero? here because #casecmp can return nil.
//   proc {|a, b| a.casecmp(b) == 0}
// else
//   proc {|a, b| a == b}
// end

// if File::ALT_SEPARATOR
//   SEPARATOR_LIST = "#{Regexp.quote File::ALT_SEPARATOR}#{Regexp.quote File::SEPARATOR}"
//   SEPARATOR_PAT = /[#{SEPARATOR_LIST}]/
// else
//   SEPARATOR_LIST = "#{Regexp.quote File::SEPARATOR}"
//   SEPARATOR_PAT = /#{Regexp.quote File::SEPARATOR}/
// end

pub fn pn_add_trailing_separator(pth: Result<ruru::RString, ruru::result::Error>) -> RString {
  let p = pth.ok().unwrap();
  let x = format!("{}{}", p.to_str(), "a");
  match x.rsplit_terminator(MAIN_SEPARATOR).next() {
    Some("a") => p,
    _ => RString::new(format!("{}{}", p.to_str(), MAIN_SEPARATOR).as_str())
  }
}

pub fn pn_is_absolute(pth: Result<ruru::RString, ruru::result::Error>) -> Boolean {
  Boolean::new(match pth.ok().unwrap_or(RString::new("")).to_str().chars().next() {
    Some(c) => c == MAIN_SEPARATOR,
    None => false
  })
}

// pub fn pn_ascend(){}

pub fn pn_basename(pth: Result<ruru::RString, ruru::result::Error>, ext: Result<ruru::RString, ruru::result::Error>) -> RString {
  RString::new(
    &basename::basename(
      pth.ok().unwrap_or(RString::new("")).to_str(),
      ext.ok().unwrap_or(RString::new("")).to_str()
    )[..]
  )
}

// pub fn pn_children(pth: Result<ruru::RString, ruru::result::Error>, with_dir: Boolean){}

pub fn pn_chop_basename(pth: Result<ruru::RString, ruru::result::Error>) -> Array {
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

// pub fn pn_cleanpath(pth: Result<ruru::RString, ruru::result::Error>){}

// pub fn pn_cleanpath_aggressive(pth: Result<ruru::RString, ruru::result::Error>){}

// pub fn pn_cleanpath_conservative(pth: Result<ruru::RString, ruru::result::Error>){}

// pub fn pn_del_trailing_separator(pth: Result<ruru::RString, ruru::result::Error>){}

// pub fn pn_descend(){}

pub fn pn_is_directory(pth: Result<ruru::RString, ruru::result::Error>) -> Boolean {
  Boolean::new(
    Path::new(
      pth.ok().unwrap_or(RString::new("")).to_str()
    ).is_dir()
  )
}

pub fn pn_dirname(pth: Result<ruru::RString, ruru::result::Error>) -> RString {
  RString::new(
    &dirname::dirname(
      pth.ok().unwrap_or(RString::new("")).to_str()
    )[..]
  )
}

// pub fn pn_each_child(){}

// pub fn pn_each_filename(pth: Result<ruru::RString, ruru::result::Error>) -> NilClass {
//   NilClass::new()
// }

pub fn pn_entries(pth: Result<ruru::RString, ruru::result::Error>) -> Array {
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

pub fn pn_extname(pth: Result<ruru::RString, ruru::result::Error>) -> RString {
  RString::new(
    &extname::extname(pth.ok().unwrap_or(RString::new("")).to_str())[..]
  )
}

// pub fn pn_find(pth: Result<ruru::RString, ruru::result::Error> ,ignore_error: Boolean){}

pub fn pn_has_trailing_separator(pth: Result<ruru::RString, ruru::result::Error>) -> Boolean {
  let v = pth.ok().unwrap_or(RString::new(""));
  match chop_basename::chop_basename(v.to_str()) {
    Some((a,b)) => {
      Boolean::new(a.len() + b.len() < v.to_str().len())
    },
    _ => Boolean::new(false)
  }
}

// pub fn pn_join(args: Array){}

// pub fn pn_mkpath(pth: Result<ruru::RString, ruru::result::Error>) -> NilClass {
//   NilClass::new()
// }

// pub fn pn_is_mountpoint(pth: Result<ruru::RString, ruru::result::Error>){}

// pub fn pn_parent(pth: Result<ruru::RString, ruru::result::Error>){}

// also need impl +
pub fn pn_plus(pth1: Result<ruru::RString, ruru::result::Error>, pth2: Result<ruru::RString, ruru::result::Error>) -> RString {
  RString::new(&plus::plus_paths(pth1.ok().unwrap().to_str(), pth2.ok().unwrap().to_str())[..])
}

// pub fn pn_prepend_prefix(prefix: Result<ruru::RString, ruru::result::Error>, relpath: Result<ruru::RString, ruru::result::Error>){}

pub fn pn_is_relative(pth: Result<ruru::RString, ruru::result::Error>) -> Boolean {
  Boolean::new(
    match pth.ok().unwrap_or(RString::new(&MAIN_SEPARATOR.to_string()[..])).to_str().chars().next() {
      Some(c) => c != MAIN_SEPARATOR,
      None => true
    }
  )
}

// pub fn pn_root(pth: Result<ruru::RString, ruru::result::Error>){}

// pub fn pn_split_names(pth: Result<ruru::RString, ruru::result::Error>){}

// pub fn pn_relative_path_from(){}

// pub fn pn_rmtree(pth: Result<ruru::RString, ruru::result::Error>) -> NilClass {
//   NilClass::new()
// }

