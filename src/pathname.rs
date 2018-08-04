use helpers::*;
use basename;
use chop_basename;
use cleanpath_aggressive;
use cleanpath_conservative;
use dirname;
use extname;
use plus;
use relative_path_from;
use debug;
use helpers::{TryFrom, to_str};
use path_parsing::{SEP, find_last_non_sep_pos};

use rutie;
use rutie::{
  RString,
  Boolean,
  Array,
  AnyObject,
  NilClass,
  Object,
  Class,
  VerifiedObject,
  Exception as Exc,
  AnyException as Exception,
};
use rutie::types::{Value, ValueType};
use std::borrow::Cow;
use std::path::{MAIN_SEPARATOR, Path};
use std::fs;

type MaybeArray = Result<rutie::Array, rutie::AnyException>;
type MaybeString = Result<rutie::RString, rutie::AnyException>;
type MaybeBoolean = Result<rutie::Boolean, rutie::AnyException>;

pub struct Pathname {
  value: Value
}

impl Pathname {
  pub fn new(path: &str) -> Pathname {
    let arguments = [RString::new_usascii_unchecked(path).to_any_object()];
    let instance = Class::from_existing("Pathname").new_instance(Some(&arguments));

    Pathname { value: instance.value() }
  }

  pub fn to_any_object(&self) -> AnyObject {
    AnyObject::from(self.value())
  }
}

impl From<Value> for Pathname {
  fn from(value: Value) -> Self {
    Pathname { value }
  }
}

