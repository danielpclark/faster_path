// Glue between Ruby and the path functions, written so that nothing Ruby
// passes in can crash the interpreter:
//
// * Panics never unwind into Ruby; they are caught and raised as `RuntimeError`.
// * Ruby exceptions are only raised once every Rust value owning memory has
//   been dropped, since raising `longjmp`s past Rust frames without running
//   their destructors.
// * Ruby code (`to_path`, `to_s`, ...) is only called through `protect_send`,
//   so an exception it raises comes back as an `Err` instead of jumping over
//   Rust frames.
// * Strings are read as bytes in their own encoding, never assumed to be UTF-8.
use std::any::Any;
use std::borrow::Cow;
use std::io;
use std::path::Path;
use std::sync::atomic::{AtomicI32, AtomicU8, Ordering};
use std::thread;

use rutie::rubysys;
use rutie::types::{c_char, c_long, Argc, EncodingIndex, Value, ValueType};
use rutie::{AnyException, AnyObject, Class, Encoding, NilClass, Object, RString, Symbol, VM};

use crate::path_parsing::Rules;

pub type RubyResult = Result<AnyObject, AnyException>;

// Defines Ruby methods taking any number of arguments (arity -1). The body
// sees the arguments as `&[AnyObject]` and returns a `RubyResult`; an `Err`
// is raised in Ruby.
macro_rules! ruby_methods {
  ($($(#[$attribute:meta])* fn $name:ident($arguments:ident) $body:block)*) => {$(
    rutie::rutie_callback! {
      $(#[$attribute])*
      pub fn $name(argc: rutie::types::Argc, argv: *const rutie::AnyObject, _itself: rutie::AnyObject) -> rutie::AnyObject {
        // Safety: Ruby passes `argc` arguments in `argv`, alive for the whole call.
        let $arguments = unsafe { $crate::ruby::arguments(argc, argv) };
        $crate::ruby::finish(std::panic::catch_unwind(std::panic::AssertUnwindSafe(
          || -> $crate::ruby::RubyResult { $body }
        )))
      }
    }
  )*};
}

/// Borrows the arguments of a method call.
///
/// # Safety
///
/// `argv` must point to `argc` Ruby values that stay alive for `'a`, as the
/// arguments of a method call do while the method runs.
pub unsafe fn arguments<'a>(argc: Argc, argv: *const AnyObject) -> &'a [AnyObject] {
  if argc <= 0 || argv.is_null() {
    &[]
  } else {
    // Safety: guaranteed by the caller.
    unsafe { std::slice::from_raw_parts(argv, argc as usize) }
  }
}

/// Returns the method's result to Ruby, or raises its error.
///
/// Raising does not return, so it happens here, after the method body and
/// everything it allocated have been dropped.
pub fn finish(result: thread::Result<RubyResult>) -> AnyObject {
  let exception = match result {
    Ok(Ok(value)) => return value,
    Ok(Err(exception)) => exception,
    Err(payload) => panic_exception(payload),
  };
  VM::raise_ex(exception);
  NilClass::new().into()
}

fn panic_exception(payload: Box<dyn Any + Send>) -> AnyException {
  let message = if let Some(message) = payload.downcast_ref::<&str>() {
    message.to_string()
  } else if let Some(message) = payload.downcast_ref::<String>() {
    message.clone()
  } else {
    "unknown error".to_string()
  };
  error(&Class::runtime_error(), &format!("faster_path internal error: {}", message))
}

pub fn error(class: &Class, message: &str) -> AnyException {
  AnyException::from_class(class, message)
}

pub fn argument_error(message: &str) -> AnyException {
  error(&Class::argument_error(), message)
}

/// Raises `ArgumentError` unless there are at most `max` arguments.
pub fn check_max_arguments(arguments: &[AnyObject], max: usize) -> Result<(), AnyException> {
  if arguments.len() > max {
    let expected = if max == 0 { "0".to_string() } else { format!("0..{}", max) };
    Err(argument_error(&format!(
      "wrong number of arguments (given {}, expected {})",
      arguments.len(),
      expected
    )))
  } else {
    Ok(())
  }
}

/// Truthiness of an optional argument.
pub fn truthy_argument(arguments: &[AnyObject], index: usize, default: bool) -> bool {
  match arguments.get(index) {
    Some(argument) => !(argument.is_nil() || argument.value().is_false()),
    None => default,
  }
}

/// A path argument.
pub enum PathArgument {
  /// Missing, `nil`, or neither a `String` nor something with `to_path`.
  /// The methods have always treated these as an empty path.
  Missing,
  /// A `String` argument.
  String(RString),
  /// The `String` an argument's `to_path` returned.
  Converted(RString),
}

impl PathArgument {
  /// Reads the argument at `index` like Ruby's `File` methods do: a `String`,
  /// or an object with `to_path`.
  pub fn new(arguments: &[AnyObject], index: usize) -> Result<Self, AnyException> {
    let argument = match arguments.get(index) {
      Some(argument) => argument,
      None => return Ok(PathArgument::Missing),
    };
    if let Ok(string) = argument.try_convert_to::<RString>() {
      return Ok(PathArgument::String(string));
    }
    if argument.is_nil() {
      return Ok(PathArgument::Missing);
    }
    let responds_to_path = argument.protect_send("respond_to?", &[Symbol::new("to_path").into()])?;
    if !responds_to_path.value().is_true() {
      return Ok(PathArgument::Missing);
    }
    let path = argument.protect_send("to_path", &[])?;
    Ok(PathArgument::Converted(expect_string(path, argument, "to_path")?))
  }

  pub fn is_missing(&self) -> bool {
    matches!(self, PathArgument::Missing)
  }

  /// The path, or `default` if the argument is missing.
  pub fn path_or(&self, default: &'static [u8]) -> Result<PathString<'_>, AnyException> {
    match self {
      PathArgument::Missing => Ok(PathString::literal(default)),
      PathArgument::String(string) => PathString::new(string),
      // Nothing but this value refers to the string `to_path` returned, and
      // the garbage collector may not see it, so keep a copy.
      PathArgument::Converted(string) => PathString::new(string).map(PathString::into_owned),
    }
  }

  pub fn path(&self) -> Result<PathString<'_>, AnyException> {
    self.path_or(b"")
  }

  /// The `String`, for error messages or to return unchanged.
  pub fn string(&self) -> Option<&RString> {
    match self {
      PathArgument::Missing => None,
      PathArgument::String(string) | PathArgument::Converted(string) => Some(string),
    }
  }
}

/// A path read from a Ruby `String`: its bytes and its encoding.
pub struct PathString<'a> {
  original: Cow<'a, [u8]>,
  // On Windows, a copy in which each `\` that is part of a multibyte
  // character is a NUL instead, so it isn't taken for a separator; see
  // `EncodingId::mask_backslashes`.
  masked: Option<Vec<u8>>,
  pub encoding: EncodingId,
}

impl<'a> PathString<'a> {
  /// An empty path, used where a `String` argument is missing.
  pub fn empty() -> Self {
    PathString::literal(b"")
  }

  pub fn literal(bytes: &'static [u8]) -> Self {
    PathString { original: Cow::Borrowed(bytes), masked: None, encoding: EncodingId::us_ascii() }
  }

  /// Reads `string`, which must be kept alive (and unmodified) while the
  /// result is in use; a method argument is.
  ///
  /// Like Ruby's own path methods this rejects strings that are not
  /// ASCII-compatible (such as UTF-16), since those can't be split on `/`,
  /// and strings containing a null byte.
  pub fn new(string: &'a RString) -> Result<Self, AnyException> {
    let encoding = EncodingId::of(string);
    if !encoding.is_ascii_compatible() {
      return Err(error(
        &Class::encoding_compatibility_error(),
        &format!("path name must be ASCII-compatible ({}): {}", encoding.name(), inspect(string)?),
      ));
    }
    // This returns the bytes of a `String`; `RString` is always one.
    let bytes = string.to_bytes_unchecked();
    if bytes.contains(&0) {
      return Err(argument_error("path name contains null byte"));
    }
    let masked = if Rules::NATIVE.is_dosish() { encoding.mask_backslashes(bytes) } else { None };
    Ok(PathString { original: Cow::Borrowed(bytes), masked, encoding })
  }

  pub fn into_owned(self) -> PathString<'static> {
    PathString { original: Cow::Owned(self.original.into_owned()), masked: self.masked, encoding: self.encoding }
  }

  /// The bytes the path functions work on.
  pub fn bytes(&self) -> &[u8] {
    self.masked.as_deref().unwrap_or(&self.original)
  }

  /// A new Ruby string with `bytes` in this path's encoding.
  pub fn to_ruby(&self, bytes: &[u8]) -> RString {
    self.encoding.new_string(bytes)
  }

  /// Whether `pos` starts a character of `bytes`, which must start at a
  /// character of this path's encoding.
  pub fn is_char_boundary(&self, bytes: &[u8], pos: usize) -> bool {
    if pos == 0 || pos >= bytes.len() {
      return true;
    }
    if self.encoding == EncodingId::utf8() {
      // Not a continuation byte
      return (bytes[pos] & 0xC0) != 0x80;
    }
    if self.encoding.is_single_byte_or_utf8() {
      return true;
    }
    // Other encodings: walk the characters from the start like Ruby does.
    let bytes = unmask(bytes);
    let mut index = 0;
    while index < pos {
      index += self.encoding.char_len(&bytes, index);
    }
    index == pos
  }

  /// The path to give the OS.
  #[cfg(unix)]
  pub fn os_path(&self) -> io::Result<Cow<'_, Path>> {
    use std::os::unix::ffi::OsStrExt;
    Ok(Cow::Borrowed(Path::new(std::ffi::OsStr::from_bytes(&self.original))))
  }

  /// The path to give the OS, which takes Unicode on Windows.
  #[cfg(not(unix))]
  pub fn os_path(&self) -> io::Result<Cow<'_, Path>> {
    let invalid = || io::Error::new(io::ErrorKind::InvalidInput, "path name can't be converted to UTF-8");
    if self.encoding == EncodingId::utf8() || self.original.is_ascii() {
      return std::str::from_utf8(&self.original).map(|path| Cow::Borrowed(Path::new(path))).map_err(|_| invalid());
    }
    let utf8 = self.encoding.transcode(&self.original, EncodingId::utf8()).ok_or_else(invalid)?;
    String::from_utf8(utf8).map(|path| Cow::Owned(path.into())).map_err(|_| invalid())
  }
}

