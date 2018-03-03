use ruru::{AnyObject, Array, Object, AnyException};
use ruru::types::{Argc, Value, CallbackPtr};
use ruru::util::str_to_cstring;
extern crate ruby_sys;
use self::ruby_sys::{class, util, vm};
use ::pathname;

pub fn raise(exception: AnyException) {
  unsafe { vm::rb_exc_raise(exception.value()); }
}

pub type ValueCallback<I, O> = extern "C" fn(Argc, *const Value, I) -> O;

pub fn define_singleton_method<I: Object, O: Object>(
  klass: Value,
  name: &str,
  callback: ValueCallback<I, O>,
) {
  let name = str_to_cstring(name);

  unsafe {
    class::rb_define_singleton_method(klass, name.as_ptr(), callback as CallbackPtr, -1);
  }
}

// ruru doesn't support splat arguments yet
pub extern fn pub_join(argc: Argc, argv: *const Value, _: AnyObject) -> AnyObject {
  let args = Value::from(0);

  unsafe {
    util::rb_scan_args(
      argc,
      argv,
      str_to_cstring("*").as_ptr(),
      &args
    )
  };
  
  pathname::pn_join(Ok(Array::from(args)))
}
