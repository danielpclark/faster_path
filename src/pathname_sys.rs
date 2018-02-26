use std::mem::size_of_val;
use ruru::{AnyObject, Array, Object};
use ruru::types::{Argc, Value, c_long, CallbackPtr};
use ruru::util::str_to_cstring;
extern crate ruby_sys;
use self::ruby_sys::{class, util};
use ::pathname;

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

fn number_of(array: &[Value; 2]) -> usize {
  size_of_val(array) / size_of_val(&array[0])
}

// ruru doesn't support splat arguments yet
pub extern fn pub_join(argc: Argc, argv: *const Value, _: AnyObject) -> AnyObject {
  let args: [Value; 2] = [
    Value::from(0),
    Value::from(0)
  ];

  unsafe {
    util::rb_scan_args(
      argc,
      argv,
      str_to_cstring("1*").as_ptr(),
      &args[0],
      &args[1]
    );
  }
  

  let args_array: Value = unsafe {
    util::rb_ary_new_from_values(
      number_of(&args) as c_long,
      args.as_ptr()
    )
  };
  let mut args_array: Array = Array::from(args_array);

  let path_self = args_array.shift();

  pathname::pn_join(Ok(path_self), Ok(args_array))
}
