#![allow(dead_code,
         mutable_transmutes,
         non_camel_case_types,
         non_snake_case,
         non_upper_case_globals,
         unused_assignments,
         unused_mut)]
extern crate libc;
extern "C" {
    #[no_mangle]
    fn calloc(_: libc::c_ulong, _: libc::c_ulong) -> *mut libc::c_void;
    #[no_mangle]
    fn free(__ptr: *mut libc::c_void);
    #[no_mangle]
    fn strdup(_: *const libc::c_char) -> *mut libc::c_char;
}
pub type size_t = libc::c_ulong;
/* -*- Mode: C; tab-width: 4; c-basic-offset: 4; indent-tabs-mode: nil -*- */
#[no_mangle]
pub static mut initial_pool_size: libc::c_int = 64i32;
#[inline]
unsafe extern "C" fn get_object(mut ptr: *mut libc::c_void)
 -> *mut libc::c_void {
    return ptr;
}
#[no_mangle]
pub unsafe extern "C" fn cache_destroy(mut cache: *mut libc::c_int) { }
#[no_mangle]
pub unsafe extern "C" fn cache_alloc(mut cache: *mut libc::c_int)
 -> *mut libc::c_void {
    let mut ret: *mut libc::c_void = 0 as *mut libc::c_void;
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn cache_free(mut cache: *mut libc::c_int,
                                    mut ptr: *mut libc::c_void) {
}
