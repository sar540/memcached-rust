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
    fn memcpy(_: *mut libc::c_void, _: *const libc::c_void, _: libc::c_ulong)
     -> *mut libc::c_void;
}
#[inline]
unsafe extern "C" fn out1(in_0: libc::c_char, mut p: *mut libc::c_char)
 -> *mut libc::c_char {
    memcpy(p as *mut libc::c_void,
           &in_0 as *const libc::c_char as *const libc::c_void,
           1i32 as libc::c_ulong);
    return p.offset(1);
}
#[no_mangle]
pub unsafe extern "C" fn itoa_u32(mut u: libc::c_int,
                                  mut p: *mut libc::c_char)
 -> *mut libc::c_char {
    let mut d: libc::c_int = 0i32;
    let mut n: libc::c_int = 0;
    panic!("Reached end of non-void function without returning");
}
#[no_mangle]
pub unsafe extern "C" fn itoa_u64(mut u: libc::c_int,
                                  mut p: *mut libc::c_char)
 -> *mut libc::c_char {
    let mut d: libc::c_int = 0;
    p = out1(('0' as i32 + d) as libc::c_char, p);
    panic!("Reached end of non-void function without returning");
}
