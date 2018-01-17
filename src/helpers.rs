use ruru::{AnyObject, Class, Object, RString};

pub fn str_to_any_obj(str_var: &str) -> AnyObject {
  RString::new(str_var).to_any_object()
}

pub fn class_new(klass: &str, params: Vec<AnyObject>) -> AnyObject {
  Class::from_existing(klass).new_instance(params)
}
