use helpers::*;
use basename;
use chop_basename;
use cleanpath_aggressive;
use cleanpath_conservative;
use dirname;
use extname;
use plus;
use path_parsing::SEP_STR;
extern crate array_tool;
use self::array_tool::vec::{Shift, Join};

use ruru;
use ruru::{
  RString,
  Boolean,
  Array,
  AnyObject,
  NilClass,
  Object,
  Class,
  VerifiedObject,
  Exception as Exc,
  AnyException as Exception
};
use ruru::types::{Value, ValueType};
use std::path::{MAIN_SEPARATOR,Path};
use std::fs;

type MaybeArray = Result<ruru::Array, ruru::result::Error>;
type MaybeString = Result<ruru::RString, ruru::result::Error>;
type MaybeBoolean = Result<ruru::Boolean, ruru::result::Error>;

pub struct Pathname {
  value: Value
}

impl Pathname {
  pub fn new(path: &str) -> Pathname {
    let mut instance = Class::from_existing("Pathname").allocate();
    instance.instance_variable_set("@path", RString::new(path).to_any_object());
    
    Pathname { value: instance.value() }
  }

  pub fn to_any_object(&self) -> AnyObject {
    AnyObject::from(self.value())
  }
}

impl From<Value> for Pathname {
  fn from(value: Value) -> Self {
    Pathname { value: value }
  }
}

impl Object for Pathname {
  #[inline]
  fn value(&self) -> Value {
    self.value
  }
}

impl VerifiedObject for Pathname {
  fn is_correct_type<T: Object>(object: &T) -> bool {
    object.value().ty() == ValueType::Class &&
      Class::from_existing("Pathname").case_equals(object)
  }

  fn error_message() -> &'static str {
    "Error converting to Pathname"
  }
}

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
          arr.push(Pathname::new(v.path().to_str().unwrap()));
        };
      } else {
        if let Ok(v) = entry {
          arr.push(Pathname::new(v.file_name().to_str().unwrap()));
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
  let pth = pth.ok().unwrap_or(RString::new(""));
  let results = chop_basename::chop_basename(pth.to_str());
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

pub fn pn_cleanpath_conservative(pth: MaybeString) -> RString {
  let path = cleanpath_conservative::cleanpath_conservative(
    pth.ok().unwrap_or(RString::new("")).to_str()
  );

  RString::new(&path)
}

pub fn pn_del_trailing_separator(pth: MaybeString) -> RString {
  if let &Ok(ref path) = &pth {
    let path = path.to_str();

    if !path.is_empty() {
      let path = path.trim_right_matches('/');

      if path.is_empty() {
        return RString::new("/");
      } else {
        return RString::new(path);
      }
    }
  } else {
    return RString::new("");
  }

  pth.unwrap()
}

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

    arr.push(Pathname::new("."));
    arr.push(Pathname::new(".."));

    for file in files {
      let file_name_str = file.unwrap().file_name().into_string().unwrap();
      arr.push(Pathname::new(&file_name_str));
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

pub fn pn_join(args: MaybeArray) -> AnyObject {
  let mut args = args.unwrap();
  let path_self = anyobject_to_string(args.shift()).unwrap();
  let mut qty = args.length();
  if qty <= 0 {
    return Pathname::new(&path_self).to_any_object();
  }

  let mut result = String::new();

  loop {
    if qty == 0 { break; }

    let item = args.pop();
    result = plus::plus_paths(&anyobject_to_string(item).unwrap(), &result);
    if result.chars().next() == Some(MAIN_SEPARATOR) {
      return Pathname::new(&result).to_any_object()
    }

    qty -= 1;
  }
  
  let result = plus::plus_paths(&path_self, &result);

  Pathname::new(&result).to_any_object()
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

pub fn pn_relative_path_from(itself: MaybeString, base_directory: MaybeString) -> Result<Pathname, Exception> {
  let dest_directory = pn_cleanpath_aggressive(itself).to_string();
  let base_directory = pn_cleanpath_aggressive(base_directory).to_string();

  let mut dest_prefix = dest_directory.clone();
  let mut dest_names: Vec<String> = vec![];
  loop {
    match chop_basename::chop_basename(&dest_prefix.clone()) {
      Some((ref dest, ref basename)) => {
        dest_prefix = dest.to_string();
        if basename != &"." {
          dest_names.unshift(basename.to_string())
        }
      },
      None => break,
    }
  }

  let mut base_prefix = base_directory.clone();
  let mut base_names: Vec<String> = vec![];
  loop {
    match chop_basename::chop_basename(&base_prefix.clone()) {
      Some((ref base, ref basename)) => {
        base_prefix = base.to_string();
        if basename != &"." {
          base_names.unshift(basename.to_string())
        }
      },
      None => break,
    }
  }

  if !is_same_path(&dest_prefix, &base_prefix) {
    return Err(
      Exception::new(
        "ArgumentError",
        Some(&format!("different prefix: {} and {}", dest_prefix, base_prefix)[..])
      )
    );
  }

  let (dest_names, base_names): (Vec<String>, Vec<String>) = dest_names.
    iter().
    zip(base_names.iter()).
    skip_while(|&(a, b)| is_same_path(a,b) ).
    fold((vec![], vec![]), |mut acc, (a, b)| {
      acc.0.push(a.to_string());
      acc.1.push(b.to_string());
      acc
    });

  if base_names.contains(&"..".to_string()) {
    return Err(
      Exception::new(
        "ArgumentError",
        Some(&format!("base_directory has ..: {}", base_directory)[..])
      )
    );
  }

  let base_names: Vec<String> = base_names.into_iter().map(|_| "..".to_string()).collect();

  if base_names.is_empty() && dest_names.is_empty() {
    Ok(Pathname::new("."))
  } else {
    Ok(Pathname::new(&base_names.iter().chain(dest_names.iter()).collect::<Vec<&String>>().join(&SEP_STR)))
  }
}

// pub fn pn_rmtree(pth: MaybeString) -> NilClass {
//   NilClass::new()
// }
//
