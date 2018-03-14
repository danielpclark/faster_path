use ruru::{RString, Object, Class, AnyObject};
extern crate ruby_sys;
use debug::RubyDebugInfo;
use ruru;

type MaybeString = Result<ruru::RString, ruru::result::Error>;

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
      unwrap_or(RString::new("")).
      to_string())
  }

  if result.respond_to("to_path") {
    return Ok(RString::from(result.send("to_path", None).value()).to_string())
  }

  Ok(RString::from(result.send("to_s", None).value()).to_string())
}

pub fn to_str(maybe_string: &MaybeString) -> &str {
  match maybe_string {
    &Ok(ref ruru_string) => ruru_string.to_str(),
    &Err(_) => "",
  }
}
