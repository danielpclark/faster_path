use rutie::{
  RString,
  AnyObject,
  Object,
};

#[derive(Debug)]
pub struct RubyDebugInfo {
  object_id: String,
  class: String,
  inspect: String,
}

impl From<AnyObject> for RubyDebugInfo {
  fn from(ao: AnyObject) -> Self {
    let object_id = ao.send("object_id", &[]).send("to_s", &[]).
      try_convert_to::<RString>().unwrap_or(RString::new_utf8("Failed to get object_id!")).to_string();
    let class = ao.send("class", &[]).send("to_s", &[]).
      try_convert_to::<RString>().unwrap_or(RString::new_utf8("Failed to get class!")).to_string();
    let inspect = ao.send("inspect", &[]).
      try_convert_to::<RString>().unwrap_or(RString::new_utf8("Failed to get inspect!")).to_string();

    RubyDebugInfo {
      object_id: object_id,
      class: class,
      inspect: inspect,
    }
  }
}
