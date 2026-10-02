use std::borrow::Cow;
use std::ffi::OsStr;
use std::fs;
use std::io;

use rutie::{AnyException, AnyObject, Array, Boolean, Class, NilClass, Object, RString};

use crate::basename;
use crate::chop_basename::{self, is_relative};
use crate::cleanpath_aggressive;
use crate::cleanpath_conservative::{self, add_trailing_separator, del_trailing_separator, has_trailing_separator};
use crate::dirname;
use crate::extname;
use crate::path_parsing::Rules;
use crate::plus;
use crate::relative_path_from::{self, RelativePathError};
use crate::ruby::{
  argument_error, check_max_arguments, error, new_pathname, path_like_to_string, pathname_class,
  truthy_argument, EncodingId, EncodingOf, PathArgument, PathString, RubyResult,
};

// Ruby's path rules on this platform
const RULES: Rules = Rules::NATIVE;

pub fn pn_add_trailing_separator(arguments: &[AnyObject]) -> RubyResult {
  check_max_arguments(arguments, 1)?;
  let argument = PathArgument::new(arguments, 0)?;
  if argument.is_missing() {
    return Err(implicit_conversion_error(arguments.first()));
  }
  let path = argument.path()?;
  match (add_trailing_separator(RULES, Cow::Borrowed(path.bytes())), argument.string()) {
    (Cow::Borrowed(_), Some(string)) => Ok(string.to_any_object()),
    (result, _) => Ok(path.to_ruby(&result).into()),
  }
}

pub fn pn_is_absolute(arguments: &[AnyObject]) -> RubyResult {
  check_max_arguments(arguments, 1)?;
  let argument = PathArgument::new(arguments, 0)?;
  let path = argument.path()?;
  Ok(Boolean::new(!is_relative(RULES, path.bytes())).into())
}

pub fn pn_basename(arguments: &[AnyObject]) -> RubyResult {
  check_max_arguments(arguments, 2)?;
  let argument = PathArgument::new(arguments, 0)?;
  let path = argument.path()?;
  let ext_argument = PathArgument::new(arguments, 1)?;
  let ext = ext_argument.path()?;
  let (component, base_len) = basename::last_component(RULES, path.bytes());
  let result = match base_len {
    // Like Ruby, ignore an extension in an incompatible encoding, and only
    // remove one that starts a character.
    Some(base_len) if EncodingOf::new(&path).merge(&ext).is_ok() => {
      let end = basename::ext_end(RULES, component, base_len, ext.bytes());
      if path.is_char_boundary(component, end) { &component[..end] } else { component }
    }
    _ => component,
  };
  Ok(path.to_ruby(result).into())
}

pub fn pn_children(arguments: &[AnyObject]) -> RubyResult {
  check_max_arguments(arguments, 2)?;
  let argument = PathArgument::new(arguments, 0)?;
  let path = argument.path_or(b".")?;
  let with_directory = truthy_argument(arguments, 1, true) && path.bytes() != b".";
  let error_path = error_path(&argument, ".");
  children(&path, with_directory, &error_path, |entry| Ok(entry.into()))
}

pub fn pn_children_compat(arguments: &[AnyObject]) -> RubyResult {
  check_max_arguments(arguments, 2)?;
  let argument = PathArgument::new(arguments, 0)?;
  let path = argument.path()?;
  let with_directory = truthy_argument(arguments, 1, true) && path.bytes() != b".";
  let error_path = error_path(&argument, "");
  let pathname = pathname_class()?;
  children(&path, with_directory, &error_path, |entry| new_pathname(&pathname, entry))
}