// Puts back the `\`s `EncodingId::mask_backslashes` replaced. NUL is never
// in a path otherwise.
fn unmask(bytes: &[u8]) -> Cow<'_, [u8]> {
  if Rules::NATIVE.is_dosish() && memchr::memchr(0, bytes).is_some() {
    Cow::Owned(bytes.iter().map(|&c| if c == 0 { b'\\' } else { c }).collect())
  } else {
    Cow::Borrowed(bytes)
  }
}

/// An encoding, by its index in Ruby's table of encodings.
///
/// `Encoding` objects compare with `Encoding#==` and answer
/// `Encoding#ascii_compatible?` through Ruby method calls, which cost more
/// than most path operations; this compares indexes, and remembers whether
/// an encoding is ASCII-compatible (which never changes).
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct EncodingId(EncodingIndex);

static UTF8_INDEX: AtomicI32 = AtomicI32::new(-1);
static US_ASCII_INDEX: AtomicI32 = AtomicI32::new(-1);
static ASCII_8BIT_INDEX: AtomicI32 = AtomicI32::new(-1);

const UNKNOWN: u8 = 0;
const ASCII_COMPATIBLE: u8 = 1;
const NOT_ASCII_COMPATIBLE: u8 = 2;
#[allow(clippy::declare_interior_mutable_const)]
const UNKNOWN_SLOT: AtomicU8 = AtomicU8::new(UNKNOWN);
static ASCII_COMPATIBILITY: [AtomicU8; 128] = [UNKNOWN_SLOT; 128];

