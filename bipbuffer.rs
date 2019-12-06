#![allow(dead_code,
         mutable_transmutes,
         non_camel_case_types,
         non_snake_case,
         non_upper_case_globals,
         unused_assignments,
         unused_mut)]
extern crate libc;
#[no_mangle]
pub unsafe extern "C" fn bipbuf_unused(mut me: *const libc::c_int)
 -> libc::c_int {
    panic!("Reached end of non-void function without returning");
    /* distance between region B and region A */
}
#[no_mangle]
pub unsafe extern "C" fn bipbuf_size(mut me: *const libc::c_int)
 -> libc::c_int {
    panic!("Reached end of non-void function without returning");
}
#[no_mangle]
pub unsafe extern "C" fn bipbuf_used(mut me: *const libc::c_int)
 -> libc::c_int {
    panic!("Reached end of non-void function without returning");
}
#[no_mangle]
pub unsafe extern "C" fn bipbuf_init(mut me: *mut libc::c_int,
                                     size: libc::c_uint) {
}
#[no_mangle]
pub unsafe extern "C" fn bipbuf_free(mut me: *mut libc::c_int) { }
#[no_mangle]
pub unsafe extern "C" fn bipbuf_is_empty(mut me: *const libc::c_int)
 -> libc::c_int {
    panic!("Reached end of non-void function without returning");
}