fn children<F>(path: &PathString, with_directory: bool, error_path: &AnyObject, mut wrap: F) -> RubyResult
  where F: FnMut(RString) -> RubyResult {
  let entries = read_dir(path, error_path)?;
  let filesystem = EncodingId::filesystem();
  let mut array = Array::with_capacity(entries.size_hint().0);
  for entry in entries {
    let entry = entry.map_err(|e| system_call_error(&e, error_path, "dir_read"))?;
    let os_name = entry.file_name();
    let (name, name_encoding) = file_name(&os_name, filesystem);
    let string = if with_directory {
      // `File.join(path, name)`, encoded like that, or as the file name if they don't mix.
      let encoding = EncodingOf::new(path).merge_with(EncodingOf::of_bytes(&name, name_encoding)).
        map_or(name_encoding, |encoding| encoding.encoding);
      encoding.new_string(&RULES.join(path.bytes(), &name))
    } else {
      name_encoding.new_string(&name)
    };
    array.push(wrap(string)?);
  }
  Ok(array.into())
}

pub fn pn_chop_basename(arguments: &[AnyObject]) -> RubyResult {
  check_max_arguments(arguments, 1)?;
  let argument = PathArgument::new(arguments, 0)?;
  let path = argument.path()?;
  match chop_basename::chop_basename(RULES, path.bytes()) {
    Some((dirname, basename)) => {
      let mut array = Array::with_capacity(2);
      array.push(path.to_ruby(dirname));
      array.push(path.to_ruby(basename));
      Ok(array.into())
    },
    None => Ok(NilClass::new().into()),
  }
}

pub fn pn_cleanpath_aggressive(arguments: &[AnyObject]) -> RubyResult {
  check_max_arguments(arguments, 1)?;
  let argument = PathArgument::new(arguments, 0)?;
  let path = argument.path()?;
  Ok(path.to_ruby(&cleanpath_aggressive::cleanpath_aggressive(RULES, path.bytes())).into())
}

pub fn pn_cleanpath_conservative(arguments: &[AnyObject]) -> RubyResult {
  check_max_arguments(arguments, 1)?;
  let argument = PathArgument::new(arguments, 0)?;
  let path = argument.path()?;
  Ok(path.to_ruby(&cleanpath_conservative::cleanpath_conservative(RULES, path.bytes())).into())
}

pub fn pn_del_trailing_separator(arguments: &[AnyObject]) -> RubyResult {
  check_max_arguments(arguments, 1)?;
  let argument = PathArgument::new(arguments, 0)?;
  let path = argument.path()?;
  match (del_trailing_separator(RULES, path.bytes()), argument.string()) {
    (Cow::Borrowed(result), Some(string)) if result.len() == path.bytes().len() => Ok(string.to_any_object()),
    (result, _) => Ok(path.to_ruby(&result).into()),
  }
}

pub fn pn_is_directory(arguments: &[AnyObject]) -> RubyResult {
  check_max_arguments(arguments, 1)?;
  let argument = PathArgument::new(arguments, 0)?;
  let path = argument.path()?;
  // Windows takes "dir\\..." for "dir"; Ruby doesn't.
  let is_directory = !RULES.has_dots_name(path.bytes()) &&
    path.os_path().map_or(false, |os_path| os_path.is_dir());
  Ok(Boolean::new(is_directory).into())
}

pub fn pn_dirname(arguments: &[AnyObject]) -> RubyResult {
  check_max_arguments(arguments, 1)?;
  let argument = PathArgument::new(arguments, 0)?;
  let path = argument.path()?;
  Ok(path.to_ruby(&dirname::dirname(RULES, path.bytes())).into())
}

// Returns an array of `String`s
pub fn pn_entries(arguments: &[AnyObject]) -> RubyResult {
  check_max_arguments(arguments, 1)?;
  let argument = PathArgument::new(arguments, 0)?;
  let path = argument.path()?;
  let error_path = error_path(&argument, "");
  entries(&path, &error_path, |entry| Ok(entry.into()))
}

// Returns an array of `Pathname`s
pub fn pn_entries_compat(arguments: &[AnyObject]) -> RubyResult {
  check_max_arguments(arguments, 1)?;
  let argument = PathArgument::new(arguments, 0)?;
  let path = argument.path()?;
  let error_path = error_path(&argument, "");
  let pathname = pathname_class()?;
  entries(&path, &error_path, |entry| new_pathname(&pathname, entry))
}

