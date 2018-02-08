use ruru::{AnyObject, Class};

pub fn class_new(klass: &str, params: Option<&[AnyObject]>) -> AnyObject {
  Class::from_existing(klass).new_instance(params)
}