impl EncodingId {
  /// The encoding of a `String`.
  pub fn of(string: &RString) -> Self {
    // Safety: `string` is a `String`.
    EncodingId(unsafe { rubysys::encoding::rb_enc_get_index(string.value()) })
  }

  pub fn from_encoding(encoding: &Encoding) -> Self {
    EncodingId(encoding.index())
  }

  pub fn filesystem() -> Self {
    EncodingId::from_encoding(&Encoding::filesystem())
  }

  pub fn utf8() -> Self {
    EncodingId::known(&UTF8_INDEX, Encoding::utf8)
  }

  pub fn us_ascii() -> Self {
    EncodingId::known(&US_ASCII_INDEX, Encoding::us_ascii)
  }

  pub fn ascii_8bit() -> Self {
    EncodingId::known(&ASCII_8BIT_INDEX, Encoding::ascii_8bit)
  }

  fn known(cache: &AtomicI32, encoding: fn() -> Encoding) -> Self {
    let index = cache.load(Ordering::Relaxed);
    if index >= 0 {
      return EncodingId(index);
    }
    let index = encoding().index();
    cache.store(index, Ordering::Relaxed);
    EncodingId(index)
  }

  /// The `Encoding` object.
  pub fn to_encoding(self) -> Encoding {
    // Safety: the index is valid (see `is_char_boundary`).
    Encoding::from(unsafe {
      rubysys::encoding::rb_enc_from_encoding(rubysys::encoding::rb_enc_from_index(self.0))
    })
  }

