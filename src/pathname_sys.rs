use rutie::{AnyObject, Array, Object, AnyException};
use rutie::types::{Argc, Value};
use rutie::util::str_to_cstring;
use rutie::rubysys::{util, vm};
use ::pathname;
use std::mem;

pub fn raise(exception: AnyException) {
  unsafe { vm::rb_exc_raise(exception.value()); }
}

// rutie doesn't support splat arguments yet natively
pub extern fn pub_join(argc: Argc, argv: *const AnyObject, _: AnyObject) -> AnyObject {
  let args = Value::from(0);

  unsafe {
    let p_argv: *const Value = mem::transmute(argv);

    util::rb_scan_args(
      argc,
      p_argv,
      str_to_cstring("*").as_ptr(),
      &args
    )
  };
  
  pathname::pn_join(Ok(Array::from(args)))
}
