use helpers::*;
use basename;
use chop_basename;
use cleanpath_aggressive;
use dirname;
use extname;
use plus;

use ruru;
use ruru::{RString, Boolean, Array, AnyObject, NilClass, Object};
use std::path::{MAIN_SEPARATOR,Path};
use std::fs;

type MaybeAnyObject = Result<ruru::AnyObject, ruru::result::Error>;
type MaybeArray = Result<ruru::Array, ruru::result::Error>;
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

pub fn pn_children(pth: MaybeString, with_dir: MaybeBoolean) -> AnyObject {
  let val = pth.ok().unwrap_or(RString::new("."));
  let val = val.to_str();

  if let Ok(entries) = fs::read_dir(val) {
    let mut with_directory = with_dir.ok().unwrap_or(Boolean::new(true)).to_bool();
    if val == "." {
      with_directory = false;
    }

    let mut arr = Array::new();
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

    arr.to_any_object()
  } else {
    // TODO: When ruru exceptions are available switch the exception logic
    // from the Ruby side to the Rust side
    NilClass::new().to_any_object()
  }
}

pub fn pn_children_compat(pth: MaybeString, with_dir: MaybeBoolean) -> AnyObject {
  let val = pth.ok().unwrap_or(RString::new("."));
  let val = val.to_str();

  if let Ok(entries) = fs::read_dir(val) {
    let mut with_directory = with_dir.ok().unwrap_or(Boolean::new(true)).to_bool();
    if val == "." {
      with_directory = false;
    }

    let mut arr = Array::new();
    for entry in entries {
      if with_directory {
        if let Ok(v) = entry {
          arr.push(new_pathname_instance(v.path().to_str().unwrap()));
        };
      } else {
        if let Ok(v) = entry {
          arr.push(new_pathname_instance(v.file_name().to_str().unwrap()));
        };
      }
    }

    arr.to_any_object()
  } else {
    // TODO: When ruru exceptions are available switch the exception logic
    // from the Ruby side to the Rust side
    NilClass::new().to_any_object()
  }
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

pub fn pn_cleanpath_aggressive(pth: MaybeString) -> RString {
  let path = cleanpath_aggressive::cleanpath_aggressive(
    pth.ok().unwrap_or(RString::new("")).to_str()
  );

  RString::new(&path)
}

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
    dirname::dirname(
      pth.ok().unwrap_or(RString::new("")).to_str()
    )
  )
}

// pub fn pn_each_child(){}

// pub fn pn_each_filename(pth: MaybeString) -> NilClass {
//   NilClass::new()
// }

pub fn pn_entries(pth: MaybeString) -> AnyObject {
  if let Ok(files) = fs::read_dir(pth.ok().unwrap_or(RString::new("")).to_str()) {
    let mut arr = Array::new();

    arr.push(RString::new("."));
    arr.push(RString::new(".."));

    for file in files {
      let file_name_str = file.unwrap().file_name().into_string().unwrap();
      arr.push(RString::new(&file_name_str[..]));
    }

    arr.to_any_object()
  } else {
    // TODO: When ruru exceptions are available switch the exception logic
    // from the Ruby side to the Rust side
    NilClass::new().to_any_object()
  }
}

pub fn pn_entries_compat(pth: MaybeString) -> AnyObject {
  if let Ok(files) = fs::read_dir(pth.ok().unwrap_or(RString::new("")).to_str()) {
    let mut arr = Array::new();

    arr.push(new_pathname_instance("."));
    arr.push(new_pathname_instance(".."));

    for file in files {
      let file_name_str = file.unwrap().file_name().into_string().unwrap();
      arr.push(new_pathname_instance(&file_name_str));
    }

    arr.to_any_object()
  } else {
    // TODO: When ruru exceptions are available switch the exception logic
    // from the Ruby side to the Rust side
    NilClass::new().to_any_object()
  }
}

pub fn pn_extname(pth: MaybeString) -> RString {
  RString::new(
    extname::extname(pth.ok().unwrap_or(RString::new("")).to_str())
  )
}

// pub fn pn_find(pth: MaybeString, ignore_error: Boolean){}

pub fn pn_has_trailing_separator(pth: MaybeString) -> Boolean {
  let v = pth.ok().unwrap_or(RString::new(""));
  match chop_basename::chop_basename(v.to_str()) {
    Some((a,b)) => {
      Boolean::new(a.len() + b.len() < v.to_str().len())
    },
    _ => Boolean::new(false)
  }
}

pub fn pn_join(path_self: MaybeAnyObject, args: MaybeArray) -> AnyObject {
  let args = args.unwrap(); //_or(Array::new());
  let path_self = path_self.unwrap_or(RString::new("").to_any_object());
  let length = args.length();
  if length == 0 {
    return into_pathname(path_self).unwrap();
  }

  let mut result = String::new();
  for idx in 0..(length as usize) {
    let index = length - idx;
    let item = args.at(index as i64);
    result = plus::plus_paths(&anyobject_to_string(item), &result);
    if result.chars().next() == Some(MAIN_SEPARATOR) {
      return new_pathname_instance(&result).to_any_object()
    }
  }

  let build = into_pathname(path_self).unwrap();
  let path = build.
    instance_variable_get("@path").
    try_convert_to::<RString>().
    unwrap_or(RString::new("")).
    to_string();

  new_pathname_instance(&plus::plus_paths(&path, &result)).to_any_object()
}

// pub fn pn_mkpath(pth: MaybeString) -> NilClass {
//   NilClass::new()
// }

// pub fn pn_is_mountpoint(pth: MaybeString){}

// pub fn pn_parent(pth: MaybeString){}

pub fn pn_plus(pth1: MaybeString, pth2: MaybeString) -> RString {
  RString::new(
    &plus::plus_paths(
      pth1.ok().unwrap_or(RString::new("")).to_str(),
      pth2.ok().unwrap_or(RString::new("")).to_str()
    )[..]
  )
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
//