  pub fn name(self) -> String {
    self.to_encoding().name()
  }

  pub fn is_ascii_compatible(self) -> bool {
    let slot = usize::try_from(self.0).ok().and_then(|index| ASCII_COMPATIBILITY.get(index));
    match slot.map(|slot| slot.load(Ordering::Relaxed)) {
      Some(ASCII_COMPATIBLE) => true,
      Some(NOT_ASCII_COMPATIBLE) => false,
      _ => {
        let compatible = self.to_encoding().is_ascii_compatible();
        if let Some(slot) = slot {
          slot.store(if compatible { ASCII_COMPATIBLE } else { NOT_ASCII_COMPATIBLE }, Ordering::Relaxed);
        }
        compatible
      }
    }
  }

  // UTF-8, or an encoding of single bytes where every character starts a
  // character.
  fn is_single_byte_or_utf8(self) -> bool {
    self == EncodingId::utf8() || self == EncodingId::us_ascii() || self == EncodingId::ascii_8bit()
  }

  // The length of the character at `start` (`rb_enc_mbclen`), at least 1.
  fn char_len(self, bytes: &[u8], start: usize) -> usize {
    // Safety: the index is valid (see `is_char_boundary`), and both
    // pointers are within `bytes`, `end` one past its last byte.
    let length = unsafe {
      rubysys::encoding::rb_enc_mbclen(
        bytes[start..].as_ptr() as *const c_char,
        bytes.as_ptr_range().end as *const c_char,
        rubysys::encoding::rb_enc_from_index(self.0),
      )
    };
    (length.max(1) as usize).min(bytes.len() - start)
  }

  /// In encodings such as Shift_JIS, `\` (0x5C) can be the second byte of a
  /// character. Windows Ruby only takes a `\` that starts a character for a
  /// separator; this returns a copy of `bytes` with the others replaced by
  /// NUL (which a path never contains), or `None` if there are none.
  pub fn mask_backslashes(self, bytes: &[u8]) -> Option<Vec<u8>> {
    if self.is_single_byte_or_utf8() || !bytes.contains(&b'\\') {
      return None;
    }
    let mut masked: Option<Vec<u8>> = None;
    let mut index = 0;
    while index < bytes.len() {
      let length = self.char_len(bytes, index);
      for trailing in index + 1..index + length {
        if bytes[trailing] == b'\\' {
          masked.get_or_insert_with(|| bytes.to_vec())[trailing] = 0;
        }
      }
      index += length;
    }
    masked
  }

  /// `bytes`, in this encoding, converted to the encoding `to`
  /// (`String#encode`), or `None` if they can't be.
  #[cfg_attr(unix, allow(dead_code))]
  pub fn transcode(self, bytes: &[u8], to: EncodingId) -> Option<Vec<u8>> {
    let string = self.new_string(bytes);
    let encoded = string.protect_send("encode", &[to.to_encoding().into()]).ok()?;
    // Copied before anything else can run the garbage collector
    let encoded = encoded.try_convert_to::<RString>().ok()?;
    Some(encoded.to_bytes_unchecked().to_vec())
  }

