use ruru::{RString, Object, Class, AnyObject};
use pathname::Pathname;

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
    return Ok(Class::from(result.send("to_path", None).value()).
      instance_variable_get("@path").
      try_convert_to::<RString>().
      unwrap_or(RString::new("")).
      to_string())
  }

  Ok(RString::from(result.send("to_s", None).value()).to_string())
}

#[allow(dead_code)]
pub fn into_pathname(itself: AnyObject) -> Result<AnyObject, RubyDebugInfo> {
  if Class::from_existing("String").case_equals(&itself) {
    Ok(Pathname::new(
        &RString::from(itself.value()).to_string()
    ).to_any_object())
  } else if Class::from_existing("Pathname").case_equals(&itself) {
    Ok(itself)
  } else {
    Err(RubyDebugInfo::from(itself))
  }
}

#[derive(Debug)]
pub struct RubyDebugInfo {
  object_id: String,
  class: String,
  inspect: String,
}

impl From<AnyObject> for RubyDebugInfo {
  fn from(ao: AnyObject) -> Self {
    let object_id = ao.send("object_id", None).send("to_s", None).
      try_convert_to::<RString>().unwrap_or(RString::new("Failed to get object_id!")).to_string();
    let class = ao.send("class", None).send("to_s", None).
      try_convert_to::<RString>().unwrap_or(RString::new("Failed to get class!")).to_string();
    let inspect = ao.send("inspect", None).
      try_convert_to::<RString>().unwrap_or(RString::new("Failed to get inspect!")).to_string();

    RubyDebugInfo {
      object_id: object_id,
      class: class,
      inspect: inspect,
    }
  }
}