fn entries<F>(path: &PathString, error_path: &AnyObject, mut wrap: F) -> RubyResult
  where F: FnMut(RString) -> RubyResult {
  let files = read_dir(path, error_path)?;
  let filesystem = EncodingId::filesystem();
  let mut array = Array::with_capacity(files.size_hint().0 + 2);
  array.push(wrap(filesystem.new_string(b"."))?);
  array.push(wrap(filesystem.new_string(b".."))?);
  for file in files {
    let file = file.map_err(|e| system_call_error(&e, error_path, "dir_read"))?;
    let os_name = file.file_name();
    let (name, encoding) = file_name(&os_name, filesystem);
    array.push(wrap(encoding.new_string(&name))?);
  }
  Ok(array.into())
}

pub fn pn_extname(arguments: &[AnyObject]) -> RubyResult {
  check_max_arguments(arguments, 1)?;
  let argument = PathArgument::new(arguments, 0)?;
  let path = argument.path()?;
  Ok(path.to_ruby(extname::extname(RULES, path.bytes())).into())
}

pub fn pn_has_trailing_separator(arguments: &[AnyObject]) -> RubyResult {
  check_max_arguments(arguments, 1)?;
  let argument = PathArgument::new(arguments, 0)?;
  let path = argument.path()?;
  Ok(Boolean::new(has_trailing_separator(RULES, path.bytes())).into())
}

pub fn pn_join(arguments: &[AnyObject]) -> RubyResult {
  if arguments.is_empty() {
    return Err(argument_error("wrong number of arguments (given 0, expected 1+)"));
  }
  let pathname = pathname_class()?;

  // Copy every path out of Ruby before calling more Ruby code: the strings
  // returned by `to_path` or `to_s` aren't referenced from anywhere the
  // garbage collector looks once we only hold their bytes.
  let mut paths: Vec<Vec<u8>> = Vec::with_capacity(arguments.len());
  let mut encoding: Option<EncodingOf> = None;
  for argument in arguments {
    let string = path_like_to_string(argument, &pathname)?;
    let path = PathString::new(&string)?;
    encoding = Some(match encoding {
      None => EncodingOf::new(&path),
      Some(previous) => previous.merge(&path)?,
    });
    paths.push(path.bytes().to_vec());
  }

  let mut parts = paths.iter().rev();
  let mut result: Cow<[u8]> = match parts.next() {
    Some(last) => Cow::Borrowed(last),
    None => Cow::Borrowed(b""),
  };
  for part in parts {
    if !is_relative(RULES, &result) {
      break;
    }
    result = Cow::Owned(plus::plus_paths(RULES, part, &result).into_owned());
  }
  let encoding = match encoding {
    Some(encoding) => encoding.encoding,
    None => EncodingId::us_ascii(),
  };
  new_pathname(&pathname, encoding.new_string(&result))
}

pub fn pn_plus(arguments: &[AnyObject]) -> RubyResult {
  check_max_arguments(arguments, 2)?;
  let argument1 = PathArgument::new(arguments, 0)?;
  let path1 = argument1.path()?;
  let argument2 = PathArgument::new(arguments, 1)?;
  let path2 = argument2.path()?;
  let encoding = EncodingOf::new(&path1).merge(&path2)?.encoding;
  Ok(encoding.new_string(&plus::plus_paths(RULES, path1.bytes(), path2.bytes())).into())
}

pub fn pn_is_relative(arguments: &[AnyObject]) -> RubyResult {
  check_max_arguments(arguments, 1)?;
  let argument = PathArgument::new(arguments, 0)?;
  if argument.is_missing() {
    return Ok(Boolean::new(false).into());
  }
  let path = argument.path()?;
  Ok(Boolean::new(is_relative(RULES, path.bytes())).into())
}