  /// A new `String` of `bytes` in this encoding.
  pub fn new_string(self, bytes: &[u8]) -> RString {
    let bytes = unmask(bytes);
    // Safety: `bytes` is valid for its length, which Ruby copies, and the
    // index is valid (see `is_char_boundary`).
    RString::from(unsafe {
      rubysys::string::rb_enc_str_new(
        bytes.as_ptr() as *const c_char,
        bytes.len() as c_long,
        rubysys::encoding::rb_enc_from_index(self.0),
      )
    })
  }
}

/// The encoding of a string built from parts of other strings, and whether
/// all of those were ASCII only.
pub struct EncodingOf {
  pub encoding: EncodingId,
  ascii_only: bool,
}

impl EncodingOf {
  pub fn new(path: &PathString) -> Self {
    EncodingOf::of_bytes(path.bytes(), path.encoding)
  }

  pub fn of_bytes(bytes: &[u8], encoding: EncodingId) -> Self {
    EncodingOf { encoding, ascii_only: bytes.is_ascii() }
  }

  pub fn merge(self, path: &PathString) -> Result<Self, AnyException> {
    self.merge_with(EncodingOf::new(path))
  }

  /// Ruby's rules for the encoding of two concatenated strings.
  pub fn merge_with(self, other: EncodingOf) -> Result<Self, AnyException> {
    if self.encoding == other.encoding || other.ascii_only {
      Ok(EncodingOf { ascii_only: self.ascii_only && other.ascii_only, ..self })
    } else if self.ascii_only {
      Ok(other)
    } else {
      Err(error(
        &Class::encoding_compatibility_error(),
        &format!("incompatible character encodings: {} and {}", self.encoding.name(), other.encoding.name()),
      ))
    }
  }
}

/// `object.inspect`
pub fn inspect<T: Object>(object: &T) -> Result<String, AnyException> {
  let inspected = object.protect_send("inspect", &[])?;
  Ok(match inspected.try_convert_to::<RString>() {
    Ok(string) => String::from_utf8_lossy(string.to_bytes_unchecked()).into_owned(),
    Err(_) => String::new(),
  })
}

/// Converts a path-like object to a `String` the way `FasterPath.join` and
/// `FasterPath.relative_path_from` always have: `String`s as they are,
/// `Pathname`s by their path, then `to_path`, then `to_s`.
pub fn path_like_to_string(object: &AnyObject, pathname_class: &Class) -> Result<RString, AnyException> {
  if object.ty() == ValueType::RString {
    return object.try_convert_to::<RString>();
  }
  if object.is_kind_of(pathname_class) {
    let path = object.protect_send("to_s", &[])?;
    return expect_string(path, object, "to_s");
  }
  let responds_to_path = object.protect_send("respond_to?", &[Symbol::new("to_path").into()])?;
  let method = if responds_to_path.value().is_true() { "to_path" } else { "to_s" };
  let path = object.protect_send(method, &[])?;
  expect_string(path, object, method)
}

fn expect_string(result: AnyObject, object: &AnyObject, method: &str) -> Result<RString, AnyException> {
  result.try_convert_to::<RString>().map_err(|_| {
    let class_name = object.protect_send("class", &[]).ok().
      and_then(|class| inspect(&class).ok()).unwrap_or_default();
    error(
      &Class::type_error(),
      &format!("can't convert {} to String ({}#{} gives {})", class_name, class_name, method, result_class(&result)),
    )
  })
}

fn result_class(result: &AnyObject) -> String {
  result.protect_send("class", &[]).ok().and_then(|class| inspect(&class).ok()).unwrap_or_default()
}

pub fn pathname_class() -> Result<Class, AnyException> {
  Class::from_path("Pathname")
}

extern "C" {
  fn rb_obj_alloc(klass: Value) -> Value;
}

/// A `Pathname` of `path`, a new `String` without null bytes.
///
/// This sets `@path` like `Pathname#initialize` does instead of calling
/// `Pathname.new`, which is much faster and runs no Ruby code.
pub fn new_pathname(pathname_class: &Class, path: RString) -> RubyResult {
  // Safety: `pathname_class` is a class (`Class::from_path` checks), and
  // `Pathname` uses the default allocator.
  let mut pathname = AnyObject::from(unsafe { rb_obj_alloc(pathname_class.value()) });
  pathname.instance_variable_set("@path", path);
  Ok(pathname)
}
