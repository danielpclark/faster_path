#[repr(C)]
pub struct RubyArray {
  len: libc::size_t,
  data: *const libc::c_void,
}

impl RubyArray {
  #[allow(dead_code)]
  fn from_vec<T>(vec: Vec<T>) -> RubyArray {
    let array = RubyArray { 
      data: vec.as_ptr() as *const libc::c_void, 
        len: vec.len() as libc::size_t 
    };
    mem::forget(vec);
    array
  }
}
