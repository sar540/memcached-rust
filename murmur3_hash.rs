#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case,
         non_upper_case_globals, unused_assignments, unused_mut)]
//-----------------------------------------------------------------------------
/* Definition modified slightly from the public domain interface (no seed +
 * return value */
#[no_mangle]
pub unsafe extern "C" fn MurmurHash3_x86_32(mut key: *const libc::c_void,
                                            mut length: libc::c_int)
 -> libc::c_int {
    let mut data: *const libc::c_int = 0 as *const libc::c_int;
    let nblocks: libc::c_int = 0;
    //----------
  // body
    let mut blocks: *const libc::c_int = 0 as *const libc::c_int;
    let mut i: libc::c_int = -nblocks;
    while i != 0 { i += 1 }
    //----------
  // tail
    let mut tail: *const libc::c_int = 0 as *const libc::c_int;
    panic!("Reached end of non-void function without returning");
    //----------
  // finalization
    //*(uint32_t*)out = h1;
}