pub fn pn_relative_path_from(arguments: &[AnyObject]) -> RubyResult {
  check_max_arguments(arguments, 2)?;
  let pathname = pathname_class()?;
  let argument = PathArgument::new(arguments, 0)?;
  let dest = argument.path()?;
  let base = match arguments.get(1) {
    Some(base_directory) => {
      let base_string = path_like_to_string(base_directory, &pathname)?;
      // Copied right away, see `pn_join`.
      PathString::new(&base_string)?.into_owned()
    }
    None => PathString::empty(),
  };
  let encoding = EncodingOf::new(&dest).merge(&base)?.encoding;
  let inspect = |bytes: &[u8]| crate::ruby::inspect(&encoding.new_string(bytes));

  match relative_path_from::relative_path_from(RULES, dest.bytes(), base.bytes()) {
    Ok(path) => new_pathname(&pathname, encoding.new_string(&path)),
    Err(RelativePathError::DifferentPrefix(dest_prefix, base_directory)) => {
      let message = format!("different prefix: {} and {}", inspect(&dest_prefix)?, inspect(&base_directory)?);
      Err(argument_error(&message))
    },
    Err(RelativePathError::BaseDirectoryHasDotDot(base_directory)) => {
      Err(argument_error(&format!("base_directory has ..: {}", inspect(&base_directory)?)))
    },
  }
}

// The path to name in a `SystemCallError`
fn error_path(argument: &PathArgument, default: &str) -> AnyObject {
  match argument.string() {
    Some(string) => string.to_any_object(),
    None => RString::new_utf8(default).into(),
  }
}

fn read_dir(path: &PathString, error_path: &AnyObject) -> Result<fs::ReadDir, AnyException> {
  let os_path = path.os_path().map_err(|e| system_call_error(&e, error_path, "dir_initialize"))?;
  fs::read_dir(&*os_path).map_err(|e| system_call_error(&e, error_path, "dir_initialize"))
}

// `SystemCallError.new(path, errno, func)` gives the same `Errno::*` error
// and message as Ruby's `Dir` methods, e.g.
// "No such file or directory @ dir_initialize - path".
fn system_call_error(err: &io::Error, path: &AnyObject, func: &str) -> AnyException {
  #[cfg(unix)]
  {
    if let Some(errno) = err.raw_os_error() {
      let arguments = [path.clone(), rutie::Integer::new(errno.into()).into(), RString::new_utf8(func).into()];
      if let Ok(exception) = Class::system_call_error().protect_send("new", &arguments) {
        if let Ok(exception) = exception.try_convert_to::<AnyException>() {
          return exception;
        }
      }
    }
  }
  #[cfg(not(unix))]
  let _ = func;
  let path = crate::ruby::inspect(path).unwrap_or_default();
  AnyException::from_io_error(err, &path)
}

// A file name from the OS as Ruby's `Dir` methods return it, in the
// filesystem encoding: its bytes, and their encoding.
#[cfg(unix)]
fn file_name(name: &OsStr, filesystem: EncodingId) -> (Cow<'_, [u8]>, EncodingId) {
  use std::os::unix::ffi::OsStrExt;
  (Cow::Borrowed(name.as_bytes()), filesystem)
}

#[cfg(not(unix))]
fn file_name(name: &OsStr, filesystem: EncodingId) -> (Cow<'_, [u8]>, EncodingId) {
  let utf8 = match name.to_string_lossy() {
    Cow::Borrowed(name) => Cow::Borrowed(name.as_bytes()),
    Cow::Owned(name) => Cow::Owned(name.into_bytes()),
  };
  if utf8.is_ascii() || filesystem == EncodingId::utf8() {
    return (utf8, filesystem);
  }
  match EncodingId::utf8().transcode(&utf8, filesystem) {
    Some(encoded) => (Cow::Owned(encoded), filesystem),
    None => (utf8, EncodingId::utf8()),
  }
}

fn implicit_conversion_error(argument: Option<&AnyObject>) -> AnyException {
  let name = match argument {
    None => "nil".to_string(),
    Some(argument) if argument.is_nil() => "nil".to_string(),
    Some(argument) if argument.value().is_true() => "true".to_string(),
    Some(argument) if argument.value().is_false() => "false".to_string(),
    Some(argument) => argument.protect_send("class", &[]).ok().
      and_then(|class| crate::ruby::inspect(&class).ok()).unwrap_or_default(),
  };
  error(&Class::type_error(), &format!("no implicit conversion of {} into String", name))
}
