#[no_mangle]
pub extern fn rust_arch_bits() -> i32 {
  use std::mem::size_of;
  let s = size_of::<usize>() * 8;
  s as i32
}

#[test]
fn it_is_32_or_64(){
  assert!(rust_arch_bits() == 32 || rust_arch_bits() == 64);
}