impl TryFrom<AnyObject> for Pathname {
  type Error = debug::RubyDebugInfo;
  fn try_from(obj: AnyObject) -> Result<Pathname, Self::Error> {
    if Class::from_existing("String").case_equals(&obj) {
      Ok(Pathname::new(&RString::from(obj.value()).to_str()))
    } else if Class::from_existing("Pathname").case_equals(&obj) {
      Ok(Pathname::from(obj.value()))
    } else if obj.respond_to("to_path") {
      Ok(Pathname::new(&RString::from(obj.send("to_path", None).value()).to_str()))
    } else {
      Err(Self::Error::from(obj))
    }
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
  let p = pth.unwrap();
  let x = format!("{}{}", p.to_str(), "a");
  match x.rsplit_terminator(MAIN_SEPARATOR).next() {
    Some("a") => p,
    _ => RString::new_usascii_unchecked(format!("{}{}", p.to_str(), MAIN_SEPARATOR).as_str())
  }
}

pub fn pn_is_absolute(pth: MaybeString) -> Boolean {
  Boolean::new(to_str(&pth).as_bytes().get(0) == Some(&SEP))
}

// pub fn pn_ascend(){}

pub fn pn_basename(pth: MaybeString, ext: MaybeString) -> RString {
  RString::new_usascii_unchecked(basename::basename(to_str(&pth), to_str(&ext)))
}

pub fn pn_children(pth: MaybeString, with_dir: MaybeBoolean) -> Result<AnyObject, Exception> {
  let path = pth.unwrap_or(RString::new_usascii_unchecked("."));
  let path = path.to_str();

  if let Ok(entries) = fs::read_dir(path) {
    let mut with_directory = with_dir.unwrap_or(Boolean::new(true)).to_bool();
    if path == "." {
      with_directory = false;
    }

    let mut arr = Array::with_capacity(entries.size_hint().1.unwrap_or(0));
    for entry in entries {
      if with_directory {
        match entry {
          Ok(v) => { arr.push(RString::new_usascii_unchecked(v.path().to_str().unwrap())); },
          _ => {}
        };
      } else {
        match entry {
          Ok(v) => { arr.push(RString::new_usascii_unchecked(v.file_name().to_str().unwrap())); },
          _ => {}
        };
      }
    }

    Ok(arr.to_any_object())
  } else {
    let msg = format!("No such file or directory @ dir_initialize - {}", path);
    Err(Exception::new("Errno::NOENT", Some(&msg)))
  }
}

pub fn pn_children_compat(pth: MaybeString, with_dir: MaybeBoolean) -> Result<AnyObject, Exception> {
  let path = to_str(&pth);

  if let Ok(entries) = fs::read_dir(path) {
    let mut with_directory = with_dir.unwrap_or(Boolean::new(true)).to_bool();
    if path == "." {
      with_directory = false;
    }

    let mut arr = Array::with_capacity(entries.size_hint().1.unwrap_or(0));
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

    Ok(arr.to_any_object())
  } else {
    let msg = format!("No such file or directory @ dir_initialize - {}", path);
    Err(Exception::new("Errno::NOENT", Some(&msg)))
  }
}

pub fn pn_chop_basename(pth: MaybeString) -> AnyObject {
  match chop_basename::chop_basename(to_str(&pth)) {
    Some((dirname, basename)) => {
      let mut arr = Array::with_capacity(2);
      arr.push(RString::new_usascii_unchecked(&dirname));
      arr.push(RString::new_usascii_unchecked(&basename));
      arr.to_any_object()
    },
    None => NilClass::new().to_any_object()
  }
}

// pub fn pn_cleanpath(pth: MaybeString){}

pub fn pn_cleanpath_aggressive(pth: MaybeString) -> RString {
  RString::new_usascii_unchecked(&cleanpath_aggressive::cleanpath_aggressive(to_str(&pth)))
}

pub fn pn_cleanpath_conservative(pth: MaybeString) -> RString {
  RString::new_usascii_unchecked(&cleanpath_conservative::cleanpath_conservative(to_str(&pth)))
}

pub fn pn_del_trailing_separator(pth: MaybeString) -> RString {
  {
    let path = to_str(&pth);
    if path.is_empty() {
      return RString::new_usascii_unchecked("/");
    }
    let pos = match find_last_non_sep_pos(path.as_bytes()) {
      Some(pos) => pos,
      None => return RString::new_usascii_unchecked("/"),
    };
    if pos != path.len() - 1 {
      return RString::new_usascii_unchecked(&path[..pos + 1]);
    }
  }
  pth.unwrap()
}

// pub fn pn_descend(){}

pub fn pn_is_directory(pth: MaybeString) -> Boolean {
  Boolean::new(Path::new(to_str(&pth)).is_dir())
}

pub fn pn_dirname(pth: MaybeString) -> RString {
  RString::new_usascii_unchecked(dirname::dirname(to_str(&pth)))
}

// pub fn pn_each_child(){}

// pub fn pn_each_filename(pth: MaybeString) -> NilClass {
//   NilClass::new()
// }

pub fn pn_entries(pth: MaybeString) -> Result<AnyObject, Exception> {
  let path = to_str(&pth);
  if let Ok(files) = fs::read_dir(path) {
    let mut arr = Array::with_capacity(files.size_hint().1.unwrap_or(0) + 2);

    arr.push(RString::new_usascii_unchecked("."));
    arr.push(RString::new_usascii_unchecked(".."));

    for file in files {
      arr.push(RString::new_usascii_unchecked(file.unwrap().file_name().to_str().unwrap()));
    }

    Ok(arr.to_any_object())
  } else {
    let msg = format!("No such file or directory @ dir_initialize - {}", path);
    Err(Exception::new("Errno::NOENT", Some(&msg)))
  }
}

pub fn pn_entries_compat(pth: MaybeString) -> Result<AnyObject, Exception> {
  let path = to_str(&pth);
  if let Ok(files) = fs::read_dir(path) {
    let mut arr = Array::with_capacity(files.size_hint().1.unwrap_or(0) + 2);

    arr.push(Pathname::new("."));
    arr.push(Pathname::new(".."));

    for file in files {
      arr.push(Pathname::new(file.unwrap().file_name().to_str().unwrap()));
    }

    Ok(arr.to_any_object())
  } else {
    let msg = format!("No such file or directory @ dir_initialize - {}", path);
    Err(Exception::new("Errno::NOENT", Some(&msg)))
  }
}

pub fn pn_extname(pth: MaybeString) -> RString {
  RString::new_usascii_unchecked(extname::extname(to_str(&pth)))
}

// pub fn pn_find(pth: MaybeString, ignore_error: Boolean){}

pub fn pn_has_trailing_separator(pth: MaybeString) -> Boolean {
  let v = to_str(&pth);
  match chop_basename::chop_basename(v) {
    Some((a,b)) => Boolean::new(a.len() + b.len() < v.len()),
    _ => Boolean::new(false)
  }
}

pub fn pn_join(args: MaybeArray) -> AnyObject {
  let paths = args.unwrap().into_iter().map(|arg| anyobject_to_string(arg).unwrap()).collect::<Vec<_>>();
  let mut paths_iter = paths.iter().rev();
  let mut result = Cow::Borrowed(paths_iter.next().unwrap().as_str());
  for part in paths_iter {
    result = plus::plus_paths(&part, result.as_ref());
    if result.as_bytes().first() == Some(&SEP) {
      break;
    }
  }
  Pathname::new(&result).to_any_object()
}

// pub fn pn_mkpath(pth: MaybeString) -> NilClass {
//   NilClass::new()
// }

// pub fn pn_is_mountpoint(pth: MaybeString){}

// pub fn pn_parent(pth: MaybeString){}

pub fn pn_plus(pth1: MaybeString, pth2: MaybeString) -> RString {
  RString::new_usascii_unchecked(&plus::plus_paths(to_str(&pth1), to_str(&pth2)))
}

// pub fn pn_prepend_prefix(prefix: MaybeString, relpath: MaybeString){}

pub fn pn_is_relative(pth: MaybeString) -> Boolean {
  let path = match &pth {
    &Ok(ref rutie_string) => rutie_string.to_str(),
    &Err(_) => return Boolean::new(false),
  };
  Boolean::new(path.as_bytes().get(0) != Some(&SEP))
}

// pub fn pn_root(pth: MaybeString){}

// pub fn pn_split_names(pth: MaybeString){}

pub fn pn_relative_path_from(itself: MaybeString, base_directory: MaybeString) -> Result<Pathname, Exception> {
  relative_path_from::relative_path_from(itself, base_directory)
}

// pub fn pn_rmtree(pth: MaybeString) -> NilClass {
//   NilClass::new()
// }
//
