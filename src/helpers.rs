extern crate ruby_sys;
use self::ruby_sys::string;

use ruru::{AnyObject, Class};
use ruru::types::{c_char, c_long, Value};

#[inline]
pub fn str_to_value(string: &str) -> Value {
  let str_ptr = string.as_ptr() as *const c_char;
  let len = string.len() as c_long;

  unsafe { string::rb_str_new(str_ptr, len) }
}

pub fn str_to_any_obj(str_var: &str) -> AnyObject {
  AnyObject::from(str_to_value(str_var))
}

pub fn class_new(klass: &str, params: Vec<AnyObject>) -> AnyObject {
  Class::from_existing(klass).new_instance(params)
}
