use ruru::{RString, Object, Class, AnyObject, Fixnum};

#[inline]
pub fn new_pathname_instance(path: &str) -> Class {
  let mut instance = Class::from_existing("Pathname").allocate();
  instance.instance_variable_set("@path", RString::new(path).to_any_object());

  instance
}

pub fn anyobject_to_string(item: AnyObject) -> String {
  if let Ok(result) = item.try_convert_to::<Class>() {
    let result = &result;
    if Class::from_existing("String").case_equals(result) {
      RString::from(result.value()).to_string()
    } else if Class::from_existing("Pathname").case_equals(result) {
      result.instance_variable_get("@path").
        try_convert_to::<RString>().
        unwrap_or(RString::new("")).
        to_string()
    } else if result.respond_to ("to_path") {
      Class::from(result.send("to_path", None).value()).
        instance_variable_get("@path").
        try_convert_to::<RString>().
        unwrap_or(RString::new("")).
        to_string()
    } else {
      RString::from(result.send("to_s", None).value()).to_string()
    }
  } else {
    "".to_string()
  }
}

pub fn into_pathname(itself: AnyObject) -> Result<AnyObject, RubyDebugInfo> {
  if Class::from_existing("String").case_equals(&itself) {
    Ok(new_pathname_instance(
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
  object_id: i64,
  class: String,
  inspect: String,
}

impl From<AnyObject> for RubyDebugInfo {
  fn from(ao: AnyObject) -> Self {
    let object_id = ao.send("object_id", None).
      try_convert_to::<Fixnum>().unwrap_or(Fixnum::new(0)).to_i64();
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
