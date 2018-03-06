use ruru::{RString, Object, Class, AnyObject};
extern crate ruby_sys;
use debug::RubyDebugInfo;
use pathname::Pathname;

pub trait TryFrom<T>: Sized {
  type Error;

  fn try_from(value: T) -> Result<Self, Self::Error>;
}

#[inline]
pub fn is_same_path(a: &str, b: &str) -> bool {
  if cfg!(windows) {
    a.to_uppercase() == b.to_uppercase()
  } else {
    a == b
  }
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
    return Ok(Pathname::from(result.send("to_path", None).value()).
      instance_variable_get("@path").
      try_convert_to::<RString>().
      unwrap_or(RString::new("")).
      to_string())
  }

  Ok(RString::from(result.send("to_s", None).value()).to_string())
}
