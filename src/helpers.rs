use rutie::{RString, Object, Class, AnyObject};
use debug::RubyDebugInfo;
use rutie;

type MaybeString = Result<rutie::RString, rutie::AnyException>;

pub trait TryFrom<T>: Sized {
  type Error;

  fn try_from(value: T) -> Result<Self, Self::Error>;
}

#[cfg(windows)]
#[inline]
pub fn is_same_path(a: &str, b: &str) -> bool {
  a.to_uppercase() == b.to_uppercase()
}

#[cfg(not(windows))]
#[inline]
pub fn is_same_path(a: &str, b: &str) -> bool {
  a == b
}

pub fn anyobject_to_string(item: AnyObject) -> Result<String, RubyDebugInfo> {
  let result = &item;
  if Class::from_existing("String").case_equals(result) {
    return Ok(RString::from(result.value()).to_string())
  }

  if Class::from_existing("Pathname").case_equals(result) {
    return Ok(result.instance_variable_get("@path").
      try_convert_to::<RString>().
      unwrap_or(RString::new_utf8("")).
      to_string())
  }

  if result.respond_to("to_path") {
    return Ok(RString::from(result.send("to_path", &[]).value()).to_string())
  }

  Ok(RString::from(result.send("to_s", &[]).value()).to_string())
}

pub fn to_str(maybe_string: &MaybeString) -> &str {
  match maybe_string {
    &Ok(ref rutie_string) => rutie_string.to_str_unchecked(),
    &Err(_) => "",
  }
}

pub fn to_bytes(maybe_string: &MaybeString) -> &[u8] {
  match maybe_string {
    &Ok(ref rutie_string) => rutie_string.to_bytes_unchecked(),
    &Err(_) => &[],
  }
}
