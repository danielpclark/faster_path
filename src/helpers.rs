use ruru::{RString, Object, Class};

pub fn new_pathname_instance(path: &str) -> Class {
  let mut path_instance = Class::from(
    Class::from_existing("Pathname").send("allocate", None).value()
  );
  path_instance.instance_variable_set(
    "@path",
    RString::new(path).to_any_object()
  );

  path_instance
}
